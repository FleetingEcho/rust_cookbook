# Async / Tokio 实战模式

```toml
[dependencies]
tokio  = { version = "1", features = ["full"] }
```

---

## 一、Tokio 基础工具

### 1.1 spawn：后台任务

```rust
use tokio::task;

// spawn 产生独立异步任务（类似轻量线程），立即返回 JoinHandle
let handle = tokio::spawn(async {
    println!("在独立任务中运行");
    42  // 任务的返回值
});

// 等待任务结束并拿到返回值
let result = handle.await.unwrap();  // Result<T, JoinError>
println!("任务返回: {result}");

// 不关心结果时直接 drop handle
tokio::spawn(async { background_job().await; });

// spawn 的任务要求 'static + Send
// 如果需要共享数据，用 Arc 包裹
let data = Arc::new(vec![1, 2, 3]);
let data_clone = Arc::clone(&data);
tokio::spawn(async move {
    println!("{:?}", data_clone);
});
```

### 1.2 sleep / timeout / interval

```rust
use tokio::time::{sleep, timeout, interval, Duration};

// sleep：异步等待（不阻塞线程）
sleep(Duration::from_secs(1)).await;
sleep(Duration::from_millis(500)).await;

// timeout：给 Future 设置超时
match timeout(Duration::from_secs(5), some_async_fn()).await {
    Ok(result) => println!("成功: {:?}", result),
    Err(_)     => println!("超时了"),
}

// 更常用的写法（直接用 ?）
let result = timeout(Duration::from_secs(5), fetch_data())
    .await
    .context("请求超时")?   // Elapsed error
    ?;                       // 内部 fetch_data 的 error

// interval：定时循环
let mut ticker = interval(Duration::from_secs(10));
loop {
    ticker.tick().await;     // 第一次立即触发
    do_periodic_task().await;
}
```

---

## 二、并发执行多个 Future

### 2.1 join!：等待所有完成

```rust
use tokio::join;

// 并发执行，等所有完成才返回（任何一个失败则整体等待结束后返回所有结果）
let (user, orders, settings) = join!(
    fetch_user(user_id),
    fetch_orders(user_id),
    fetch_settings(user_id),
);

// 处理各自的 Result
let user     = user?;
let orders   = orders?;
let settings = settings?;

// try_join!：任何一个失败立即返回 Err，其余任务继续运行直到完成
use tokio::try_join;
let (user, orders) = try_join!(
    fetch_user(user_id),
    fetch_orders(user_id),
)?;  // 任何一个 Err 直接传播
```

### 2.2 select!：哪个先完成用哪个

```rust
use tokio::select;

// 等待多个 Future，第一个完成的分支执行，其余 Future 被取消
select! {
    result = fetch_from_cache() => {
        println!("缓存命中: {:?}", result);
    }
    result = fetch_from_db() => {
        println!("从数据库获取: {:?}", result);
    }
}

// 带超时的 select
select! {
    result = some_operation() => result?,
    _ = sleep(Duration::from_secs(30)) => {
        bail!("操作超时");
    }
}

// 在循环中使用 select（消息处理）
loop {
    select! {
        Some(msg) = rx.recv() => {
            handle_message(msg).await;
        }
        _ = shutdown_signal() => {
            println!("收到关闭信号，退出");
            break;
        }
        _ = ticker.tick() => {
            do_heartbeat().await;
        }
    }
}
```

### 2.3 JoinSet：动态数量的并发任务

