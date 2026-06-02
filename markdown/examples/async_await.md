# Rust vs TypeScript: async/await

**运行命令：** `cargo run -p learning_notes --example rts_async_await`

## TypeScript 版本

```ts
async function fetchUser(id: number): Promise<User> {
    const res = await fetch(`/users/${id}`);
    return res.json();
}

fetchUser(1)
    .then(user => console.log(user))
    .catch(e => console.error(e));

const [user, posts] = await Promise.all([fetchUser(1), fetchPosts(1)]);

const result = await Promise.race([task1(), task2()]);

try {
    const user = await fetchUser(1);
} catch (e) {
    console.error("失败:", e);
}

await new Promise(resolve => setTimeout(resolve, 1000));

for await (const item of asyncIterable) { ... }
```

## Rust async 与 TS 的关键差异

1. **Future 是惰性的**：创建后不会自动执行，必须被 `.await` 或 runtime 驱动
2. **需要显式 runtime**（tokio/async-std），TS 内置 event loop
3. `async fn` 返回 `impl Future<Output=T>`，不是 `Promise<T>`
4. Rust 没有内置 async runtime，tokio 是最常用的选择

## 一、基本 async 函数

```rust
use std::time::Duration;
use tokio::time::sleep;

async fn greet(name: &str) -> String {
    format!("你好，{name}！")
}
```

## 二、模拟异步操作

```rust
async fn fetch_user(id: u32) -> Result<String, String> {
    sleep(Duration::from_millis(10)).await;
    match id {
        1 => Ok(String::from("Alice")),
        2 => Ok(String::from("Bob")),
        _ => Err(format!("用户 {id} 不存在")),
    }
}

async fn fetch_posts(user_id: u32) -> Vec<String> {
    sleep(Duration::from_millis(10)).await;
    vec![
        format!("用户{user_id}的文章1"),
        format!("用户{user_id}的文章2"),
    ]
}
```

## 三、async 错误处理

```rust
async fn get_user_greeting(id: u32) -> Result<String, String> {
    let user = fetch_user(id).await?;
    Ok(format!("欢迎，{user}！"))
}
```

## 四、并发执行（tokio::join!）

```rust
async fn concurrent_demo() {
    let (user_result, posts) = tokio::join!(
        fetch_user(1),
        fetch_posts(1),
    );
    println!("用户: {:?}", user_result);
    println!("文章: {:?}", posts);
}
```

## 五、并发 + 错误处理（try_join!）

```rust
async fn try_join_demo() {
    match tokio::try_join!(fetch_user(1), fetch_user(2)) {
        Ok((u1, u2)) => println!("两个用户: {u1}, {u2}"),
        Err(e)       => println!("有错误: {e}"),
    }

    match tokio::try_join!(fetch_user(1), fetch_user(99)) {
        Ok((u1, u2)) => println!("两个用户: {u1}, {u2}"),
        Err(e)       => println!("失败: {e}"),
    }
}
```

## 六、spawn（后台任务）

```rust
async fn spawn_demo() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(5)).await;
        String::from("后台任务完成")
    });
    println!("主任务继续...");
    let result = handle.await.unwrap();
    println!("后台任务结果: {result}");
}
```

## 七、超时控制

```rust
async fn timeout_demo() {
    let result = tokio::time::timeout(
        Duration::from_millis(5),
        fetch_user(1),
    ).await;

    match result {
        Ok(Ok(user))  => println!("成功: {user}"),
        Ok(Err(e))    => println!("任务失败: {e}"),
        Err(_elapsed) => println!("超时！"),
    }
}
```

## 八、惰性 vs 立即执行

```rust
async fn lazy_demo() {
    let future = fetch_user(1);
    println!("Future 已创建，但还没执行");
    let result = future.await;
    println!("现在才执行完: {:?}", result);
}
```

## main 入口

```rust
#[tokio::main]
async fn main() {
    let msg = greet("Rust").await;
    println!("{msg}");

    match get_user_greeting(1).await {
        Ok(msg)  => println!("{msg}"),
        Err(e)   => println!("错误: {e}"),
    }

    concurrent_demo().await;
    try_join_demo().await;
    spawn_demo().await;
    timeout_demo().await;
    lazy_demo().await;
}
```

## 九、select!（Promise.race 的对应）

```rust
use tokio::time::{sleep, Duration};

async fn fetch_from_primary() -> &'static str {
    sleep(Duration::from_millis(80)).await;
    "主数据库结果"
}

async fn fetch_from_replica() -> &'static str {
    sleep(Duration::from_millis(50)).await;
    "副本结果（更快）"
}

async fn select_demo() {
    // tokio::select! — 哪个 Future 先完成就取哪个，取消其余的
    // 对应 TypeScript 的 Promise.race([...])
    let result = tokio::select! {
        r = fetch_from_primary() => format!("primary 赢了: {}", r),
        r = fetch_from_replica() => format!("replica 赢了: {}", r),
    };
    println!("{}", result); // "replica 赢了: 副本结果（更快）"
}

async fn timeout_with_select() {
    // select! + sleep 实现超时（比 timeout() 更灵活）
    tokio::select! {
        result = fetch_from_primary() => {
            println!("成功: {}", result);
        }
        _ = sleep(Duration::from_millis(30)) => {
            println!("超时！30ms 内无响应");
        }
    }
}

async fn select_with_channel() {
    use tokio::sync::mpsc;

    let (tx1, mut rx1) = mpsc::channel::<&str>(1);
    let (tx2, mut rx2) = mpsc::channel::<&str>(1);

    tokio::spawn(async move { tx1.send("来自 channel 1").await.unwrap(); });
    tokio::spawn(async move { tx2.send("来自 channel 2").await.unwrap(); });

    // 监听多个 channel，处理最先到达的那个
    tokio::select! {
        Some(msg) = rx1.recv() => println!("rx1: {}", msg),
        Some(msg) = rx2.recv() => println!("rx2: {}", msg),
    }
}
```

**与 Promise.race 的差异：**
- `select!` 的落败分支被**取消**（drop），不是继续在后台运行
- 支持模式匹配，可以用不同方式处理不同分支的结果
- 可以同时监听 channel 和普通 Future

---

## TS vs Rust async 核心区别

| 特性 | TypeScript | Rust |
|------|-----------|------|
| Future/Promise | 创建即执行 | 惰性，需 .await |
| Runtime | 内置 event loop | 需 #[tokio::main] |
| 并发 | `Promise.all([...])` | `tokio::join!(...)` |
| 后台任务 | Promise 自动后台 | `tokio::spawn(...)` |
| 超时 | `Promise.race([...])` | `tokio::time::timeout` |
| 顶层 await | 支持 | 需 async fn main |
