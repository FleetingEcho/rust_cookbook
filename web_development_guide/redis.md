# Redis 实战：缓存 / Session / 限流 / 分布式锁

```toml
[dependencies]
deadpool-redis = { version = "0.16", features = ["rt_tokio_1"] }
redis          = { version = "0.26", features = ["tokio-comp", "json"] }
serde          = { version = "1", features = ["derive"] }
serde_json     = "1"
thiserror      = "1"
tokio          = { version = "1", features = ["full"] }
```

---

## 一、连接池

```rust
use deadpool_redis::{Config, Pool, Runtime, redis::AsyncCommands};

pub fn create_redis_pool(url: &str) -> anyhow::Result<Pool> {
    let cfg = Config::from_url(url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;
    Ok(pool)
}

// 在 AppState 中使用
#[derive(Clone)]
pub struct AppState {
    pub db:    sqlx::PgPool,
    pub redis: Pool,
}

// 应用启动时
let redis_pool = create_redis_pool(&config.redis_url)?;

// 健康检查
pub async fn ping_redis(pool: &Pool) -> anyhow::Result<()> {
    let mut conn = pool.get().await?;
    let pong: String = redis::cmd("PING")
        .query_async(&mut conn)
        .await?;
    anyhow::ensure!(pong == "PONG", "Redis PING 失败");
    Ok(())
}
```

---

## 二、基础操作

```rust
use deadpool_redis::{Pool, redis::AsyncCommands};

// 获取连接（每次操作从池中取，用完自动归还）
let mut conn = pool.get().await?;

// ─── String 操作 ───
// SET
conn.set::<_, _, ()>("key", "value").await?;

// SET 并设过期时间（秒）
conn.set_ex::<_, _, ()>("session:abc", "data", 3600_u64).await?;

// SET 只在不存在时（NX）
let ok: bool = conn.set_nx("lock:resource", "1").await?;

// GET
let val: Option<String> = conn.get("key").await?;
let val = val.unwrap_or_default();

// DEL
conn.del::<_, ()>("key").await?;
conn.del::<_, ()>(vec!["key1", "key2"]).await?;

// 检查是否存在
let exists: bool = conn.exists("key").await?;

// 设置过期时间（已存在的 key）
conn.expire::<_, ()>("key", 300_i64).await?;

// 查看剩余 TTL（秒）
let ttl: i64 = conn.ttl("key").await?;
// -1 = 永不过期，-2 = key 不存在

// ─── 原子计数器 ───
let new_val: i64 = conn.incr("page:views", 1_i64).await?;
let new_val: i64 = conn.incr("balance", -10_i64).await?;  // 减法

// ─── Hash 操作（适合存结构化数据）───
conn.hset::<_, _, _, ()>("user:1", "name", "Alice").await?;
conn.hset_multiple::<_, _, _, ()>("user:1", &[
    ("name", "Alice"),
    ("email", "a@b.com"),
]).await?;

let name: Option<String> = conn.hget("user:1", "name").await?;
let all: std::collections::HashMap<String, String> = conn.hgetall("user:1").await?;
conn.hdel::<_, _, ()>("user:1", "name").await?;

// ─── List 操作（队列/栈）───
conn.lpush::<_, _, ()>("queue:emails", "task1").await?;  // 左推
conn.rpush::<_, _, ()>("queue:emails", "task2").await?;  // 右推
let item: Option<String> = conn.lpop("queue:emails", None).await?;   // 左弹
let item: Option<String> = conn.rpop("queue:emails", None).await?;   // 右弹

// ─── Set 操作 ───
conn.sadd::<_, _, ()>("online_users", "user:1").await?;
conn.srem::<_, _, ()>("online_users", "user:1").await?;
let is_member: bool = conn.sismember("online_users", "user:1").await?;
let members: Vec<String> = conn.smembers("online_users").await?;
let count: i64 = conn.scard("online_users").await?;

// ─── Sorted Set（带分数的有序集合）───
conn.zadd::<_, _, _, ()>("leaderboard", 1000.0_f64, "user:1").await?;
conn.zadd::<_, _, _, ()>("leaderboard", 2000.0_f64, "user:2").await?;
// 取前 10 名（分数从高到低）
let top: Vec<(String, f64)> = conn.zrevrange_withscores("leaderboard", 0, 9).await?;
let rank: Option<i64> = conn.zrevrank("leaderboard", "user:1").await?;
```