```rust
use tokio::task::JoinSet;

// 当任务数量不固定时（运行时决定），用 JoinSet 代替 join!
let mut set = JoinSet::new();

for user_id in user_ids {
    set.spawn(async move {
        fetch_user(user_id).await
    });
}

// 收集所有结果（哪个先完成先返回）
let mut results = Vec::new();
while let Some(result) = set.join_next().await {
    match result {
        Ok(Ok(user))  => results.push(user),
        Ok(Err(e))    => eprintln!("任务失败: {e}"),
        Err(je)       => eprintln!("任务 panic: {je}"),
    }
}

// 限制并发数量（避免同时发太多请求）
let semaphore = Arc::new(tokio::sync::Semaphore::new(10)); // 最多 10 个并发

for id in ids {
    let permit = semaphore.clone().acquire_owned().await.unwrap();
    set.spawn(async move {
        let _permit = permit;  // permit drop 时自动释放
        fetch(id).await
    });
}
```

---

## 三、Channel：任务间通信

```rust
// 选型速查：
// mpsc   → 多生产者单消费者（最常用，任务队列、事件流）
// oneshot → 一次性发送（请求-响应模式）
// broadcast → 一对多广播（订阅通知）
// watch  → 最新值订阅（配置变更、状态更新）
```

### 3.1 mpsc（任务队列）

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<String>(32);  // 有界，容量 32

// 生产者（可以 clone tx 给多个任务）
let tx1 = tx.clone();
tokio::spawn(async move {
    tx1.send("消息1".to_string()).await.unwrap();
});

let tx2 = tx.clone();
tokio::spawn(async move {
    tx2.send("消息2".to_string()).await.unwrap();
});
drop(tx);  // 关闭最后一个发送端，接收端会收到 None

// 消费者
while let Some(msg) = rx.recv().await {
    println!("收到: {msg}");
}
// 所有发送端都 drop 后，rx.recv() 返回 None，循环退出
```

### 3.2 oneshot（请求-响应）

```rust
use tokio::sync::oneshot;

// 场景：把计算任务发给工作线程，等待结果
let (tx, rx) = oneshot::channel::<i32>();

tokio::spawn(async move {
    let result = heavy_computation();
    tx.send(result).ok();  // 接收方可能已经不等了
});

match rx.await {
    Ok(result) => println!("结果: {result}"),
    Err(_)     => println!("发送方已关闭"),
}
```

### 3.3 broadcast（订阅通知）

```rust
use tokio::sync::broadcast;

let (tx, _) = broadcast::channel::<String>(16);

// 每个订阅者获得独立的接收端
let mut rx1 = tx.subscribe();
let mut rx2 = tx.subscribe();

tokio::spawn(async move {
    while let Ok(msg) = rx1.recv().await {
        println!("订阅者1: {msg}");
    }
});

tokio::spawn(async move {
    while let Ok(msg) = rx2.recv().await {
        println!("订阅者2: {msg}");
    }
});

tx.send("广播消息".to_string()).unwrap();
```

### 3.4 watch（最新值）

```rust
use tokio::sync::watch;

// 场景：配置热更新、开关状态广播
let (tx, rx) = watch::channel(false);  // 初始值 false

// 监听方：只关心最新值
let mut rx_clone = rx.clone();
tokio::spawn(async move {
    loop {
        rx_clone.changed().await.unwrap();  // 等待值变化
        let val = *rx_clone.borrow();
        println!("配置变更为: {val}");
    }
});

// 发送方：更新值
tx.send(true).unwrap();
sleep(Duration::from_secs(1)).await;
tx.send(false).unwrap();
```

---

## 四、async trait

### 4.1 Rust 1.75+ 原生支持

```rust
// Rust 1.75+ 可以直接在 trait 中写 async fn（不需要 async-trait crate）
trait DataFetcher {
    async fn fetch(&self, id: u64) -> Result<String, AppError>;
    async fn fetch_all(&self) -> Result<Vec<String>, AppError>;
}

struct DbFetcher { pool: sqlx::PgPool }

impl DataFetcher for DbFetcher {
    async fn fetch(&self, id: u64) -> Result<String, AppError> {
        // ...
        Ok("data".into())
    }
    async fn fetch_all(&self) -> Result<Vec<String>, AppError> {
        Ok(vec![])
    }
}
```

### 4.2 dyn Trait + async（需要 async-trait crate）

```rust
// 原生 async trait 不能做 dyn Trait（对象安全限制）
// 需要动态派发时，用 async-trait crate

