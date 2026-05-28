# Rust 线程同步：Atomic 原子类型与内存顺序

在多线程编程中，`Mutex` 适用于串行访问，`RwLock` 适用于多读少写，但它们都涉及锁机制，可能带来性能开销。**Atomic（原子操作）**提供了一种无锁的线程安全机制，在高并发场景下表现优越。

> Atomic 无法替代 Mutex，因为 Atomic 只能操作基础类型（整数、布尔）。复杂数据结构（如 `Vec<T>`）仍需 Mutex 保护。

## 1. 什么是 Atomic（原子操作）？

**原子操作（Atomic Operation）**是指不可被 CPU 上下文切换打断的机器指令。

Rust 在 `std::sync::atomic` 模块中提供了一系列原子类型，例如：

- `AtomicU64`（无符号 64 位整数）
- `AtomicBool`（布尔值）
- `AtomicIsize`（有符号整数）

### 相比 Mutex，Atomic 的优势

- 无需手动加锁/解锁（减少锁竞争）
- 支持并发修改（如 `fetch_add` 实现自增）
- 性能更优（适用于无锁数据结构）

## 2. Atomic 基础用法

### AtomicU64 作为全局计数器

```rust
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Instant;

// 常量定义
const N_TIMES: u64 = 10_000_000; // 每个线程增加 1000 万次
const N_THREADS: usize = 10; // 10 个线程

// 使用 AtomicU64 作为全局变量
static R: AtomicU64 = AtomicU64::new(0);

// 线程函数，每个线程执行 N 次 `fetch_add`
fn add_n_times(n: u64) -> JoinHandle<()> {
    thread::spawn(move || {
        for _ in 0..n {
            R.fetch_add(1, Ordering::Relaxed);
        }
    })
}

fn main() {
    let start = Instant::now();
    let mut threads = Vec::with_capacity(N_THREADS);

    // 创建 10 个线程，每个线程执行 1000 万次加 1
    for _ in 0..N_THREADS {
        threads.push(add_n_times(N_TIMES));
    }

    // 等待所有线程执行完成
    for thread in threads {
        thread.join().unwrap();
    }

    // 验证最终结果是否正确
    assert_eq!(N_TIMES * N_THREADS as u64, R.load(Ordering::Relaxed));

    println!("执行时间: {:?}", start.elapsed());
}
```

**📌 解析**

- `AtomicU64::new(0)`：创建一个原子变量 R，初始值为 0。
- `fetch_add(1, Ordering::Relaxed)`：线程安全地执行自增操作。
- `load(Ordering::Relaxed)`：原子地读取变量值。

### 性能对比

| 实现方式 | 耗时 |
|---------|------|
| Atomic | 673ms |
| Mutex | 1136ms（慢 41%） |

Atomic 避免了锁的开销，因此更快。

## 3. Atomic 具备内部可变性

Atomic 类型本质上是内部可变的，无需使用 `mut` 修饰：

```rust
use std::sync::{Mutex};
use std::sync::atomic::{AtomicU64, Ordering};

struct Counter {
    count: u64,
}

fn main() {
    // `Mutex` 需要 `mut` 修饰
    let n = Mutex::new(Counter { count: 0 });
    n.lock().unwrap().count += 1;

    // `AtomicU64` 无需 `mut`
    let n = AtomicU64::new(0);
    n.fetch_add(1, Ordering::Relaxed);
}
```

**📌 解析**

- `Mutex` 必须使用 `mut` 修饰，因为它涉及锁定 + 修改。
- `Atomic` 具有内部可变性，可直接调用 `fetch_add()` 修改值。

## 4. Ordering 内存顺序

为什么需要 `Ordering`？

多线程程序可能因编译器优化、CPU 缓存机制导致内存访问顺序不同步，导致数据竞争。Rust 提供 5 种内存顺序模式，用于控制原子操作的执行顺序。

### Ordering 内存顺序 5 种模式

| 模式 | 作用 | 场景 |
|------|------|------|
| `Relaxed` | 无序操作，最快 | 计数器、自增 |
| `Release` | 释放内存屏障，保证之前的操作顺序正确 | 生产者（写入数据） |
| `Acquire` | 获取内存屏障，保证之后的操作顺序正确 | 消费者（读取数据） |
| `AcqRel` | `Acquire` + `Release` 组合 | 既要获取又要释放 |
| `SeqCst` | 最严格，保证全局一致性 | 绝对保证顺序 |

## 5. Release + Acquire 内存屏障

**📌 代码：生产者-消费者模型**

```rust
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::thread;

static DATA: AtomicU64 = AtomicU64::new(0);
static READY: AtomicBool = AtomicBool::new(false);

fn producer() {
    DATA.store(100, Ordering::Relaxed);   // 先写入数据
    READY.store(true, Ordering::Release); // 释放屏障，保证 `DATA = 100` 先执行
}

fn consumer() {
    while !READY.load(Ordering::Acquire) {} // 获取屏障，确保 `DATA = 100` 已完成
    assert_eq!(DATA.load(Ordering::Relaxed), 100);
}

fn main() {
    let t1 = thread::spawn(producer);
    let t2 = thread::spawn(consumer);

    t1.join().unwrap();
    t2.join().unwrap();
}
```

**📌 解析**

- `Release` 保证生产者 `DATA = 100` 先执行。
- `Acquire` 保证消费者 `READY = true` 后读取 `DATA`。

## 6. 多线程中使用 Atomic

Atomic 通常与 `Arc` 一起使用，确保多线程共享所有权：

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{hint, thread};

fn main() {
    let spinlock = Arc::new(AtomicUsize::new(1));
    let spinlock_clone = Arc::clone(&spinlock);

    let thread = thread::spawn(move || {
        spinlock_clone.store(0, Ordering::SeqCst);
    });

    // 等待其它线程释放锁
    while spinlock.load(Ordering::SeqCst) != 0 {
        hint::spin_loop(); // CPU 低功耗等待
    }

    thread.join().unwrap();
}
```

**📌 解析**

- `Arc<AtomicUsize>` 允许多个线程共享同一个 Atomic 变量。
- `spin_loop()` 避免空循环浪费 CPU 资源。

## 7. Atomic vs Mutex

| 方式 | 适用场景 |
|------|---------|
| Mutex | 适用于复杂对象，多个线程访问不同部分 |
| Atomic | 适用于数值变量（如计数器、状态标记），避免锁开销 |

> 📌 Atomic 无法替代 Mutex，因为 Atomic 只能操作基础类型（整数、布尔）。复杂数据结构（如 `Vec<T>`）仍需 Mutex 保护。

## 总结

- ✅ Atomic 适用于高性能计数、全局变量、无锁数据结构。
- ✅ `Ordering` 控制内存屏障，`Release + Acquire` 确保顺序一致。
- ✅ 高并发场景首选 Atomic，复杂数据结构仍需 Mutex。

> 🚀 Rust 提供强大的 Atomic 类型，掌握 Ordering，可构建高效的无锁并发程序！
