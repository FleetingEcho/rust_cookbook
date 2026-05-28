// Rust 线程安全：Send 和 Sync
// 在 Rust 的并发模型中，Send 和 Sync 是确保数据 线程安全 的关键特征。理解它们，可以帮助我们避免数据竞争和未定义行为，同时还能优化 Rust 并发代码。

// 1. Send 和 Sync 是什么？
// Rust 通过 Send 和 Sync 这两个 标记（marker trait） 确保线程安全：

// Send：实现了 Send 的类型可以 安全地在线程间传递所有权。
// Sync：实现了 Sync 的类型可以 安全地在线程间共享（通过引用）。


// 📌 规则
// 实现 Send 的类型 可以被 move 到另一个线程中。
// 实现 Sync 的类型 允许多个线程共享同一个实例的引用（&T）。
// 如果 T: Sync，则 &T: Send，即 T 可被多个线程共享，&T 也可以被 Send。



// 2. Rc 不能跨线程，但 Arc 可以
// Rc 无法用于多线程，因为 Rc 内部的 引用计数（RefCell<T>） 不是线程安全的：


use std::thread;
use std::rc::Rc;

fn main() {
    let v = Rc::new(5);

    let t = thread::spawn(move || {
        println!("{}", v);// 🚨 报错
    });

    t.join().unwrap();
}


// error[E0277]: `Rc<i32>` cannot be sent between threads safely
// 📌 原因：

// Rc<T> 未实现 Send，不能安全地在线程间传递。
// Rc<T> 未实现 Sync，不能在多个线程间共享。
// ✅ Arc<T> 可以安全共享
// 使用 Arc<T>（原子引用计数） 代替 Rc<T>：


use std::sync::Arc;
use std::thread;

fn main() {
    let v = Arc::new(5);

    let t = thread::spawn(move || {
        println!("{}", v);
    });

    t.join().unwrap();
}
// 📌 Arc<T>（Atomic Reference Counted）

// 适用于 多线程，内部计数器是原子的（Atomic），保证线程安全。
// 性能比 Rc 低，但支持跨线程共享。



// 3. Send 和 Sync 的底层实现
// Rust 默认自动派生 Send 和 Sync，除非类型中包含 非线程安全 的成员，例如：

// 裸指针 *const T、*mut T
// RefCell<T>
// Rc<T>
// 📌 Rc<T> 和 Arc<T> 的 Send / Sync 实现对比


// // Rc<T> 源码
// impl<T: ?Sized> !Send for Rc<T> {} // Rc 不能在线程间传递
// impl<T: ?Sized> !Sync for Rc<T> {} // Rc 不能在线程间共享

// // Arc<T> 源码
// unsafe impl<T: ?Sized + Send + Sync> Send for Arc<T> {} // Arc 可以传递
// unsafe impl<T: ?Sized + Send + Sync> Sync for Arc<T> {} // Arc 可以共享
// 📌 Rc<T> 明确禁止 Send 和 Sync，而 Arc<T> 则需要 T 本身实现 Send + Sync。




// 4. Mutex<T> vs RwLock<T>：锁的 Send / Sync
// 在多线程环境中，我们需要确保共享数据的安全：


// // RwLock 允许多个读，但 T 必须是 Sync
// unsafe impl<T: ?Sized + Send + Sync> Sync for RwLock<T> {}

// // Mutex 允许单个线程访问，T 只需 Send
// unsafe impl<T: ?Sized + Send> Sync for Mutex<T> {}
// 📌 对比

// RwLock<T>：支持多个读，T 必须是 Sync。
// Mutex<T>：只允许一个线程访问，T 只需 Send。



// 5. 手动实现 Send 和 Sync
// 有时，我们需要手动实现 Send 和 Sync（不推荐，需谨慎使用 unsafe）。

// 🚨 问题：裸指针 *mut T 不能跨线程

// use std::thread;

// fn main() {
//     let p = 5 as *mut u8;

//     let t = thread::spawn(move || {
//         println!("{:?}", p);
//     });

//     t.join().unwrap();
// }
// ❌ 报错


// error[E0277]: `*mut u8` cannot be sent between threads safely
// 📌 解决方案：使用 newtype 结构体，并手动实现 Send：


// use std::thread;

// #[derive(Debug)]
// struct MyBox(*mut u8);

// unsafe impl Send for MyBox {} // 让 `MyBox` 可以跨线程传递

// fn main() {
//     let p = MyBox(5 as *mut u8);

//     let t = thread::spawn(move || {
//         println!("{:?}", p);
//     });

//     t.join().unwrap();
// }
// 📌 实现 Send 后，裸指针 *mut u8 可以安全传递。

// 6. Sync：裸指针的多线程共享
// Sync 允许 多个线程 共享 &T，但 *const T 默认不是 Sync：


use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct MyBox(*const u8);

unsafe impl Send for MyBox {} // `MyBox` 现在可以传递
unsafe impl Sync for MyBox {} // `MyBox` 现在可以共享

fn main() {
    let b = &MyBox(5 as *const u8);
    let v = Arc::new(Mutex::new(b));

    let t = thread::spawn(move || {
        let _v1 = v.lock().unwrap();
    });

    t.join().unwrap();
}
// 📌 解决方案

// 使用 Arc<Mutex<T>> 让 MyBox 线程安全
// 实现 Sync 让 MyBox 可在线程间共享


// 7. 总结
// 特性	作用	适用情况
// Send	允许类型在线程间移动（move 语义）	Arc<T>、Mutex<T>
// Sync	允许类型在多个线程共享（&T 可跨线程）	RwLock<T>、Atomic<T>
// ✅ Send 用于移动所有权，Sync 用于跨线程共享。
// ✅ 绝大多数类型默认实现 Send 和 Sync（除 Rc<T>、RefCell<T>、裸指针）。
// ✅ 可以使用 unsafe impl Send / Sync 让自定义类型跨线程传递。
// ✅ 多线程共享数据时，使用 Arc<T> + Mutex<T> 保护数据安全。

// 🚀 Rust 的 Send / Sync 机制，让多线程编程更安全可靠！







