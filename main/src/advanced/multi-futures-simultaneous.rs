// Rust 并发：join! 和 select! 的使用
// 在 Rust 的异步编程中，await 只能等待单个 Future，而 join! 和 select! 提供了更强的并发能力：

// join!：并发运行多个 Future，等待所有任务完成。
// select!：并发运行多个 Future，等待其中一个任务完成，并立即处理结果。
// 1. join!：并发运行多个 Future
// 1.1 传统 .await 的局限

async fn enjoy_book_and_music() -> (Book, Music) {
    let book = enjoy_book().await;
    let music = enjoy_music().await;
    (book, music)
}
// 问题：必须 先看完书 再 听音乐，两者是串行执行的。

// 1.2 join! 让多个 Future 同时执行

use futures::join;
async fn enjoy_book_and_music() -> (Book, Music) {
    let book_fut = enjoy_book();
    let music_fut = enjoy_music();
    join!(book_fut, music_fut) // ✅ 书和音乐同时进行
}
// join! 同时运行 book_fut 和 music_fut，避免 等待
// 返回的是 元组，每个 Future 的结果按顺序存入元组。


// 1.3 join_all 处理多个 Future
// 如果需要同时运行多个任务（如数组中的 Future），可以使用 join_all：


use futures::future::join_all;

async fn run_tasks() {
    let futures = vec![task1(), task2(), task3()];
    let results = join_all(futures).await;
    println!("所有任务完成: {:?}", results);
}
// 适用于多个数量不固定的 Future。


// 2. try_join!：出错即终止
// 2.1 try_join!：遇到错误立即返回
// 如果希望 任意 Future 失败就终止执行，可使用 try_join!：


use futures::try_join;

async fn get_book() -> Result<Book, String> { /* ... */ Ok(Book) }
async fn get_music() -> Result<Music, String> { /* ... */ Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    try_join!(get_book(), get_music()) // ✅ 任意 `Future` 出错就返回
}

// join! 必须等所有 Future 完成。
// try_join! 遇到 Err 立刻返回。


// 2.2 try_join! 处理不同的错误类型
// 如果 Future 的错误类型不同，需要统一错误类型：


use futures::future::TryFutureExt;
use futures::try_join;

async fn get_book() -> Result<Book, ()> { Ok(Book) }
async fn get_music() -> Result<Music, String> { Ok(Music) }

async fn get_book_and_music() -> Result<(Book, Music), String> {
    let book_fut = get_book().map_err(|_| "无法获取书籍".to_string());
    let music_fut = get_music();
    try_join!(book_fut, music_fut) // ✅ 统一错误类型
}
// 3. select!：并发运行多个 Future，处理最先完成的
// 3.1 select! 让最快完成的任务先处理

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
// 特点：

// select! 并发运行 t1 和 t2，第一个完成的 Future 会被优先处理。
// 不会等待所有任务，一个任务完成后，立即执行对应分支。



// 3.2 select! 的 default 和 complete

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


// complete：所有 Future 完成后执行。
// default：没有 Future 就绪时执行（这里不会触发）。


// 4. select! 的底层机制
// 4.1 .fuse() 和 pin_mut!

// let t1 = task_one().fuse();
// let t2 = task_two().fuse();
// pin_mut!(t1, t2);
// .fuse()：让 Future 实现 FusedFuture 特征，防止完成的 Future 继续被 poll。
// pin_mut!：让 Future 实现 Unpin，使 select! 能安全地多次访问 Future。


// 4.2 FusedFuture 和 FusedStream

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
// 总结

// FusedFuture：防止 Future 完成后仍被 poll。
// FusedStream：防止 Stream 完成后仍被 next()。



// 5. Fuse::terminated() 让 Future 变为空
// 用于：在 select! 内部 动态创建 Future。


use futures::{
    future::{Fuse, FutureExt},
    pin_mut,
    select,
};

async fn get_new_num() -> u8 { 5 }

async fn run_loop() {
    let get_new_num_fut = Fuse::terminated(); // 创建一个空的 Future
    pin_mut!(get_new_num_fut);

    loop {
        select! {
            new_num = get_new_num_fut => {
                println!("收到新数字: {}", new_num);
                get_new_num_fut.set(get_new_num().fuse()); // ✅ 重新设置 Future
            },
            complete => break,
        }
    }
}
// 作用

// Fuse::terminated() 初始化为空 Future。
// 之后 get_new_num_fut.set() 动态填充 Future，形成循环。



