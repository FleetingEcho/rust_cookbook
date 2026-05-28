// Rust 多线程消息传递详解
// Rust 提供了 多线程间的安全数据共享，其中 消息传递（Message Passing） 是最常用的方法之一。Rust 通过 mpsc（multiple producer, single consumer）通道 实现了 多发送者、单接收者 的消息通信，同时还支持 同步通道和异步通道。

// 本节将深入探讨 Rust 中的 多线程消息传递机制，包括 基础用法、所有权传递、同步 vs. 异步通道、多发送者、通道关闭 及 第三方库优化。

// 1. 多线程间的消息传递概念
// 在 Rust 中，线程间的通信方式有两种：

// 共享状态 + Mutex<Arc<T>>（下一节讲解）
// 消息传递（Channel） ✅（本节重点）
// Rust 标准库提供的 mpsc（Multiple Producer, Single Consumer）即 多发送者、单接收者，类似 Go 语言的 chan：


// Do not communicate by sharing memory; instead, share memory by communicating
// （不要通过共享内存进行通信，而应通过通信共享内存。）

// 2. mpsc 通道基础用法
// 在 Rust 中，std::sync::mpsc 提供了 channel() 方法来创建一个通道，返回 (发送者 Sender, 接收者 Receiver)。

// 2.1 单发送者 & 单接收者

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel(); // 创建通道

    thread::spawn(move || {
        tx.send(1).unwrap(); // 发送数据
    });

    println!("Received: {}", rx.recv().unwrap()); // 接收数据
}
// 🔍 解析
// mpsc::channel() 创建通道，返回 (tx, rx)（Sender 和 Receiver）。
// tx.send(1).unwrap(); 发送数据（可能失败）。
// rx.recv().unwrap(); 接收数据（可能失败）。
// ⚠️ 注意

// rx.recv() 是阻塞的，如果没有消息，它会 阻塞线程，直到收到消息。
// tx.send() 返回 Result<T, E>，如果 rx 被 drop，则 send() 会返回错误。
// 📌 输出
// Received: 1



// 3. try_recv()：非阻塞消息接收
// 使用 try_recv() 尝试接收消息，不会阻塞。


use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send(1).unwrap();
    });

    println!("Received: {:?}", rx.try_recv()); // 可能会返回 `Err(Empty)`
}

// 🔍 解析
// try_recv() 不会阻塞，如果 当前没有消息，它会返回 Err(Empty)。
// 运行结果 不确定（取决于线程调度）：
// Received: Err(Empty)
// 或
// Received: Ok(1)

// 4. 传递具有所有权的数据
// 消息通道会 转移数据所有权：

// 如果数据类型实现 Copy（如 i32），则 复制数据 进行传输。
// 如果数据类型未实现 Copy（如 String），则 所有权会被移动。

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let s = String::from("我，飞走咯!");
        tx.send(s).unwrap();
        // println!("val is {}", s); // ❌ `s` 已被转移，不能再使用
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}
// 📌 解析

// 由于 String 没有 Copy 特征，发送 s 后，它的所有权 被移动到接收端。
// println!("val is {}", s); 会报错，因为 s 已 失去所有权。



// 5. for 循环接收多个消息
// 通道 Receiver 实现了 Iterator，可以直接 for 循环接收消息：


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["hi", "from", "the", "thread"];

        for val in vals {
            tx.send(val.to_string()).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
// rx 作为 Iterator，会 持续接收消息，直到 发送者关闭。
// for received in rx {} 运行后 自动结束，无须 recv()。



// 6. 多个发送者
// 多个线程共享 Sender，可通过 clone() 克隆发送者：


use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        tx.send("Hi from raw tx").unwrap();
    });

    thread::spawn(move || {
        tx1.send("Hi from cloned tx").unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
// tx.clone() 克隆发送者，让多个线程可以发送数据。
// for received in rx {} 直到所有 Sender 被 drop 才会结束。




// 7. sync_channel()：同步通道
// 同步通道 (mpsc::sync_channel()) 发送消息时 如果缓冲区已满，会阻塞发送者：


use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::sync_channel(1); // 设定缓冲大小为 1

    thread::spawn(move || {
        println!("发送之前");
        tx.send(1).unwrap();
        println!("发送之后"); // 只有 `rx.recv()` 之后才会输出
    });

    thread::sleep(Duration::from_secs(3));
    println!("睡眠之后");

    println!("Received: {}", rx.recv().unwrap());
}
// 📌 解析
// 同步通道 需要 缓冲区有空闲空间，否则 send() 会阻塞，直到有接收者消费消息。
// mpsc::sync_channel(1) 设置缓冲大小，可以 发送 1 条消息不阻塞。



// 8. 关闭通道
// 所有 Sender 被 drop，rx 会返回 Err(Disconnected)。
// 所有 Receiver 被 drop，tx.send() 会返回 Err(Disconnected)。




// 9. 传输多种类型的数据
// 可以使用 enum 传输不同数据类型：


use std::sync::mpsc::{self, Receiver, Sender};

enum Fruit {
    Apple(u8),
    Orange(String),
}

fn main() {
    let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

    tx.send(Fruit::Orange("sweet".to_string())).unwrap();
    tx.send(Fruit::Apple(2)).unwrap();

    for _ in 0..2 {
        match rx.recv().unwrap() {
            Fruit::Apple(count) => println!("received {} apples", count),
            Fruit::Orange(flavor) => println!("received {} oranges", flavor),
        }
    }
}



// 10. mpmc（多发送者多接收者）
// Rust 标准库 mpsc 只支持 多发送者单接收者，要实现 多发送者多接收者（mpmc），可以使用 crossbeam-channel 或 flume。

// 📌 crossbeam-channel 比 mpsc 更快、更灵活：


// use crossbeam_channel::unbounded;
// let (tx, rx) = unbounded(); // 多发送者多接收者
// ✅ Rust 提供了安全、高效的多线程通信机制，mpsc 适用于多发送者单接收者，crossbeam-channel 适用于多发送者多接收者！ 🚀