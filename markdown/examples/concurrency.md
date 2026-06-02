# Rust vs TypeScript: 并发与多线程

**运行命令：** `cargo run -p learning_notes --example rts_concurrency`

## TypeScript 版本

```ts
// Node.js Worker Threads
import { Worker, workerData, parentPort } from "worker_threads";

const worker = new Worker("./worker.js", { workerData: { n: 42 } });
worker.on("message", (result) => console.log("结果:", result));
worker.on("error", (err) => console.error(err));

// MessageChannel：线程间通信
const { port1, port2 } = new MessageChannel();
port1.on("message", (msg) => console.log("收到:", msg));
port2.postMessage("hello");

// SharedArrayBuffer + Atomics：共享内存
const sab = new SharedArrayBuffer(4);
const arr = new Int32Array(sab);
Atomics.add(arr, 0, 1);
Atomics.wait(arr, 0, 0); // 等待值变化

// Promise.all 并发（但仍是单线程 event loop）
const [r1, r2] = await Promise.all([task1(), task2()]);
```

## Rust 并发 vs TS 的关键差异

1. **真正的并行**：Rust 线程跑在独立 OS 线程上；Node.js 的 Worker 也是，但大多数 JS 代码运行在单线程 event loop
2. **编译期线程安全**：`Send`/`Sync` trait 保证编译器拒绝不安全的跨线程共享，TS 只能靠文档约定
3. **channel 是标准库内置的**：`std::sync::mpsc`，不需要第三方库
4. **共享状态用 `Arc<Mutex<T>>`**：比 SharedArrayBuffer 更高层、更安全

---

## 一、创建线程（vs Worker）

```rust
use std::thread;

// 最基本的线程
let handle = thread::spawn(|| {
    println!("在子线程中运行，线程 ID: {:?}", thread::current().id());
    42  // 线程返回值
});

let result = handle.join().unwrap(); // 等待线程完成，拿到返回值
println!("线程返回: {}", result);
```

```rust
// 传递数据给线程 — 用 move 转移所有权
let data = vec![1, 2, 3, 4, 5];

let handle = thread::spawn(move || {  // move 把 data 的所有权转入线程
    let sum: i32 = data.iter().sum();
    sum
});

println!("求和结果: {}", handle.join().unwrap());
```

---

## 二、消息传递（vs MessageChannel / BroadcastChannel）

Rust 哲学：**不要通过共享内存通信，而要通过通信共享内存**（同 Go 的理念）。

```rust
use std::sync::mpsc; // Multiple Producer, Single Consumer
use std::thread;

// 创建 channel
let (tx, rx) = mpsc::channel::<String>();

// 发送端：可以 clone，支持多个生产者
let tx2 = tx.clone();

thread::spawn(move || {
    tx.send("来自线程1的消息".to_string()).unwrap();
    tx.send("线程1的第二条消息".to_string()).unwrap();
});

thread::spawn(move || {
    tx2.send("来自线程2的消息".to_string()).unwrap();
});

// 接收端：迭代所有消息（直到所有发送端 drop）
for msg in rx {
    println!("收到: {}", msg);
}
```

```rust
// 带超时的接收
use std::time::Duration;

let (tx, rx) = mpsc::channel::<i32>();

thread::spawn(move || {
    thread::sleep(Duration::from_millis(100));
    tx.send(42).unwrap();
});

match rx.recv_timeout(Duration::from_millis(200)) {
    Ok(val) => println!("收到: {}", val),
    Err(_)  => println!("超时！"),
}
```

---

## 三、共享状态（vs SharedArrayBuffer）

当多个线程需要读写同一份数据时，用 `Arc<Mutex<T>>`：

- **`Arc`**（Atomic Reference Count）= 线程安全的 `Rc`，允许多个所有者
- **`Mutex`** = 互斥锁，保证同一时刻只有一个线程能访问数据

```rust
use std::sync::{Arc, Mutex};
use std::thread;

// Arc<Mutex<T>> 是 Rust 中"共享可变状态"的标准模式
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for i in 0..5 {
    let counter = Arc::clone(&counter); // 每个线程拿一份引用
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap(); // 加锁，返回 MutexGuard
        *num += 1;
        println!("线程 {} 将 counter 加到 {}", i, *num);
        // MutexGuard 离开作用域时自动解锁（RAII）
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("最终 counter = {}", *counter.lock().unwrap());
```

