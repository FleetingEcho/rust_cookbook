# Rust 全局变量整理总结

在 Rust 中，全局变量是**共享数据**的常见方式，可用于全局 ID、全局配置、全局计数器等情况。Rust 提供了多种全局变量的创建方式，主要分为**编译期初始化**和**运行期初始化**两大类。

## 1. 编译期初始化的全局变量

编译期初始化的全局变量在编译时确定值，适用于静态配置、计数器、状态值等。

### 1.1 const：静态常量

`const` 定义的是**不可变**的编译期常量，类似于 C 语言的 `#define`。

```rust
const MAX_ID: usize = usize::MAX / 2;

fn main() {
   println!("用户ID允许的最大值是 {}", MAX_ID);
}
```

**📌 特点**

- 关键字是 `const`，不可变。
- 必须指定类型（如 `i32`）。
- 可在任何作用域使用，生命周期贯穿整个程序。
- 编译时已确定值，不能包含运行期计算的内容（如函数调用）。

### 1.2 static：静态变量

`static` 允许声明全局变量，适用于全局状态计数、日志管理等。

```rust
static mut REQUEST_RECV: usize = 0;

fn main() {
   unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
   }
}
```

**📌 特点**

- `static` 变量是唯一的实例，所有引用指向同一内存地址。
- 默认不可变，但 `mut` 变量需要 `unsafe` 代码块访问。
- 不能在运行期赋值，初始化值必须是常量表达式。

> ⚠️ `static mut` 在多线程环境中不安全，修改全局变量可能导致**数据竞争**。适用于单线程或不关心数据准确性的场景。

### 1.3 Atomic：线程安全的全局变量

如果需要**线程安全**的全局计数器，可以使用原子类型 (`AtomicUsize`)：

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

fn main() {
    for _ in 0..100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户请求数: {}", REQUEST_RECV.load(Ordering::Relaxed));
}
```

**📌 特点**

- `AtomicUsize` 适用于多线程环境，无需使用 `Mutex`。
- 使用 `fetch_add` 增加计数，`Ordering::Relaxed` 控制内存顺序。
- 比 `Mutex` 更高效，但仅支持基本的数值操作。

**示例：全局 ID 生成器**

利用 `AtomicUsize` 实现线程安全的全局 ID 生成器：

```rust
use std::sync::atomic::{Ordering, AtomicUsize};

static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
const MAX_ID: usize = usize::MAX / 2;

fn generate_id() -> usize {
    let current_val = GLOBAL_ID_COUNTER.load(Ordering::Relaxed);
    if current_val > MAX_ID {
        panic!("Factory IDs overflowed");
    }
    GLOBAL_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    GLOBAL_ID_COUNTER.load(Ordering::Relaxed)
}

struct Factory {
    factory_id: usize,
}

impl Factory {
    fn new() -> Self {
        Self {
            factory_id: generate_id(),
        }
    }
}
```

**📌 原子计数器**

- 适用于全局 ID 分配、状态计数等。
- `fetch_add` 确保多线程安全，避免数据竞争。

## 2. 运行期初始化的全局变量

编译期静态变量无法包含运行期计算（如 `Mutex::new(String::from("test"))`）。需要使用运行期初始化，例如全局锁、动态配置、缓存。

### 2.1 lazy_static! 宏

`lazy_static!` 允许在运行时初始化静态变量，适用于全局锁、全局配置、缓存。

```rust
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref NAMES: Mutex<String> = Mutex::new(String::from("Sunface, Jack, Allen"));
}

fn main() {
    let mut v = NAMES.lock().unwrap();
    v.push_str(", Myth");
    println!("{}", v);
}
```

**📌 特点**

- 支持 `Mutex<String>` 作为全局变量。
- 初始化在运行期，不会在编译期检查。
- 惰性初始化，仅在**首次访问时**进行。

> ⚠️ `lazy_static!` 每次访问时会有轻微性能损耗（使用 `std::sync::Once`）。

### 2.2 Box::leak 让变量变为 'static

如果不使用 `lazy_static!`，可以用 `Box::leak` 手动提升变量的生命周期：

```rust
#[derive(Debug)]
struct Config {
    a: String,
    b: String,
}
static mut CONFIG: Option<&mut Config> = None;

fn main() {
    let c = Box::new(Config {
        a: "A".to_string(),
        b: "B".to_string(),
    });

    unsafe {
        CONFIG = Some(Box::leak(c)); // 让变量成为 `'static`
        println!("{:?}", CONFIG);
    }
}
```

**📌 特点**

- `Box::leak(c)` 让变量永远不会被释放，等同于 `'static`。
- 适用于全局动态配置，但需手动管理内存。

> ⚠️ `Box::leak` 会导致内存泄漏（Rust 不能自动回收）。适用于整个程序生命周期都需要的数据。

### 2.3 OnceCell 和 OnceLock

Rust 1.70 以上提供了 `OnceCell` 和 `OnceLock`，是更现代的 `lazy_static!` 替代品。

```rust
use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug)]
struct Logger;

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| Logger)
    }
}
```

**📌 特点**

- 标准库自带，无需外部依赖。
- 惰性初始化，线程安全。
- `get_or_init` 接受闭包，保证只执行一次初始化。

## 3. 总结

| 方案 | 初始化时机 | 线程安全 | 适用场景 |
|------|-----------|---------|---------|
| `const` | 编译期 | ✅ | 不可变常量 |
| `static` | 编译期 | ❌ (需 unsafe) | 简单全局状态 |
| `AtomicUsize` | 编译期 | ✅ | 线程安全计数 |
| `lazy_static!` | 运行期 | ✅ | 全局锁/配置 |
| `Box::leak` | 运行期 | ❌ (需 unsafe) | 全局动态配置 |
| `OnceLock` | 运行期 | ✅ | 现代替代 lazy_static |

> 合理选择全局变量的创建方式，在安全性和灵活性之间找到平衡。

## 📘 TypeScript 对比

Rust 全局变量 — TS 没有编译期/运行期的概念。

```ts
// TS 中全局变量就是 module-level const/let
const MAX_ID = 0x7FFFFFFF;
let counter = 0;
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 编译期常量 | `const` | `const` |
| 全局可变 | `static mut` (unsafe) | module-level `let` |
| 线程安全 | `AtomicUsize` | 单线程无此问题 |
| 惰性初始化 | `lazy_static!` / `OnceLock` | module scope 自动惰性 |

> ⚠️ Rust 对全局变量的线程安全性有严格要求，TypeScript 运行在单线程环境不需要考虑这些问题。

详细对照 → `rust_vs_typescript.rs §8 "全局变量"`