---

## 三、JSON 序列化存取（实际业务最常用）

```rust
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct UserSession {
    pub user_id: i64,
    pub email:   String,
    pub role:    String,
    pub login_at: chrono::DateTime<chrono::Utc>,
}

/// 存储 JSON 对象
pub async fn set_json<T: Serialize>(
    pool:    &Pool,
    key:     &str,
    value:   &T,
    ttl_sec: u64,
) -> anyhow::Result<()> {
    let json = serde_json::to_string(value)?;
    let mut conn = pool.get().await?;
    conn.set_ex::<_, _, ()>(key, json, ttl_sec).await?;
    Ok(())
}

/// 读取 JSON 对象
pub async fn get_json<T: for<'de> Deserialize<'de>>(
    pool: &Pool,
    key:  &str,
) -> anyhow::Result<Option<T>> {
    let mut conn = pool.get().await?;
    let raw: Option<String> = conn.get(key).await?;
    match raw {
        Some(s) => Ok(Some(serde_json::from_str(&s)?)),
        None    => Ok(None),
    }
}

// 使用
let session = UserSession {
    user_id:  1,
    email:    "a@b.com".into(),
    role:     "user".into(),
    login_at: chrono::Utc::now(),
};
set_json(&pool, "session:abc123", &session, 7200).await?;

let session: Option<UserSession> = get_json(&pool, "session:abc123").await?;
```

---

## 四、Cache-Aside 缓存模式

```rust
/// 先查缓存，没有就查数据库并写入缓存
pub async fn get_user_cached(
    redis:   &Pool,
    db:      &sqlx::PgPool,
    user_id: i64,
) -> anyhow::Result<Option<User>> {
    let cache_key = format!("user:{user_id}");

    // 1. 查缓存
    if let Some(user) = get_json::<User>(redis, &cache_key).await? {
        return Ok(Some(user));
    }

    // 2. 缓存未命中，查数据库
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id)
        .fetch_optional(db)
        .await?;

    // 3. 写入缓存（10 分钟 TTL）
    if let Some(ref u) = user {
        set_json(redis, &cache_key, u, 600).await?;
    }

    Ok(user)
}

/// 更新数据时删除缓存（Cache Invalidation）
pub async fn update_user(
    redis:   &Pool,
    db:      &sqlx::PgPool,
    user_id: i64,
    email:   &str,
) -> anyhow::Result<()> {
    // 1. 更新数据库
    sqlx::query!("UPDATE users SET email = $1 WHERE id = $2", email, user_id)
        .execute(db)
        .await?;

    // 2. 删除缓存（下次访问重新加载）
    let mut conn = redis.get().await?;
    conn.del::<_, ()>(format!("user:{user_id}")).await?;
    Ok(())
}
```

---

## 五、Session 管理

```rust
use uuid::Uuid;

pub struct SessionStore { pool: Pool }

impl SessionStore {
    pub fn new(pool: Pool) -> Self { Self { pool } }

    fn key(session_id: &str) -> String {
        format!("session:{session_id}")
    }

    /// 创建 session，返回 session_id
    pub async fn create(&self, data: &UserSession, ttl_sec: u64) -> anyhow::Result<String> {
        let session_id = Uuid::new_v4().to_string();
        set_json(&self.pool, &Self::key(&session_id), data, ttl_sec).await?;
        Ok(session_id)
    }

    /// 读取 session
    pub async fn get(&self, session_id: &str) -> anyhow::Result<Option<UserSession>> {
        get_json(&self.pool, &Self::key(session_id)).await
    }

    /// 刷新过期时间
    pub async fn refresh(&self, session_id: &str, ttl_sec: i64) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        conn.expire::<_, ()>(Self::key(session_id), ttl_sec).await?;
        Ok(())
    }

    /// 删除 session（登出）
    pub async fn delete(&self, session_id: &str) -> anyhow::Result<()> {
        let mut conn = self.pool.get().await?;
        conn.del::<_, ()>(Self::key(session_id)).await?;
        Ok(())
    }
}
```

