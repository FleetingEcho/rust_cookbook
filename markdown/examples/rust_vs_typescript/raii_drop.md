# Rust vs TypeScript：RAII 与 Drop

> **运行命令**：`cargo run -p learning_notes --example rts_raii_drop`

---

## TypeScript 参考版本

```ts
// RAII = Resource Acquisition Is Initialization
// 资源获取即初始化：对象的生命周期 = 资源持有期
//
// TS/JS 没有 RAII 概念，资源管理依赖 GC 或手动 try/finally：

// 文件操作
let fileHandle: FileHandle | null = null;
try {
    fileHandle = fs.openSync("data.txt");
    const content = fs.readFileSync(fileHandle);
    // 使用内容...
} finally {
    if (fileHandle) fs.closeSync(fileHandle);  // 必须手动清理
}

// 锁操作
const lock = new Mutex();
lock.acquire();
try {
    // 临界区...
} finally {
    lock.release();  // 必须手动释放
}

// 计时器
const timer = setInterval(() => {}, 1000);
// 必须手动调用 clearInterval(timer)

// GC 的局限性：GC 只管理内存，不管理文件句柄、锁、网络连接等 OS 资源
// JS 的 WeakRef / FinalizationRegistry 可以监听对象被 GC，但不保证执行时机
```

---

## 一、RAII 核心思想

**TS 程序员需要理解的思维转变**：

| TypeScript | Rust (RAII) |
|---|---|
| `new File()` → 手动 `close()` | `File::open()` → 离开作用域自动 `close()` |
| `lock()` → 手动 `unlock()` | `lock()` → 作用域结束自动 `unlock()` |
| 忘记释放 = 资源泄漏 | 忘记释放 = 编译错误（不可能的） |

```
TS: 你负责 acquire → ... → release（try/finally）
Rust: 你负责 acquire → ... 编译器负责 release
```

---

## 二、RAII 在标准库中的应用

### 1. Mutex 锁（最经典的 RAII 示例）

**TS**: `lock(); try { ... } finally { unlock(); }`

```rust
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0_i32));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        // lock() 返回 MutexGuard，离开此作用域自动释放锁！
        let mut guard = c.lock().unwrap();
        *guard += 1;
        // guard 在这里被 drop，锁自动释放
        // TS 需要 try/finally 手动释放
    });
    handles.push(handle);
}

for h in handles { h.join().unwrap(); }
println!("counter = {}", *counter.lock().unwrap()); // 10
```

### 2. 文件操作

```rust
{
    let mut file = File::create(Path::new("/tmp/raii_demo.txt"))
        .expect("创建文件失败");
    file.write_all(b"Hello from Rust RAII!").expect("写入失败");
    println!("文件已写入，但尚未关闭");
} // file.drop() 被自动调用 → 文件关闭
println!("文件已自动关闭");
```

### 3. 错误路径上的 RAII

```rust
fn might_fail(should_fail: bool) -> Result<(), String> {
    let mut file = File::create("/tmp/raii_error_demo.txt")
        .map_err(|e| format!("创建文件失败: {e}"))?;
    file.write_all(b"some data").map_err(|e| format!("写入失败: {e}"))?;

    if should_fail {
        // 即使提前 return Err，file 也会被正确关闭
        return Err(String::from("模拟错误"));
    }
    Ok(())
}

// 无论成功还是失败，文件句柄都不会泄漏！
```

---

## 三、自定义 Drop 实现

**TS**: 没有 `drop` 机制。

**Rust**: `Drop` trait 可以在值离开作用域时执行清理逻辑。

```rust
#[derive(Debug)]
struct FileHandle {
    name: String,
}

impl Drop for FileHandle {
    fn drop(&mut self) {
        println!("文件 '{}' 正在关闭...", self.name);
        // 这里执行实际关闭文件的逻辑
    }
}

{
    let f = FileHandle { name: String::from("test.txt") };
    println!("文件 '{}' 已打开", f.name);
} // f 离开作用域 → 自动调用 drop() → 打印"文件正在关闭"
println!("文件已自动关闭");
```