// Cargo.toml: async-trait = "0.1"
use async_trait::async_trait;

#[async_trait]
trait Repository: Send + Sync {
    async fn find_by_id(&self, id: u64) -> Result<User, AppError>;
    async fn save(&self, user: &User) -> Result<(), AppError>;
}

#[async_trait]
impl Repository for PgRepository {
    async fn find_by_id(&self, id: u64) -> Result<User, AppError> {
        // ...
        Ok(User::default())
    }
    async fn save(&self, user: &User) -> Result<(), AppError> {
        Ok(())
    }
}

// 可以 Box<dyn Repository>（测试时传 mock）
async fn handler(repo: &dyn Repository) -> Result<User, AppError> {
    repo.find_by_id(1).await
}
```

---

## 五、Mutex 在 async 中的正确使用

### 5.1 async 中的 Mutex 选择

```rust
// 规则：
// tokio::sync::Mutex → 持有锁时需要 .await（会跨 await 点）
// std::sync::Mutex   → 持有锁时不跨 .await（在同一个同步块内完成）

use tokio::sync::Mutex as AsyncMutex;
use std::sync::Mutex as SyncMutex;

// ✅ 锁的范围跨越 await → 用 tokio::Mutex
let cache = Arc::new(AsyncMutex::new(HashMap::new()));
let cache_clone = Arc::clone(&cache);

tokio::spawn(async move {
    let mut map = cache_clone.lock().await;  // async 锁
    let result = fetch_data().await;          // ← 锁跨越了 await 点
    map.insert("key", result);
    // lock guard drop 在这里
});

// ✅ 锁的范围不跨 await → 用 std::Mutex（性能更好）
let counter = Arc::new(SyncMutex::new(0_u64));

tokio::spawn(async move {
    {
        let mut n = counter.lock().unwrap();  // 同步锁
        *n += 1;
        // guard 在这里 drop，不跨 await
    }
    some_async_fn().await;  // await 在锁释放之后
});
```

### 5.2 常见死锁：持有 Mutex 跨 .await

```rust
// ❌ 死锁风险：std::Mutex 持有时跨了 await
let data = Arc::new(std::sync::Mutex::new(vec![]));

async fn bad_example(data: Arc<std::sync::Mutex<Vec<i32>>>) {
    let mut guard = data.lock().unwrap();   // 加锁
    let new_item = fetch_item().await;       // ← await：当前任务让出，
                                             //   其他任务可能也想拿这把锁
                                             //   → 死锁！
    guard.push(new_item);
}

// ✅ 解决方案1：提前拿数据，释放锁，再 await
async fn good_1(data: Arc<std::sync::Mutex<Vec<i32>>>) {
    let snapshot = {
        data.lock().unwrap().clone()  // 拿完立即 drop
    };
    let new_item = fetch_item().await;
    data.lock().unwrap().push(new_item);
}

// ✅ 解决方案2：改用 tokio::Mutex
async fn good_2(data: Arc<tokio::sync::Mutex<Vec<i32>>>) {
    let mut guard = data.lock().await;       // async 锁，可以安全跨 await
    let new_item = fetch_item().await;
    guard.push(new_item);
}
```

---

## 六、spawn_blocking：在 async 中运行阻塞代码

```rust
// async runtime 的线程不能被阻塞，否则会影响其他任务
// CPU 密集计算或同步阻塞 IO 要放到专用线程池

// ❌ 直接在 async 中做 CPU 密集计算，阻塞整个线程
async fn bad() -> u64 {
    heavy_cpu_computation()  // 阻塞了 tokio 的工作线程
}

// ✅ 用 spawn_blocking 交给线程池
async fn good() -> anyhow::Result<u64> {
    let result = tokio::task::spawn_blocking(|| {
        heavy_cpu_computation()  // 在独立线程中运行，不阻塞 async runtime
    }).await?;  // JoinError（panic）转换
    Ok(result)
}

