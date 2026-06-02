# 数据库操作：SQLx 实战

```toml
[dependencies]
sqlx = { version = "0.7", features = [
    "postgres",     # 或 "mysql" / "sqlite"
    "runtime-tokio-rustls",
    "macros",       # query! 宏
    "migrate",      # 数据库迁移
    "chrono",       # DateTime 类型映射
    "uuid",         # UUID 类型映射
    "json",         # jsonb 类型映射
] }
```

> SQLx 的核心优势：**编译期检查 SQL 语句**（需要连接数据库或使用 offline 模式）。

---

## 一、连接池

```rust
use sqlx::PgPool;

// 创建连接池（应用启动时做一次，全局共享）
let pool = PgPool::connect(&database_url).await?;

// 带配置的连接池
use sqlx::postgres::PgPoolOptions;
let pool = PgPoolOptions::new()
    .max_connections(20)             // 最大连接数（默认 10）
    .min_connections(2)              // 最小维持连接数
    .acquire_timeout(std::time::Duration::from_secs(3))   // 获取连接超时
    .idle_timeout(std::time::Duration::from_secs(600))    // 空闲超时
    .connect(&database_url)
    .await?;

// 连接池状态检查
println!("活跃连接: {}", pool.size());

// 在 axum 中共享：放入 AppState
#[derive(Clone)]
struct AppState { db: PgPool }
```

---

## 二、三种查询方式

### 2.1 query!（编译期检查，推荐）

```rust
// query! 宏：在编译期验证 SQL 语句和返回类型
// 返回匿名结构体，字段名对应列名

// 查询单行（不存在则 Err）
let row = sqlx::query!(
    "SELECT id, username, email FROM users WHERE id = $1",
    user_id  // $1 对应第一个参数，PostgreSQL 风格
)
.fetch_one(&pool)
.await?;

println!("{} {}", row.id, row.username);  // 字段有类型，IDE 可补全

// 查询多行
let rows = sqlx::query!(
    "SELECT id, username FROM users WHERE active = $1 ORDER BY id",
    true
)
.fetch_all(&pool)
.await?;

for row in rows {
    println!("{}: {}", row.id, row.username);
}

// 查询可能为空的单行（不存在返回 None）
let row = sqlx::query!(
    "SELECT id, username FROM users WHERE email = $1",
    email
)
.fetch_optional(&pool)
.await?;

match row {
    Some(r) => println!("找到: {}", r.username),
    None    => println!("不存在"),
}

// 流式处理（大量数据，避免一次性加载进内存）
use futures::TryStreamExt;
let mut stream = sqlx::query!("SELECT id, username FROM users")
    .fetch(&pool);
while let Some(row) = stream.try_next().await? {
    process(row.id, &row.username);
}
```

### 2.2 query_as!（映射到结构体）

```rust
use sqlx::FromRow;

// 结构体字段名必须与列名完全一致（或用 rename）
#[derive(Debug, FromRow)]
struct User {
    pub id:         i64,
    pub username:   String,
    pub email:      String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub active:     bool,
}

// 直接映射到 User 结构体（推荐，比 query! 更清晰）
let user = sqlx::query_as!(
    User,
    "SELECT id, username, email, created_at, active FROM users WHERE id = $1",
    user_id
)
.fetch_one(&pool)
.await?;

// 查询多行
let users: Vec<User> = sqlx::query_as!(
    User,
    "SELECT id, username, email, created_at, active FROM users \
     WHERE active = true \
     ORDER BY created_at DESC \
     LIMIT $1 OFFSET $2",
    per_page,
    (page - 1) * per_page
)
.fetch_all(&pool)
.await?;
```

### 2.3 query（运行时，无编译检查，适合动态 SQL）

```rust
// 不用 ! 的版本，无编译期检查，但可以动态构建
let row = sqlx::query("SELECT id, username FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_one(&pool)
    .await?;

let id: i64    = row.get("id");
let name: String = row.get("username");
// 或按位置：row.get::<i64, _>(0)
```

---

## 三、增删改

```rust
// INSERT 并获取新记录
let user = sqlx::query_as!(
    User,
    r#"
    INSERT INTO users (username, email, password_hash, created_at)
    VALUES ($1, $2, $3, NOW())
    RETURNING id, username, email, created_at, active
    "#,
    username,
    email,
    password_hash,
)
.fetch_one(&pool)
.await?;

// INSERT，不需要返回值
sqlx::query!(
    "INSERT INTO audit_logs (user_id, action, created_at) VALUES ($1, $2, NOW())",
    user_id,
    action,
)
.execute(&pool)
.await?;

// UPDATE 并检查是否更新了记录
let result = sqlx::query!(
    "UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2",
    new_email,
    user_id,
)
.execute(&pool)
.await?;

if result.rows_affected() == 0 {
    return Err(AppError::UserNotFound { id: user_id });
}

// DELETE
let result = sqlx::query!("DELETE FROM users WHERE id = $1", user_id)
    .execute(&pool)
    .await?;

println!("删除了 {} 条记录", result.rows_affected());

// UPSERT（INSERT ON CONFLICT）
sqlx::query!(
    r#"
    INSERT INTO user_settings (user_id, key, value)
    VALUES ($1, $2, $3)
    ON CONFLICT (user_id, key) DO UPDATE SET value = EXCLUDED.value
    "#,
    user_id, key, value,
)
.execute(&pool)
.await?;
```