---

## 四、手动 Drop

```rust
{
    let f = FileHandle { name: String::from("manual.txt") };
    println!("手动 drop 前");
    drop(f);  // 显式提前 drop
    // println!("{}", f.name); // ❌ 已被 drop，不可用
}
println!("手动 drop 已执行");
```

---

## 五、Drop 在锁和 RAII 中的应用

**TS**: `const lock = mutex.lock(); try { ... } finally { lock.unlock(); }`

**Rust 自动做 finally 的工作。**

```rust
let mutex = Mutex::new(0_i32);
{
    let mut guard = mutex.lock().unwrap();
    *guard += 1;
    // guard 离开作用域时自动 unlock
}
```

---

## 六、Rc 和 Arc 的引用计数（也是 Drop 的应用）

**TS** 的 GC 在后台运行，**Rust** 的 Rc/Arc 在 drop 时立即调整计数。

```rust
use std::rc::Rc;

{
    let a = Rc::new(String::from("hello"));
    println!("引用计数: {}", Rc::strong_count(&a)); // 1
    {
        let _b = Rc::clone(&a);
        println!("引用计数: {}", Rc::strong_count(&a)); // 2
    } // _b.drop() → 计数减1
    println!("引用计数: {}", Rc::strong_count(&a)); // 1
    // 当 a 离开作用域时，计数变为 0，内存立即释放
    // TS 的 GC 无法精确控制释放时机
} // a.drop() → 计数0 → 内存释放
```

---

## 七、作用域守卫（Scope Guard）模式

**TS** 需要 try/finally 或 defer 库。

```rust
struct Defer<F: FnMut()> {
    f: F,
}

impl<F: FnMut()> Drop for Defer<F> {
    fn drop(&mut self) {
        (self.f)();  // 离开作用域时执行闭包
    }
}

fn defer<F: FnMut()>(f: F) -> Defer<F> {
    Defer { f }
}

// 使用：类似 TS 的 try/finally
{
    let _guard = defer(|| println!(">> 清理操作：关闭文件"));
    let _guard2 = defer(|| println!(">> 清理操作：释放锁"));
    println!("主逻辑执行中...");
} // 两个 defer 在这里反向执行（类似 TS finally 栈）
// 输出顺序：
//   主逻辑执行中...
//   >> 清理操作：释放锁
//   >> 清理操作：关闭文件
```

---

## 八、RAII 在 Rust 生态中的常见应用

1. `std::fs::File` — 文件自动关闭
2. `std::sync::Mutex` — 锁自动释放
3. `std::sync::RwLockReadGuard` / `RwLockWriteGuard` — 读写锁
4. `std::io::BufWriter` — 自动 flush
5. `tracing/log` 的 span — 进入/离开日志
6. `sqlx::Transaction` — 事务自动提交/回滚
7. `tempfile::TempDir` — 临时目录自动删除
8. `scopeguard` crate — `defer!` 宏

---

## 总结对照表

| TypeScript | Rust |
|---|---|
| GC 管理内存 | 所有权系统 + Drop 管理所有资源 |
| 手动 try/finally 释放非内存资源 | 自动释放（RAII + Drop trait） |
| 可能忘记 close/unlock 导致泄漏 | 编译器确保释放（无法忘记） |
| 清理代码在 try 块的 finally 中 | 清理代码在资源的 Drop::drop() 中 |
| setTimeout/clearTimeout 手动清除 | Timer drop 时自动取消 |
| FinalizationRegistry 不可靠 | Drop 确定性执行 |
| 作用域结束不自动触发清理 | 离开作用域 = 触发所有 Drop |
| 错误路径容易忘记 finally | 错误路径的 Drop 同样执行 |
| 未定义变量的释放时机 | 变量离开作用域立即释放 |
