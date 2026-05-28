# Rust async/.await 简单入门

## 1. async/.await 简介

Rust 内置的 `async`/`.await` 语法使得编写异步代码变得像同步代码一样直观。

`async` 关键字标记的代码块会被转换成实现 `Future` 特征的状态机。`Future` 在执行时不会阻塞线程，而是让出线程的控制权，以便其他 `Future` 任务可以继续执行，实现高效并发。

## 2. 引入 futures 依赖

在 `Cargo.toml` 中添加：

```toml
[dependencies]
futures = "0.3"
```

## 3. async 关键字

异步函数的返回值是 `Future`，直接调用不会执行：

```rust
async fn do_something() {
    println!("go go go !");
}

fn main() {
    do_something(); // 不会输出任何内容
}
```

编译器会警告 `Future` 未被执行，因此需要使用**执行器 (executor)** 来运行 `Future`。

## 4. block_on 执行 Future

使用 `futures::executor::block_on`：

```rust
use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

fn main() {
    let future = hello_world(); // 返回一个 Future
    block_on(future);           // 执行 Future 并等待其完成
}
```

**运行结果：**

```
hello, world!
```

## 5. await 关键字

`.await` 用于在 `async` 函数中等待 `Future` 任务完成：

```rust
use futures::executor::block_on;

async fn hello_world() {
    hello_cat().await;           // 等待 hello_cat 完成
    println!("hello, world!");
}

async fn hello_cat() {
    println!("hello, kitty!");
}

fn main() {
    block_on(hello_world());
}
```

**运行结果：**

```
hello, kitty!
hello, world!
```

> `.await` 不会阻塞线程，而是异步等待 `Future` 任务完成，同时让出线程的执行权。

## 6. async/.await 并发执行

假设有 **学歌**、**唱歌** 和 **跳舞** 任务：

### 6.1 不使用 `.await`，同步执行

```rust
use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "周杰伦".to_string(),
        name: String::from("《菊花台》"),
    }
}

async fn sing_song(song: Song) {
    println!("给大家献上一首{}的{} ~ {}", song.author, song.name, "菊花残，满地伤~ ~");
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

fn main() {
    let song = block_on(learn_song());
    block_on(sing_song(song));
    block_on(dance());
}
```

此代码**依次执行**学歌、唱歌、跳舞，每个任务执行时都会阻塞线程。

### 6.2 使用 `.await` 并发执行

```rust
use futures::executor::block_on;

struct Song {
    author: String,
    name: String,
}

async fn learn_song() -> Song {
    Song {
        author: "曲婉婷".to_string(),
        name: String::from("《我的歌声里》"),
    }
}

async fn sing_song(song: Song) {
    println!("给大家献上一首{}的{} ~ {}", song.author, song.name, "你存在我深深的脑海里~ ~");
}

async fn dance() {
    println!("唱到情深处，身体不由自主的动了起来~ ~");
}

async fn learn_and_sing() {
    let song = learn_song().await;   // 先学歌
    sing_song(song).await;           // 再唱歌
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` 让多个 Future 并发执行
    futures::join!(f1, f2);
}

fn main() {
    block_on(async_main());
}
```

**运行结果：**

```
唱到情深处，身体不由自主的动了起来~ ~
给大家献上一首曲婉婷的《我的歌声里》 ~ 你存在我深深的脑海里~ ~
```

**分析：**

- `join!` 让 `learn_and_sing` 和 `dance` 并发执行。
- `learn_and_sing` 内部 `.await` 了 `learn_song()`，但线程仍然可用于 `dance()`。
- 学歌和跳舞可以同时进行，不会相互阻塞，提高了程序的并发性。

## 7. 结论

- `async`/`.await` 允许在 Rust 中编写异步代码，同时保持同步代码的可读性。
- `.await` 不会阻塞线程，而是异步等待 `Future` 任务完成，实现**并发执行**。
- `futures::join!` 可用于同时执行多个 `Future` 任务，提高效率。
- 学习 `async`/`.await` 后，建议继续深入了解 `Future` 的底层实现及执行机制，以更好地理解 Rust 异步编程的核心原理。

---

## 📘 TypeScript 对比

Rust `async/await` 和 TypeScript 非常相似！

**Rust：**

```rust
async fn fetch() -> String { some_fn().await }
```

**TypeScript：**

```ts
async function fetch(): Promise<string> { return await someFn(); }
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 返回值 | `Future` trait | `Promise<T>` |
| 运行时 | 需显式选（tokio/async-std） | 内置事件循环 |
| 执行 | 惰性——不 poll 就不执行 | Promise 创建即执行 |
| 并发 | `join!` / `tokio::spawn` | `Promise.all` |
| 取消 | drop Future | AbortController |

> ⚠️ **最大区别：Rust `Future` 是惰性的。**
>
> 你写 `async fn` 得到的 `Future` 不会执行，直到你 `.await` 它或传给执行器。
>
> TS 的 `async` 函数一调用 `Promise` 就开始执行。

详细对照 → [rust_vs_typescript.rs §20 "async/await"](../rust_vs_typescript.rs)