// ✅ 同步文件 IO 也用 spawn_blocking（或改用 tokio::fs）
async fn read_file(path: &str) -> anyhow::Result<String> {
    let path = path.to_string();
    let content = tokio::task::spawn_blocking(move || {
        std::fs::read_to_string(&path)
    }).await??;  // 第一个 ? 处理 JoinError，第二个处理 io::Error
    Ok(content)
}

// 推荐：直接用 tokio 的 async 版 IO
async fn read_file_async(path: &str) -> anyhow::Result<String> {
    Ok(tokio::fs::read_to_string(path).await?)
}
```

---

## 七、优雅关闭（Graceful Shutdown）

```rust
use tokio::signal;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 创建关闭信号（CancellationToken 是最推荐的方式）
    let token = tokio_util::sync::CancellationToken::new();

    // 后台任务：监听关闭
    let token_clone = token.clone();
    tokio::spawn(async move {
        loop {
            select! {
                _ = token_clone.cancelled() => {
                    println!("收到关闭信号，后台任务退出");
                    break;
                }
                _ = do_background_work() => {}
            }
        }
    });

    // 等待 Ctrl+C 或 SIGTERM
    signal::ctrl_c().await?;
    println!("开始优雅关闭...");

    // 触发关闭
    token.cancel();

    // 给任务一点时间清理
    sleep(Duration::from_secs(5)).await;
    println!("关闭完成");
    Ok(())
}
```

---

## 八、常见模式速查

```rust
// ─ 并发请求多个接口，全部成功才继续 ─
let (a, b, c) = try_join!(api_a(), api_b(), api_c())?;

// ─ 并发请求，有一个成功就用（竞速）─
select! {
    Ok(r) = fast_source()  => use_result(r),
    Ok(r) = slow_source()  => use_result(r),
}

// ─ 动态数量并发，限速 ─
let sem = Arc::new(Semaphore::new(5));
let mut set = JoinSet::new();
for id in ids {
    let sem = sem.clone();
    set.spawn(async move {
        let _p = sem.acquire_owned().await?;
        fetch(id).await
    });
}

// ─ 定时任务 ─
let mut interval = tokio::time::interval(Duration::from_secs(60));
loop {
    interval.tick().await;
    cleanup_expired_sessions().await;
}

// ─ 带超时重试 ─
for attempt in 0..3 {
    match timeout(Duration::from_secs(5), operation()).await {
        Ok(Ok(result)) => return Ok(result),
        Ok(Err(e))     => eprintln!("第 {attempt} 次失败: {e}"),
        Err(_)         => eprintln!("第 {attempt} 次超时"),
    }
    sleep(Duration::from_millis(500 * 2_u64.pow(attempt))).await;  // 指数退避
}
bail!("重试 3 次后仍然失败");
```

---

## 速查表

```
tokio::spawn(async { })            产生独立异步任务
tokio::time::sleep(dur).await      异步等待
tokio::time::timeout(dur, fut)     给 Future 设置超时
tokio::time::interval(dur)         定时触发

join!(f1, f2, f3)                  并发等待所有（不短路）
try_join!(f1, f2, f3)              并发等待所有（任意 Err 则短路）
select! { pat = fut => {} }        等待第一个完成
JoinSet::spawn / join_next         动态数量并发任务

mpsc::channel(n)                   有界多生产者单消费者
oneshot::channel()                 一次性收发
broadcast::channel(n)              一对多广播
watch::channel(init)               最新值订阅

Semaphore::new(n)                  并发数量限制
tokio::sync::Mutex                 async 场景用（可跨 await）
std::sync::Mutex                   不跨 await 时更快

spawn_blocking(|| { })             在线程池运行阻塞代码
tokio::fs::*                       异步文件 IO
```