---

## 六、分布式锁

```rust
use std::time::Duration;

pub struct RedisLock {
    pool:  Pool,
    key:   String,
    token: String,   // 唯一值，防止误删他人的锁
}

impl RedisLock {
    /// 尝试加锁（非阻塞）
    /// ttl_ms：锁的最大持有时间（防止持锁者崩溃导致死锁）
    pub async fn try_acquire(
        pool:   &Pool,
        key:    &str,
        ttl_ms: u64,
    ) -> anyhow::Result<Option<RedisLock>> {
        let token  = Uuid::new_v4().to_string();
        let lock_key = format!("lock:{key}");

        let mut conn = pool.get().await?;

        // SET key token NX PX ttl_ms
        // NX = 只在不存在时设置；PX = 毫秒为单位的过期时间
        let ok: bool = redis::cmd("SET")
            .arg(&lock_key)
            .arg(&token)
            .arg("NX")
            .arg("PX")
            .arg(ttl_ms)
            .query_async(&mut conn)
            .await
            .unwrap_or(false);

        if ok {
            Ok(Some(RedisLock { pool: pool.clone(), key: lock_key, token }))
        } else {
            Ok(None)  // 加锁失败（已被其他进程持有）
        }
    }

    /// 加锁（阻塞等待，最多等待 timeout）
    pub async fn acquire(
        pool:    &Pool,
        key:     &str,
        ttl_ms:  u64,
        timeout: Duration,
    ) -> anyhow::Result<RedisLock> {
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            if let Some(lock) = Self::try_acquire(pool, key, ttl_ms).await? {
                return Ok(lock);
            }
            if tokio::time::Instant::now() >= deadline {
                anyhow::bail!("获取锁超时: {key}");
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    /// 释放锁（Lua 脚本保证原子性：只删自己的锁）
    pub async fn release(self) -> anyhow::Result<()> {
        let script = r#"
            if redis.call("GET", KEYS[1]) == ARGV[1] then
                return redis.call("DEL", KEYS[1])
            else
                return 0
            end
        "#;
        let mut conn = self.pool.get().await?;
        redis::Script::new(script)
            .key(&self.key)
            .arg(&self.token)
            .invoke_async::<i64>(&mut conn)
            .await?;
        Ok(())
    }
}

// 使用
async fn process_payment(redis: &Pool, order_id: i64) -> anyhow::Result<()> {
    // 对同一订单加锁，防止并发重复支付
    let lock = RedisLock::acquire(
        redis,
        &format!("payment:{order_id}"),
        30_000,                          // 锁最多持有 30 秒
        Duration::from_secs(5),          // 最多等待 5 秒
    ).await?;

    // 执行业务逻辑（临界区）
    do_payment(order_id).await?;

    // 释放锁
    lock.release().await?;
    Ok(())
}
```

---

## 七、限流（滑动窗口计数器）

