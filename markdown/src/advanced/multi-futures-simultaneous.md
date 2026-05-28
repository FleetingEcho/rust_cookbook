# Rust 并发：join! 和 select! 的使用

在 Rust 的异步编程中，`.await` 只能等待单个 `Future`，而 `join!` 和 `select!` 提供了更强的并发能力：

- **join!**：并发运行多个 Future，等待所有任务完成。
- **select!**：并发运行多个 Future，等待其中一个任务完成，并立即处理结果。

## 1. join!：并发运行多个 Future

### 1.1 传统 .await 的局限

```rust
async fn enjoy_book_and_music() -> (Book, Music) {
    let book = enjoy_book().await;
    let music = enjoy_music().await;
    (book, music)
}
```

**问题：** 必须**先看完书**再**听音乐**，两者是串行执行的。

### 1.2 join! 让多个 Future 同时执行

```rust
use futures::join;
async fn enjoy_book_and_music() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    join!(book_fut, music_fut) // ✅ 书和音乐同时进行
}
```

- `join!` 同时运行 `book_fut` 和 `music_fut`，避免等待
- 返回的是元组，每个 Future 的结果按顺序存入元组

### 1.3 join_all 处理多个 Future

如果需要同时运行多个任务（如数组中的 Future），可以使用 `join_all`：

```rust
use futures::future::join_all;

async fn run_tasks() {
    let futures = vec![task1(), task2(), task3()];
    let results = join_all(futures).await;
    println!("所有任务完成: {:?}", results);
}
```

适用于多个数量不固定的 Future。

## 2. try_join!：出错即终止

### 2.1 try_join!：遇到错误立即返回

如果希望任意 Future 失败就终止执行，可使用 `try_join!`：

```rust
use futures::try_join;

async fn get_book() -> Result<Book, String> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    try_join!(get_book(), get_music()) // ✅ 任意 `Future` 出错就返回
}
```

- `join!` 必须等所有 Future 完成。
- `try_join!` 遇到 `Err` 立刻返回。

### 2.2 try_join! 处理不同的错误类型

如果 Future 的错误类型不同，需要统一错误类型：

```rust
use futures::future::TryFutureExt;
use futures::try_join;

async fn get_book() -> Result<Book, ()> { Ok(Book) }
async fn get_music() -> Result<Music, String> { Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|_| "无法获取书籍".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut) // ✅ 统一错误类型
}
```

## 3. select!：并发运行多个 Future，处理最先完成的

### 3.1 select! 让最快完成的任务先处理

```rust
use futures::{future::FutureExt, pin_mut, select};

async fn task_one() { /* ... */ }
async fn task_two() { /* ... */ }

async fn race_tasks() {
    let t1 = task_one().fuse();
    let t2 = task_two().fuse();
    pin_mut!(t1, t2);

    select! {
        () = t1 => println!("任务1率先完成"),
        () = t2 => println!("任务2率先完成"),
    }
}
```

**特点：**

- `select!` 并发运行 `t1` 和 `t2`，第一个完成的 Future 会被优先处理。
- 不会等待所有任务，一个任务完成后，立即执行对应分支。

### 3.2 select! 的 default 和 complete

```rust
use futures::{future, select};

fn main() {
    let mut a_fut = future::ready(4);
    let mut b_fut = future::ready(6);
    let mut total = 0;

    loop {
        select! {
            a = a_fut => total += a,
            b = b_fut => total += b,
            complete => break, // ✅ 所有 `Future` 完成后，结束循环
            default => panic!(), // ❌ 这里不会执行
        };
    }
    assert_eq!(total, 10);
}
```

- `complete`：所有 Future 完成后执行。
- `default`：没有 Future 就绪时执行（这里不会触发）。

## 4. select! 的底层机制

### 4.1 .fuse() 和 pin_mut!

```rust
let t1 = task_one().fuse();
let t2 = task_two().fuse();
pin_mut!(t1, t2);
```

- `.fuse()`：让 Future 实现 `FusedFuture` 特征，防止完成的 Future 继续被 poll。
- `pin_mut!`：让 Future 实现 `Unpin`，使 `select!` 能安全地多次访问 Future。

### 4.2 FusedFuture 和 FusedStream

```rust
use futures::{
    stream::{Stream, StreamExt, FusedStream},
    select,
};

async fn add_two_streams(
    mut s1: impl Stream<Item = u8> + FusedStream + Unpin,
    mut s2: impl Stream<Item = u8> + FusedStream + Unpin,
) -> u8 {
    let mut total = 0;

    loop {
        let item = select! {
            x = s1.next() => x,
            x = s2.next() => x,
            complete => break,
        };
        if let Some(next_num) = item {
            total += next_num;
        }
    }

    total
}
```

## 总结

- `join!` — 并发运行多个 Future，等待全部完成
- `try_join!` — 并发运行，遇到错误立即返回
- `select!` — 并发运行，最先完成的优先处理
- `.fuse()` + `pin_mut!` — `select!` 的底层要求