---

## 四、事务

```rust
// 简单事务
let mut tx = pool.begin().await?;

let user = sqlx::query_as!(
    User,
    "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *",
    username, email,
)
.fetch_one(&mut *tx)   // ← 注意：传 &mut *tx 而不是 &pool
.await?;

sqlx::query!(
    "INSERT INTO user_profiles (user_id, bio) VALUES ($1, '')",
    user.id,
)
.execute(&mut *tx)
.await?;

tx.commit().await?;    // 提交
// tx.rollback().await?;   // 回滚（或者直接 drop tx，自动回滚）

// ── 封装成函数 ──
async fn create_user_with_profile(
    pool: &PgPool,
    username: &str,
    email: &str,
) -> Result<User, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = async {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (username, email) VALUES ($1, $2) RETURNING *",
            username, email,
        )
        .fetch_one(&mut *tx)
        .await?;

        sqlx::query!(
            "INSERT INTO user_profiles (user_id) VALUES ($1)",
            user.id,
        )
        .execute(&mut *tx)
        .await?;

        Ok::<User, sqlx::Error>(user)
    }.await;

    match result {
        Ok(user) => { tx.commit().await?; Ok(user) }
        Err(e)   => { tx.rollback().await?; Err(e) }
    }
}
```

---

## 五、批量操作

```rust
// 批量 INSERT（UNNEST 方式，PostgreSQL 推荐）
let user_ids: Vec<i64>   = vec![1, 2, 3];
let tags:     Vec<String> = vec!["rust".into(), "go".into(), "python".into()];

sqlx::query!(
    r#"
    INSERT INTO user_tags (user_id, tag)
    SELECT * FROM UNNEST($1::bigint[], $2::text[])
    "#,
    &user_ids[..] as &[i64],
    &tags[..] as &[String],
)
.execute(&pool)
.await?;

// 批量 INSERT（逐条，事务内，适合跨数据库）
let mut tx = pool.begin().await?;
for item in items {
    sqlx::query!(
        "INSERT INTO logs (user_id, message) VALUES ($1, $2)",
        item.user_id, item.message,
    )
    .execute(&mut *tx)
    .await?;
}
tx.commit().await?;

// 批量查询（IN 语句）
let ids = vec![1_i64, 2, 3, 4, 5];
let users = sqlx::query_as!(
    User,
    "SELECT * FROM users WHERE id = ANY($1)",  // PostgreSQL
    &ids[..] as &[i64],
)
.fetch_all(&pool)
.await?;
```

---

## 六、数据库迁移

```toml
# Cargo.toml
[dependencies]
sqlx = { version = "0.7", features = ["migrate", ...] }
```

```
# 目录结构
migrations/
├── 20240101000001_create_users.sql
├── 20240101000002_create_orders.sql
└── 20240102000001_add_user_avatar.sql
```

```sql
-- migrations/20240101000001_create_users.sql
CREATE TABLE users (
    id           BIGSERIAL PRIMARY KEY,
    username     VARCHAR(50)  NOT NULL UNIQUE,
    email        VARCHAR(255) NOT NULL UNIQUE,
    password_hash TEXT        NOT NULL,
    active       BOOLEAN     NOT NULL DEFAULT true,
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users(email);
```

```rust
// 应用启动时自动执行未执行的迁移
sqlx::migrate!("./migrations")
    .run(&pool)
    .await
    .expect("数据库迁移失败");

// main.rs 中
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = PgPool::connect(&config.database_url).await?;

    // 先迁移，再启动服务
    sqlx::migrate!("./migrations").run(&pool).await?;

    // 启动 axum...
    Ok(())
}
```

```bash
# CLI 命令（需安装 sqlx-cli）
cargo install sqlx-cli
sqlx migrate add create_users     # 创建迁移文件
sqlx migrate run                  # 执行迁移
sqlx migrate revert               # 回滚最后一次迁移
sqlx migrate info                 # 查看状态
```

---

## 七、类型映射速查

