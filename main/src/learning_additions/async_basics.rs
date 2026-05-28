// async/await 用于表达“现在可能等一等，之后继续执行”的逻辑。
// 这个文件只放标准库可编译的基础例子；真正的网络请求例子在 main/projects/hacker_news。

pub async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

pub async fn async_pipeline(value: i32) -> i32 {
    let doubled = async_add(value, value).await;
    doubled + 1
}

pub fn explain_async_terms() -> &'static str {
    "async fn 返回 Future；.await 会等待 Future 完成；运行 Future 需要 tokio、async-std 等运行时。"
}

// 📘 TypeScript 对比
// ====================
// Rust async 和 TS async 语法几乎一样！
//
// ```rust
// async fn add(a: i32, b: i32) -> i32 { a + b }
// let result = add(1, 2).await;
// ```
// ```ts
// async function add(a: number, b: number): Promise<number> { return a + b; }
// let result = await add(1, 2);
// ```
//
// | 区别 | Rust | TypeScript |
// |------|------|-----------|
// | Future 是否惰性 | ✅ 不 await 就不执行 | ❌ Promise 创建即执行 |
// | 运行时 | 需 tokio 等 | 内置事件循环 |
// | 并发 | `join!` / `tokio::spawn` | `Promise.all` |
//
// 详细对照 → rust_vs_typescript.rs §20