// 6. FuturesUnordered 并发处理多个 Future
// 适用于：多个 Future 需要同时运行，不终止旧任务。


use futures::{
    stream::{FuturesUnordered, StreamExt},
    select,
};

async fn get_new_num() -> u8 { 5 }

async fn run_on_new_num(_: u8) -> u8 { 5 }

async fn run_loop() {
    let mut futures = FuturesUnordered::new();
    futures.push(run_on_new_num(10)); // 初始化时先启动一个任务

    loop {
        select! {
            new_num = get_new_num() => {  // 获取新的数字
                futures.push(run_on_new_num(new_num)); // ✅ 并发运行新的 Future
            },
            res = futures.select_next_some() => { // 获取执行完成的任务结果
                println!("完成: {:?}", res);
            },
            complete => break, // 如果所有 Future 结束，退出循环
        }
    }
}

// ================================


use futures::{
    future::{Fuse, FusedFuture, FutureExt},
    stream::{FusedStream, FuturesUnordered, Stream, StreamExt},
    pin_mut,
    select,
};

async fn get_new_num() -> u8 { /* ... */ 5 }

async fn run_on_new_num(_: u8) -> u8 { /* ... */ 5 }


// 使用从 `get_new_num` 获取的最新数字 来运行 `run_on_new_num`
//
// 每当计时器结束后，`get_new_num` 就会运行一次，它会立即取消当前正在运行的`run_on_new_num` ,
// 并且使用新返回的值来替换
async fn run_loop(
    mut interval_timer: impl Stream<Item = ()> + FusedStream + Unpin,
    starting_num: u8,
) {
    let mut run_on_new_num_futs = FuturesUnordered::new();
    run_on_new_num_futs.push(run_on_new_num(starting_num));
    let get_new_num_fut = Fuse::terminated();//初始时是 终止状态 (terminated())，避免 select! 误触发。
    pin_mut!(get_new_num_fut);
    loop {
        select! {
            () = interval_timer.select_next_some() => {
                 // 定时器已结束，若 `get_new_num_fut` 没有在运行，就创建一个新的
                if get_new_num_fut.is_terminated() {
                    get_new_num_fut.set(get_new_num().fuse());//如果 get_new_num_fut 处于终止状态，就重新启动 get_new_num 任务。
                }
            },
            new_num = get_new_num_fut => {
                 // 收到新的数字 -- 创建一个新的 `run_on_new_num_fut` (并没有像之前的例子那样丢弃掉旧值)
                run_on_new_num_futs.push(run_on_new_num(new_num));
            },
            // 运行 `run_on_new_num_futs`, 并检查是否有已经完成的
            res = run_on_new_num_futs.select_next_some() => {
                println!("run_on_new_num_fut returned {:?}", res);
            },
            // 若所有任务都完成，直接 `panic`， 原因是 `interval_timer` 应该连续不断的产生值，而不是结束
            //后，执行到 `complete` 分支
            complete => panic!("`interval_timer` completed unexpectedly"),
        }
    }
}

/*
FuturesUnordered：

保证所有 run_on_new_num 任务都能完成（不会被 select! 丢弃）。
并行执行 run_on_new_num 任务，提高性能。


.fuse() 将 Future 或 Stream 转换为 FusedFuture 或 FusedStream，使其在完成后保持“终止”状态，避免 select! 误触发。这样可以用 is_terminated() 检测其是否已完成，以决定是否重新启动。

*/

// 区别

// FuturesUnordered 不会取消旧任务，而是让 多个 Future 并发运行。
// 总结


// 宏	用途	适用场景
// join!	等待所有 Future 完成	需要并发运行多个任务
// try_join!	遇到错误立刻返回	某个任务失败就终止
// select!	第一个完成的 Future 先执行	任务竞速（如超时处理）
// FuturesUnordered	多个 Future 并行运行	并发处理多个任务
// 这样，Rust 异步编程就能更加高效、安全地执行多个 Future 任务！ 🚀


// Rust async 常见问题及解决方案
// Rust 的 async 生态仍在发展，许多问题短时间内无法解决，因此本文总结了一些典型的疑难杂症及其临时解决方案。

// 1. 在 async 语句块中使用 ?
// 问题
// async 语句块不同于 async fn，它无法显式声明返回值。当在 async 语句块中使用 ? 操作符时，可能会遇到类型推导错误：