```
PostgreSQL              Rust
─────────────────────────────────────────
BIGINT / BIGSERIAL      i64
INTEGER / SERIAL        i32
SMALLINT                i16
BOOLEAN                 bool
REAL                    f32
DOUBLE PRECISION        f64
TEXT / VARCHAR          String
BYTEA                   Vec<u8>
UUID                    uuid::Uuid       （需 features = ["uuid"]）
TIMESTAMPTZ             DateTime<Utc>    （需 features = ["chrono"]）
TIMESTAMP               NaiveDateTime    （需 features = ["chrono"]）
DATE                    NaiveDate        （需 features = ["chrono"]）
JSONB / JSON            serde_json::Value（需 features = ["json"]）
BIGINT[]                Vec<i64>
TEXT[]                  Vec<String>

可空列（NULL）          Option<T>
```

---

## 八、常见模式

### 8.1 分页查询

```rust
#[derive(Deserialize)]
struct PageParams { page: u32, per_page: u32 }

async fn list_users_paginated(
    pool: &PgPool,
    params: &PageParams,
) -> Result<(Vec<User>, u64), sqlx::Error> {
    let offset = (params.page.saturating_sub(1)) as i64 * params.per_page as i64;
    let limit  = params.per_page as i64;

    // 用 JOIN LATERAL 或 CTE 一次查出总数和数据（避免两次查询）
    let total: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM users WHERE active = true")
        .fetch_one(pool)
        .await?
        .unwrap_or(0);

    let users = sqlx::query_as!(
        User,
        "SELECT * FROM users WHERE active = true ORDER BY id LIMIT $1 OFFSET $2",
        limit, offset,
    )
    .fetch_all(pool)
    .await?;

    Ok((users, total as u64))
}
```

### 8.2 动态条件查询（QueryBuilder）

```rust
use sqlx::QueryBuilder;

async fn search_users(
    pool: &PgPool,
    keyword:  Option<&str>,
    active:   Option<bool>,
    min_age:  Option<i32>,
) -> Result<Vec<User>, sqlx::Error> {
    let mut builder = QueryBuilder::new(
        "SELECT id, username, email FROM users WHERE 1=1"
    );

    if let Some(kw) = keyword {
        builder.push(" AND username ILIKE ");
        builder.push_bind(format!("%{kw}%"));
    }
    if let Some(a) = active {
        builder.push(" AND active = ");
        builder.push_bind(a);
    }
    if let Some(age) = min_age {
        builder.push(" AND age >= ");
        builder.push_bind(age);
    }

    builder.push(" ORDER BY id LIMIT 100");

    builder.build_query_as::<User>()
        .fetch_all(pool)
        .await
}
```

### 8.3 把 Repository 封装为 struct

```rust
#[derive(Clone)]
pub struct UserRepository { pool: PgPool }

impl UserRepository {
    pub fn new(pool: PgPool) -> Self { Self { pool } }

    pub async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await
    }

    pub async fn create(&self, username: &str, email: &str, hash: &str) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (username, email, password_hash) VALUES ($1,$2,$3) RETURNING *",
            username, email, hash,
        )
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_email(&self, id: i64, email: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "UPDATE users SET email = $1, updated_at = NOW() WHERE id = $2",
            email, id,
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let r = sqlx::query!("DELETE FROM users WHERE id = $1", id)
            .execute(&self.pool)
            .await?;
        Ok(r.rows_affected() > 0)
    }
}
```

---

## 九、Offline 模式（CI/CD 不连接数据库）

```bash
# 开发时：保存查询元数据
export DATABASE_URL=postgres://user:pass@localhost/mydb
cargo sqlx prepare      # 生成 .sqlx/ 目录

# CI 环境：使用保存的元数据，无需数据库
SQLX_OFFLINE=true cargo build
```

```toml
# .cargo/config.toml（或环境变量）
[env]
SQLX_OFFLINE = "true"
```

---

## 速查表

```
PgPool::connect(url)                        创建连接池
PgPoolOptions::new()....connect(url)        带配置的连接池

sqlx::query!("SQL", args)                  编译期检查，返回匿名结构体
sqlx::query_as!(Type, "SQL", args)         编译期检查，返回 Type
sqlx::query("SQL").bind(val)               运行时，无检查

.fetch_one(&pool)                           返回一行，无则 Err
.fetch_optional(&pool)                      返回 Option<row>
.fetch_all(&pool)                           返回 Vec<row>
.fetch(&pool)                               返回流（Stream）
.execute(&pool)                             执行，返回 QueryResult

pool.begin().await                          开始事务（返回 Transaction）
tx.commit().await                           提交
tx.rollback().await                         回滚（drop 也会自动回滚）
// 事务内的查询传 &mut *tx 而非 &pool

sqlx::migrate!("./migrations").run(&pool)  执行迁移
result.rows_affected()                      影响的行数
```
