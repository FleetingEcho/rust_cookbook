# async/await 基础

## async/await 简介

`async/await` 用于表达"现在可能等一等，之后继续执行"的逻辑，是 Rust 处理 IO 密集型任务的标准方式。

### 与 TypeScript 的核心差异

| 差异 | Rust | TypeScript |
|------|------|-----------|
| Future 是否惰性 | ✅ 不 `.await` 就不执行 | ❌ Promise 创建即执行 |
| 运行时 | 需 tokio / async-std | 内置事件循环 |
| 并发 | `join!` / `spawn` | `Promise.all` |
| 取消 | drop Future 即取消 | `AbortController` |

---

## 一、async fn 与 .await

`async fn` 返回 `impl Future<Output = T>`，不是直接的值。
`.await` 把控制权交还给运行时，等 Future 就绪后继续执行。

```rust
pub async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

pub async fn async_pipeline(value: i32) -> i32 {
    let doubled = async_add(value, value).await;  // 暂停等待
    doubled + 1
}
```

**TS 对比：**
```ts
async function asyncAdd(left: number, right: number): Promise<number> {
    return left + right;
}
```

---

## 二、Future 是惰性的（Rust 独有特性）

```rust
async fn say_hello() {
    println!("Hello!");
}

// ❌ 这什么都不做！Future 创建了但从未执行
let future = say_hello();

// ✅ 必须 .await 才会执行
say_hello().await;
```

**TypeScript 相比：**
```ts
// TS 的 Promise 创建即开始执行！
const p = sayHello(); // 立刻执行，打印 "Hello!"
```

---

## 三、async 块

除了 `async fn`，还可以在函数内部用 `async { }` 块创建匿名 Future：

```rust
let future = async {
    let x = async_add(1, 2).await;
    x * 10
};

let result = future.await;  // 执行，得到 30
println!("{result}");
```

---

## 四、async 中的错误处理

`async fn` 可以返回 `Result`，`?` 运算符在 async 上下文中同样工作：

```rust
use std::io;

async fn read_config(path: &str) -> Result<String, io::Error> {
    // 假设这是异步读取（实际需要 tokio::fs::read_to_string）
    let content = std::fs::read_to_string(path)?;  // ? 照常用
    Ok(content.trim().to_string())
}

async fn load_and_greet(path: &str) -> Result<(), io::Error> {
    let name = read_config(path).await?;  // await 后接 ?
    println!("你好，{name}！");
    Ok(())
}
```

---

## 五、需要运行时：tokio

Rust 的 async 需要外部运行时来驱动 Future 执行，最常用的是 `tokio`：

```toml
# Cargo.toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
// #[tokio::main] 把 main 变成 async，并启动 tokio 运行时
#[tokio::main]
async fn main() {
    let result = async_pipeline(5).await;
    println!("结果: {result}");  // 11
}
```

**测试中使用 tokio：**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]   // 用 #[tokio::test] 代替 #[test]
    async fn test_async_add() {
        assert_eq!(async_add(2, 3).await, 5);
    }

    #[tokio::test]
    async fn test_pipeline() {
        assert_eq!(async_pipeline(4).await, 9);
    }
}
```

---

## 六、并发执行多个 Future（join!）

```rust
use tokio::time::{sleep, Duration};

async fn fetch_user(id: u32) -> String {
    sleep(Duration::from_millis(10)).await;
    format!("用户{id}")
}

async fn fetch_posts(user_id: u32) -> Vec<String> {
    sleep(Duration::from_millis(15)).await;
    vec![format!("用户{user_id}的文章")]
}

async fn concurrent_demo() {
    // 两个 Future 并发执行，总耗时约 15ms（不是 25ms）
    let (user, posts) = tokio::join!(
        fetch_user(1),
        fetch_posts(1),
    );
    println!("{user}: {:?}", posts);
}
```

**TS 对比：**
```ts
const [user, posts] = await Promise.all([fetchUser(1), fetchPosts(1)]);
```

---

## 七、spawn：后台任务

```rust
async fn background_demo() {
    let handle = tokio::spawn(async {
        sleep(Duration::from_millis(5)).await;
        "后台任务完成"
    });

    println!("主任务继续运行...");

    let result = handle.await.unwrap();
    println!("{result}");
}
```

`spawn` 类似 TS 的"不 await 的 Promise"，但 Rust 中你可以等待它的结果。

---

## 八、关键概念速查

```
async fn foo() -> T       返回 impl Future<Output = T>
foo().await               执行 Future，等待结果
async { expr }            创建匿名 Future
tokio::join!(f1, f2)      并发执行，全部完成后返回
tokio::spawn(async { })   后台任务，立即开始
tokio::select! { ... }    哪个先完成取哪个（类似 Promise.race）
```

深入内容 → [async_await.md](../../examples/rust_vs_typescript/async_await.md)（含 select!、timeout、惰性详解）
