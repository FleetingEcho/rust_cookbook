// Rust 多线程同步：锁、条件变量和信号量
// Rust 提供了 消息传递 和 共享内存（锁机制） 两种主要的线程同步方式：

// 消息传递：基于 mpsc::channel() 进行数据传输，每个值只能由一个线程持有（单所有权）。
// 共享内存：多个线程可以同时访问和修改数据（多所有权），通过 锁 保证数据安全。
// 📌 何时使用哪种方式？

// 方式	适用场景
// 消息传递	需要可靠性、任务流水线、模拟现实世界（如事件通知）
// 共享内存（锁）	需要高性能、简洁实现、多线程同时访问共享资源



// 1. Mutex<T> 互斥锁
// 🔹 作用：让多个线程串行访问同一资源，防止竞态条件。

// 1.1 单线程中的 Mutex

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // 获取锁
        *num = 6; // 修改数据
    } // 锁自动释放

    println!("m = {:?}", m);
}
// 📌 解析

// m.lock().unwrap() 获取锁，返回 MutexGuard（类似智能指针）。
// MutexGuard 实现 Deref，可直接修改数据。
// MutexGuard 实现 Drop，作用域结束后自动释放锁，避免死锁。


// 1.2 多线程中的 Mutex
// 🚨 Rc<T> 不能用于多线程，需用 Arc<T> 代替。


use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// 📌 解析

// Arc<T>（原子引用计数）让多个线程共享 Mutex<T> 的所有权。
// counter.lock().unwrap() 确保每次只有一个线程修改 counter。
// 避免竞态条件，确保最终 counter == 10。



// 2. RwLock<T> 读写锁
// 🔹 作用：多个线程可同时 读取 数据，但 写入时互斥。


use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    // 允许多个读
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, 5);
        assert_eq!(*r2, 5);
    } // 读锁在此处被释放

    // 只能有一个写
    {
        let mut w = lock.write().unwrap();
        *w += 1;
    } // 写锁在此处被释放

    println!("lock = {:?}", lock);
}
// 📌 解析

// 多个线程可同时 read()，但 write() 会阻塞所有 read() 操作。
// try_read() 和 try_write() 不会阻塞，如果锁被占用则返回 Err(WouldBlock)。
// 📌 Mutex vs. RwLock

// 选择	适用场景
// Mutex	读写较均衡，锁机制简单
// RwLock	读多写少，提高并发性能



// 3. 避免死锁
// 🔹 死锁：两个线程都在等待对方释放资源，导致永久阻塞。


use std::sync::{Mutex};
use std::thread;

fn main() {
    let data = Mutex::new(0);
    let d1 = data.lock().unwrap();
    // let d2 = data.lock().unwrap(); // ❌ 死锁
    let d3 = data.try_lock().unwrap(); // OK
}
// 📌 解决方法

// try_lock() 代替 lock()，避免无限等待：

// if let Ok(guard) = data.try_lock() {
//     // 成功获取锁
// } else {
//     // 处理无法获取锁的情况
// }
// 固定锁的顺序，避免交叉持有多个锁。



// 4. Condvar 条件变量
// 🔹 作用：线程可以 挂起等待某个条件达成，然后被唤醒。


use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;

fn main() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = pair.clone();

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one(); // 唤醒等待的线程
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        started = cvar.wait(started).unwrap(); // 挂起等待
    }

    println!("Thread started!");
}
// 📌 解析

// wait() 释放锁并挂起线程，直到 notify_one() 唤醒。
// notify_one() 通知一个线程，notify_all() 通知所有线程。


// 5. Semaphore 信号量
// 🔹 作用：限制最多 N 个任务同时执行。


use std::sync::Arc;
use tokio::sync::Semaphore;

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(3));
    let mut join_handles = vec![];

    for _ in 0..5 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(async move {
            println!("执行任务...");
            drop(permit); // 释放信号量
        }));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
// 📌 解析

// Semaphore::new(3)：最多允许 3 个任务同时执行。
// acquire_owned().await 申请信号量，超过 3 时会等待。
// drop(permit) 释放信号量，让等待的任务继续执行。



// 总结
// 方式	作用	适用场景
// Mutex<T>	互斥锁	读写较均衡，单线程独占资源
// RwLock<T>	读写锁	读多写少，提高并发性能
// Condvar	条件变量	控制线程执行顺序
// Semaphore	信号量	限制并发任务数


// ✅ Rust 提供了丰富的多线程同步机制，合理使用锁、条件变量和信号量，可提高程序的安全性和并发性能！🚀