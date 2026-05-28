// 运行命令：cargo run -p learning_notes --example rts_async_await
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // async/await
// async function fetchUser(id: number): Promise<User> {
//     const res = await fetch(`/users/${id}`);
//     return res.json();
// }
//
// // Promise 链
// fetchUser(1)
//     .then(user => console.log(user))
//     .catch(e => console.error(e));
//
// // 并发执行
// const [user, posts] = await Promise.all([fetchUser(1), fetchPosts(1)]);
//
// // 竞争执行
// const result = await Promise.race([task1(), task2()]);
//
// // 错误处理
// try {
//     const user = await fetchUser(1);
// } catch (e) {
//     console.error("失败:", e);
// }
//
// // setTimeout
// await new Promise(resolve => setTimeout(resolve, 1000));
//
// // 异步迭代器
// for await (const item of asyncIterable) { ... }
// ============================================================
//
// Rust async 与 TS 的关键差异：
// 1. Future 是惰性的：创建后不会自动执行，必须被 .await 或 runtime 驱动
// 2. 需要显式 runtime（tokio/async-std），TS 内置 event loop
// 3. async fn 返回 impl Future<Output=T>，不是 Promise<T>
// 4. Rust 没有内置 async runtime，tokio 是最常用的选择

use std::time::Duration;
use tokio::time::sleep;

// ============================================================
// 一、基本 async 函数
// TS: async function greet(name: string): Promise<string>
// ============================================================
async fn greet(name: &str) -> String {
    // async fn 返回 impl Future<Output=String>
    // 调用时不会立即执行，必须 .await
    format!("你好，{name}！")
}

// ============================================================
// 二、模拟异步操作（带延迟）
// TS: await new Promise(resolve => setTimeout(resolve, ms))
// ============================================================
async fn fetch_user(id: u32) -> Result<String, String> {
    sleep(Duration::from_millis(10)).await;  // 模拟网络延迟

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

// ============================================================
// 三、async 错误处理（与 Option/Result 结合）
// TS: try/catch + await
// ============================================================
async fn get_user_greeting(id: u32) -> Result<String, String> {
    let user = fetch_user(id).await?;   // ? 在 async fn 中同样适用
    Ok(format!("欢迎，{user}！"))
}

// ============================================================
// 四、并发执行（tokio::join!）
// TS: Promise.all([...])
// ============================================================
async fn concurrent_demo() {
    println!("=== 并发执行（类似 Promise.all）===");

    // tokio::join! 同时等待多个 Future（并发，不是并行）
    // TS: const [user, posts] = await Promise.all([fetchUser(1), fetchPosts(1)])
    let (user_result, posts) = tokio::join!(
        fetch_user(1),
        fetch_posts(1),
    );

    println!("用户: {:?}", user_result);
    println!("文章: {:?}", posts);
}

// ============================================================
// 五、并发 + 错误处理（try_join!）
// TS: Promise.all（任一失败则整体失败）
// ============================================================
async fn try_join_demo() {
    println!("\n=== try_join!（任一失败则整体失败）===");

    // tokio::try_join! 中任一 Future 失败则立即返回 Err
    // TS: Promise.all([...])（默认行为就是这样）
    match tokio::try_join!(
        fetch_user(1),
        fetch_user(2),
    ) {
        Ok((u1, u2)) => println!("两个用户: {u1}, {u2}"),
        Err(e)       => println!("有错误: {e}"),
    }

    // 其中一个失败
    match tokio::try_join!(
        fetch_user(1),
        fetch_user(99),  // 这个会失败
    ) {
        Ok((u1, u2)) => println!("两个用户: {u1}, {u2}"),
        Err(e)       => println!("失败: {e}"),
    }
}

// ============================================================
// 六、spawn（后台任务）
// TS: Promise 自动在后台运行
// Rust: Future 是惰性的，必须 spawn 才能后台运行
// ============================================================
async fn spawn_demo() {
    println!("\n=== tokio::spawn（后台任务）===");

    // tokio::spawn 把 Future 提交给 runtime 后台运行
    // TS: fetchUser(1)（Promise 创建后自动开始执行）
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(5)).await;
        String::from("后台任务完成")
    });

    // 主任务继续执行
    println!("主任务继续...");

    // 等待后台任务完成
    // TS: await promise
    let result = handle.await.unwrap();
    println!("后台任务结果: {result}");
}

// ============================================================
// 七、超时（tokio::time::timeout）
// TS: Promise.race([task, new Promise((_, reject) => setTimeout(reject, ms))])
// ============================================================
async fn timeout_demo() {
    println!("\n=== 超时控制 ===");

    // TS: await Promise.race([fetch(...), timeout(1000)])
    let result = tokio::time::timeout(
        Duration::from_millis(5),
        fetch_user(1),   // 这个任务 10ms，超时设置 5ms
    ).await;

    match result {
        Ok(Ok(user))  => println!("成功: {user}"),
        Ok(Err(e))    => println!("任务失败: {e}"),
        Err(_elapsed) => println!("超时！"),
    }
}

// ============================================================
// 八、惰性 vs 立即执行（Rust 与 TS 最大的概念差异）
// ============================================================
async fn lazy_demo() {
    println!("\n=== Future 的惰性 ===");

    // TS: Promise 创建后立即开始执行
    // let p = fetch(url);  // 立即开始网络请求！

    // Rust: Future 创建后什么都不做
    let future = fetch_user(1);  // 什么都没发生！
    println!("Future 已创建，但还没执行");

    // 只有 .await 才驱动 Future 执行
    let result = future.await;
    println!("现在才执行完: {:?}", result);

    // 实际意义：Rust 可以精确控制何时执行，避免不必要的副作用
}

// ============================================================
// #[tokio::main] 是过程宏，把 async main 包装进 tokio runtime
// TS: 顶层 await 是内置支持的，不需要额外 runtime
// ============================================================
#[tokio::main]
async fn main() {
    // 基本 async/await
    println!("=== 基本 async/await ===");
    let msg = greet("Rust").await;
    println!("{msg}");

    // 错误处理
    println!("\n=== 错误处理 ===");
    match get_user_greeting(1).await {
        Ok(msg)  => println!("{msg}"),
        Err(e)   => println!("错误: {e}"),
    }

    match get_user_greeting(99).await {
        Ok(msg)  => println!("{msg}"),
        Err(e)   => println!("错误: {e}"),
    }

    // 各种并发模式
    concurrent_demo().await;
    try_join_demo().await;
    spawn_demo().await;
    timeout_demo().await;
    lazy_demo().await;

    println!("\n=== TS vs Rust async 核心区别 ===");
    println!("1. Future 是惰性的（TS Promise 立即执行）");
    println!("2. 需要显式 runtime（#[tokio::main]）");
    println!("3. .await 只能在 async fn 内使用（TS 顶层也可以 await）");
    println!("4. tokio::join! = Promise.all");
    println!("5. tokio::spawn = 不等待的 Promise（fire-and-forget 需要 spawn）");
    println!("6. tokio::time::timeout = Promise.race([..., timeout])");
}
