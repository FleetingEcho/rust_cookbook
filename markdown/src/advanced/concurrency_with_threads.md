# Rust 多线程编程

## 1. 多线程编程的风险

由于多个线程可以并行执行，它们之间的执行顺序是不可预测的。这可能会导致以下问题：

- **竞态条件（Race Condition）**：多个线程以不确定的顺序访问共享数据，导致不一致的行为。
- **死锁（Deadlock）**：两个线程都在等待对方释放某个资源，导致无限等待。
- **难以复现的 Bug**：由于调度的不确定性，多线程程序可能会出现一些难以调试的问题。

Rust 通过所有权、借用检查和 `Send`/`Sync` trait 机制减少了这些问题的发生，但仍然需要开发者在设计时小心谨慎。

## 2. 创建线程

在 Rust 中，可以使用 `thread::spawn` 创建新的线程：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

**要点**

- 使用 `thread::spawn` 创建线程，并在闭包内执行代码。
- `thread::sleep(Duration::from_millis(1))` 让线程短暂休眠，以便其他线程有机会执行。
- 线程调度是由操作系统控制的，因此线程执行顺序是不确定的。

**可能的输出（每次运行可能不同）：**

```
Hi number 1 from the main thread!
Hi number 1 from the spawned thread!
Hi number 2 from the main thread!
Hi number 2 from the spawned thread!
Hi number 3 from the spawned thread!
Hi number 3 from the main thread!
...
```

> 注意：主线程一旦结束，程序就会立即退出，可能导致子线程提前终止，甚至子线程还没来得及执行就被强行中断。

## 3. 等待子线程结束

Rust 提供了 `join()` 方法来等待子线程执行完毕：

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap(); // 等待子线程执行完毕

    for i in 1..5 {
        println!("Hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
```

**输出：**

```
Hi number 1 from the spawned thread!
Hi number 2 from the spawned thread!
Hi number 3 from the spawned thread!
Hi number 4 from the spawned thread!
Hi number 1 from the main thread!
Hi number 2 from the main thread!
Hi number 3 from the main thread!
Hi number 4 from the main thread!
```

由于 `join()` 会阻塞主线程，直到子线程执行完毕，所以子线程先执行完，主线程再开始执行。

## 4. 使用 move 关键字在线程间传递数据

默认情况下，Rust 不允许子线程访问主线程的变量，因为可能会导致数据竞争。要解决这个问题，可以使用 `move` 关键字转移变量的所有权：

### 错误示例

```rust
// use std::thread;
// fn main() {
//     let v = vec![1, 2, 3];
//     let handle = thread::spawn(|| {
//         println!("Here's a vector: {:?}", v); // ❌ 这里会报错
//     });
//     handle.join().unwrap();
// }
```

**错误信息：**

```
error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
```

### 正确示例

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || { // v 的所有权被转移到子线程中
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```

使用 `move` 关键字后，`v` 的所有权被转移到子线程中，保证了数据的安全性。

## 5. 线程结束机制

Rust 采用**自动管理**线程的方式，即：

- 如果主线程结束，所有子线程也会被终止。
- 线程的生命周期取决于它的代码是否执行完毕。

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let new_thread = thread::spawn(move || {
        thread::spawn(move || {
            loop {
                println!("I am a new thread.");
            }
        });
    });

    new_thread.join().unwrap();
    println!("Child thread is finished!");

    thread::sleep(Duration::from_millis(100));
}
```

**分析**

- `new_thread` 结束后，main 线程等待 100ms 结束。
- `new_thread` 创建的子线程（B）仍然在无限循环，导致 CPU 被 100% 占用。

## 6. 线程屏障（Barrier）

`Barrier` 让所有线程都运行到某个点后，再一起继续执行。

```rust
use std::sync::{Arc, Barrier};
use std::thread;

fn main() {
    let barrier = Arc::new(Barrier::new(6));
    let mut handles = Vec::new();

    for _ in 0..6 {
        let b = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            println!("before wait");
            b.wait();
            println!("after wait");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
```

**输出**

```
before wait
before wait
before wait
before wait
before wait
before wait
after wait
after wait
after wait
after wait
after wait
after wait
```

所有线程都在 `b.wait()` 处等待，直到所有线程都达到屏障后才会继续执行。

## 7. 线程局部变量

线程局部变量（TLS）让每个线程都拥有自己的变量副本：

```rust
use std::cell::RefCell;
use std::thread;

thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

fn main() {
    FOO.with(|f| {
        assert_eq!(*f.borrow(), 1);
        *f.borrow_mut() = 2;
    });

    let t = thread::spawn(|| {
        FOO.with(|f| {
            assert_eq!(*f.borrow(), 1); // 子线程有独立的副本
            *f.borrow_mut() = 3;
        });
    });

    t.join().unwrap();

    FOO.with(|f| {
        assert_eq!(*f.borrow(), 2); // 主线程仍然是 2
    });
}
```

## 8. 只被调用一次的函数

`Once` 允许某个函数在多线程环境下只执行一次：

```rust
use std::sync::Once;
use std::thread;

static INIT: Once = Once::new();
static mut VAL: usize = 0;

fn main() {
    let handle1 = thread::spawn(|| {
        INIT.call_once(|| unsafe { VAL = 1 });
    });

    let handle2 = thread::spawn(|| {
        INIT.call_once(|| unsafe { VAL = 2 });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("{}", unsafe { VAL }); // 可能是 1 或 2
}
```

## 总结

- Rust 通过 `thread::spawn` 创建线程，默认情况下线程之间无序执行。
- `join()` 可以等待子线程结束，确保它的任务完成。
- `move` 关键字用于在线程间转移变量所有权，避免数据竞争。
- 线程屏障（`Barrier`）让多个线程同步执行。
- 线程局部变量（TLS）使每个线程持有独立的数据副本。
- `Once` 确保某个函数在多线程环境下只执行一次。
- Rust 提供了强大的多线程支持，同时保证了安全性，使得并发编程更加可靠！🚀

---

## 📘 TypeScript 对比

Rust 线程 ≈ Web Worker / Worker Threads，但更强大

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 线程模型 | 系统线程（1:1） | 单线程事件循环 + Worker |
| 共享内存 | `Arc<Mutex<T>>` | `SharedArrayBuffer`（受限） |
| 消息传递 | `std::sync::mpsc` | `postMessage` / 结构化克隆 |
| 数据竞争 | 编译期防止（Send/Sync） | 运行时可能发生 |
| 内存安全 | ✅ 编译期保证 | ❌ 不保证 |

> ⚠️ Rust 的 `Send`/`Sync` 是编译期 trait：
>
> - `Send`: 类型可以安全地在线程间转移所有权
> - `Sync`: 类型可以安全地在线程间共享引用
> - 编译器自动检查，违反规则直接报错
> - TS/JS 没有这种检查

详细对照 → [rust_vs_typescript.rs §19 "并发"](../rust_vs_typescript.rs)