---

## 四、多线程并行计算

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn parallel_sum(numbers: Vec<i64>) -> i64 {
    let chunk_size = numbers.len() / 4;
    let numbers = Arc::new(numbers);
    let mut handles = vec![];

    for i in 0..4 {
        let numbers = Arc::clone(&numbers);
        let handle = thread::spawn(move || {
            let start = i * chunk_size;
            let end = if i == 3 { numbers.len() } else { start + chunk_size };
            numbers[start..end].iter().sum::<i64>()
        });
        handles.push(handle);
    }

    handles.into_iter().map(|h| h.join().unwrap()).sum()
}

// 更简洁的方式：rayon（见 popular_crates.md）
// numbers.par_iter().sum()
```

---

## 五、读写锁（RwLock）—— 多读单写

`Mutex` 是互斥锁，同一时刻只有一个访问者。
`RwLock` 允许**多个并发读者**或**一个独占写者**，适合读多写少的场景。

```rust
use std::sync::{Arc, RwLock};
use std::thread;

let config = Arc::new(RwLock::new(vec!["default".to_string()]));

// 多个读线程可以同时读
let mut read_handles = vec![];
for i in 0..3 {
    let config = Arc::clone(&config);
    let h = thread::spawn(move || {
        let data = config.read().unwrap(); // 共享读锁
        println!("读线程 {}: {:?}", i, *data);
    });
    read_handles.push(h);
}

for h in read_handles { h.join().unwrap(); }

// 写线程独占访问
let config_write = Arc::clone(&config);
thread::spawn(move || {
    let mut data = config_write.write().unwrap(); // 独占写锁
    data.push("new_setting".to_string());
}).join().unwrap();

println!("更新后: {:?}", *config.read().unwrap());
```

---

## 六、线程局部存储（Thread-local）

每个线程拥有独立的变量副本，无需加锁。

```rust
use std::cell::RefCell;

thread_local! {
    static REQUEST_ID: RefCell<u64> = RefCell::new(0);
}

fn set_request_id(id: u64) {
    REQUEST_ID.with(|r| *r.borrow_mut() = id);
}

fn get_request_id() -> u64 {
    REQUEST_ID.with(|r| *r.borrow())
}

// 每个线程独立，互不干扰
thread::spawn(|| {
    set_request_id(100);
    println!("线程A请求ID: {}", get_request_id()); // 100
});

thread::spawn(|| {
    set_request_id(200);
    println!("线程B请求ID: {}", get_request_id()); // 200
});
```

---

## 七、Send 和 Sync：编译期线程安全

这是 Rust 独有的概念，TS 没有对应物。

| Trait | 含义 |
|-------|------|
| `Send` | 类型的**所有权**可以安全地转移到另一个线程 |
| `Sync` | 类型的**引用**可以安全地在多个线程中共享（`&T` 是 `Send`） |

```rust
// 编译器自动推断，通常不需要手动标注
// 大多数基础类型都是 Send + Sync

// 这些不是 Send（不能跨线程传递）：
// Rc<T>          — 用 Arc<T> 代替
// RefCell<T>     — 用 Mutex<T> 代替
// *mut T / *const T  — 裸指针

fn needs_send<T: Send>(val: T) { /* 只接受可跨线程的类型 */ }
fn needs_sync<T: Sync>(val: &T) { /* 只接受可共享引用的类型 */ }
```

---

## 总结对比

| 场景 | TypeScript | Rust |
|------|-----------|------|
| 创建线程 | `new Worker(...)` | `thread::spawn(move \|\| ...)` |
| 线程间通信 | `MessageChannel` | `mpsc::channel()` |
| 多线程共享数据 | `SharedArrayBuffer` + `Atomics` | `Arc<Mutex<T>>` |
| 多读单写 | 手动实现 | `Arc<RwLock<T>>` |
| 数据并行 | `Promise.all` (event loop) | `rayon` / `thread::spawn` |
| 线程安全检查 | 运行时 / 开发者自觉 | 编译期（`Send`/`Sync` trait） |
| 线程局部存储 | `AsyncLocalStorage` | `thread_local!` |