// async fn foo() -> Result<u8, String> {
//     Ok(1)
// }

// async fn bar() -> Result<u8, String> {
//     Ok(1)
// }

// pub fn main() {
//     let fut = async {
//         foo().await?;
//         bar().await?;
//         Ok(())
//     };
// }
// 编译错误：


// error[E0282]: type annotations needed
//   --> src/main.rs:14:9
//    |
// 11 |     let fut = async {
//    |         --- consider giving `fut` a type
// ...
// 14 |         Ok(1)
//    |         ^^ cannot infer type for type parameter `E` declared on the enum `Result`
// 编译器无法推断 Result<T, E> 中的 E 的类型。

// 解决方案
// 手动为 Result 指定返回类型：


// let fut = async {
//     foo().await?;
//     bar().await?;
//     Ok::<(), String>(()) // 显式指定类型
// };
// 这样编译器就能正确推断 E 的类型为 String，通过编译。

// 2. async fn 和 Send 特征
// 问题
// Rust 的 async 任务可能会跨线程运行，而 Send 特征决定了数据是否可以在线程间安全传递。

// 如果 async fn 内部包含 Rc<T> 之类的非 Send 类型变量，就会导致编译错误。例如：


use std::rc::Rc;

#[derive(Default)]
struct NotSend(Rc<()>);

async fn bar() {}

async fn foo() {
    let x = NotSend::default();
    bar().await;
}

fn require_send(_: impl Send) {}

fn main() {
    require_send(foo());
}
// 错误信息：
// error: future cannot be sent between threads safely
// 由于 foo() 可能在 .await 之后被调度到另一个线程，而 Rc<()> 不是 Send，导致编译失败。

// 解决方案
// 1. 提前释放非 Send 变量


async fn foo() {
    {
        let x = NotSend::default(); // x 仅存在于该作用域
    }
    bar().await;
}
// 通过作用域 {} 提前释放 x，确保 .await 时 x 不存在，规避 Send 检查。

// 2. 替换 Rc 为 Arc


use std::sync::Arc;

#[derive(Default)]
struct SendType(Arc<()>);
// Arc<T> 是 Send，可以在 async fn 中安全使用。

// 3. 递归调用 async fn
// 问题
// async fn 本质上会被编译成一个状态机，递归调用时，编译器会尝试创建一个无限递归的类型，导致错误：


// async fn recursive() {
//     recursive().await;
//     recursive().await;
// }
// 错误信息：


// error[E0733]: recursion in an `async fn` requires boxing
// 原因是 async fn 无法直接递归调用自己，因为它的返回类型 Future 需要包含自身，导致编译器无法确定其大小。

// 解决方案
// 使用 BoxFuture 让 Future 存储在堆上：


use futures::future::{BoxFuture, FutureExt};

fn recursive() -> BoxFuture<'static, ()> {
    async move {
        recursive().await;
        recursive().await;
    }
    .boxed()
}
// 这样 Future 的大小就固定了，编译器不会再报错。

// 4. 在 trait 中使用 async fn
// 问题
// Rust 目前不支持在 trait 中直接使用 async fn：


// trait Test {
//     async fn test();
// }
// 错误信息：

// error[E0706]: functions in traits cannot be declared `async`
// 解决方案
// 使用 async-trait 宏：


use async_trait::async_trait;

#[async_trait]
trait Advertisement {
    async fn run(&self);
}

struct Modal;

#[async_trait]
impl Advertisement for Modal {
    async fn run(&self) {
        self.render_fullscreen().await;
        for _ in 0..4 {
            remind_user_to_join_mailing_list().await;
        }
        self.hide_for_now().await;
    }
}
// 注意： async-trait 需要动态分配堆内存 (Box)，大量调用时可能会影响性能。

// 总结
// 问题	错误原因	解决方案
// async 语句块中 ? 不能推断类型	编译器无法推断 Result<E> 的 E	显式提供 Ok::<(), String>(())
// async fn 中 Send 变量问题	.await 可能被调度到另一个线程，Rc<T> 不是 Send	1. 提前释放变量 {} 2. 使用 Arc<T>
// async fn 递归调用报错	async fn 返回 Future，其大小不固定	用 BoxFuture 包装 Future
// trait 不支持 async fn	trait 不能直接定义 async fn	用 async-trait 宏
// Rust async 仍在发展，但这些方案可以帮助解决大部分常见问题。未来随着语言特性的完善，许多临时解决方案可能会变得不再必要。