```rust
/// 基于 Redis 的滑动窗口限流
/// 返回 Ok(true) = 允许，Ok(false) = 超限
pub async fn check_rate_limit(
    pool:      &Pool,
    key:       &str,    // 限流维度，如 "rate:user:1" 或 "rate:ip:1.2.3.4"
    max_count: i64,     // 窗口内最大请求数
    window_ms: i64,     // 窗口大小（毫秒）
) -> anyhow::Result<bool> {
    // Lua 脚本：原子执行，避免竞态条件
    let script = r#"
        local key       = KEYS[1]
        local now       = tonumber(ARGV[1])
        local window    = tonumber(ARGV[2])
        local max_count = tonumber(ARGV[3])
        local uid       = ARGV[4]

        -- 移除窗口之外的旧记录
        redis.call("ZREMRANGEBYSCORE", key, 0, now - window)

        -- 当前窗口内的请求数
        local count = redis.call("ZCARD", key)

        if count < max_count then
            -- 添加当前请求（score = 时间戳）
            redis.call("ZADD", key, now, uid)
            -- 设置 key 的过期时间（防止内存泄漏）
            redis.call("PEXPIRE", key, window)
            return 1  -- 允许
        else
            return 0  -- 拒绝
        end
    "#;

    let now_ms = chrono::Utc::now().timestamp_millis();
    let uid    = Uuid::new_v4().to_string();  // 每次请求唯一

    let mut conn = pool.get().await?;
    let allowed: i64 = redis::Script::new(script)
        .key(key)
        .arg(now_ms)
        .arg(window_ms)
        .arg(max_count)
        .arg(uid)
        .invoke_async(&mut conn)
        .await?;

    Ok(allowed == 1)
}

// 在 axum 中间件中使用
pub async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Response {
    // 按 IP 或用户 ID 限流
    let key = req.headers()
        .get("x-real-ip")
        .and_then(|v| v.to_str().ok())
        .map(|ip| format!("rate:ip:{ip}"))
        .unwrap_or_else(|| "rate:unknown".to_string());

    match check_rate_limit(&state.redis, &key, 100, 60_000).await {
        Ok(true)  => next.run(req).await,
        Ok(false) => (
            StatusCode::TOO_MANY_REQUESTS,
            Json(serde_json::json!({
                "success": false,
                "error": { "code": "RATE_LIMITED", "message": "请求过于频繁，请稍后重试" }
            }))
        ).into_response(),
        Err(e) => {
            tracing::error!("限流检查失败: {e}");
            next.run(req).await  // Redis 故障时降级放行
        }
    }
}
```

---

## 八、Pub/Sub（简单消息广播）

```rust
// 适合：实时通知、多实例间事件广播
// 不适合：需要持久化、需要确认的场景（用消息队列）

use redis::AsyncCommands;

/// 发布消息
pub async fn publish(pool: &Pool, channel: &str, message: &str) -> anyhow::Result<()> {
    let mut conn = pool.get().await?;
    conn.publish::<_, _, ()>(channel, message).await?;
    Ok(())
}

/// 订阅并处理消息
pub async fn subscribe_loop(redis_url: &str, channel: &str) -> anyhow::Result<()> {
    // Pub/Sub 需要独占连接，不能用连接池
    let client = redis::Client::open(redis_url)?;
    let mut pubsub = client.get_async_pubsub().await?;
    pubsub.subscribe(channel).await?;

    use futures::StreamExt;
    let mut stream = pubsub.on_message();
    while let Some(msg) = stream.next().await {
        let payload: String = msg.get_payload()?;
        tracing::info!(channel = channel, message = %payload, "收到消息");
        handle_message(&payload).await;
    }
    Ok(())
}
```

---

## 速查表

```
连接池：
  Config::from_url(url).create_pool(Some(Runtime::Tokio1))
  pool.get().await?  → 获取连接（自动归还）

常用命令：
  conn.set_ex(key, val, secs)      SET + 过期
  conn.set_nx(key, val)            SET NX（原子占位）
  conn.get(key)                    GET → Option<T>
  conn.del(key)                    DEL
  conn.expire(key, secs)           设置过期
  conn.ttl(key)                    查剩余 TTL
  conn.incr(key, delta)            原子计数

Redis 命令（复杂命令）：
  redis::cmd("SET").arg(...).query_async(&mut conn).await
  redis::Script::new(lua).key(k).arg(a).invoke_async(&mut conn).await

模式：
  Cache-Aside：先查 Redis，未命中查 DB，写入 Redis
  分布式锁：SET NX PX + Lua 脚本原子释放
  限流：ZSET 滑动窗口（Lua 脚本原子执行）
  Session：set_ex 存 JSON，expire 续期，del 登出
```
