// Rust 全局变量整理总结
// 在 Rust 中，全局变量是 共享数据 的常见方式，可用于 全局 ID、全局配置、全局计数器 等情况。Rust 提供了多种全局变量的创建方式，主要分为 编译期初始化 和 运行期初始化 两大类。

// 1. 编译期初始化的全局变量
// 编译期初始化的全局变量 在编译时确定值，适用于 静态配置、计数器、状态值 等。

// 1.1 const：静态常量
// const 定义的是 不可变 的 编译期常量，类似于 C 语言的 #define。

const MAX_ID: usize = usize::MAX / 2;

fn main() {
   println!("用户ID允许的最大值是 {}", MAX_ID);
}
// 📌 特点

// 关键字是 const，不可变。
// 必须指定类型（如 i32）。
// 可在任何作用域使用，生命周期贯穿整个程序。
// 编译时已确定值，不能包含运行期计算的内容（如函数调用）。


// 1.2 static：静态变量
// static 允许声明 全局变量，适用于 全局状态计数、日志管理 等。


static mut REQUEST_RECV: usize = 0;

fn main() {
   unsafe {
        REQUEST_RECV += 1;
        assert_eq!(REQUEST_RECV, 1);
   }
}
// 📌 特点

// static 变量是唯一的实例，所有引用指向同一内存地址。
// 默认不可变，但 mut 变量需要 unsafe 代码块访问。
// 不能在运行期赋值，初始化值必须是 常量表达式。
// ⚠ 注意

// static mut 在多线程环境中不安全，修改全局变量可能导致 数据竞争。
// 适用于 单线程或不关心数据准确性 的场景。


// 1.3 Atomic：线程安全的全局变量
// 如果需要 线程安全 的全局计数器，可以使用 原子类型 (AtomicUsize)：


use std::sync::atomic::{AtomicUsize, Ordering};

static REQUEST_RECV: AtomicUsize = AtomicUsize::new(0);

fn main() {
    for _ in 0..100 {
        REQUEST_RECV.fetch_add(1, Ordering::Relaxed);
    }

    println!("当前用户请求数: {}", REQUEST_RECV.load(Ordering::Relaxed));
}
// 📌 特点

// AtomicUsize 适用于 多线程环境，无需使用 Mutex。
// 使用 fetch_add 增加计数，Ordering::Relaxed 控制内存顺序。
// 比 Mutex 更高效，但仅支持 基本的数值操作。
// 示例：全局 ID 生成器
// 利用 AtomicUsize 实现 线程安全的全局 ID 生成器：


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
// 📌 原子计数器

// 适用于 全局 ID 分配、状态计数 等。
// fetch_add 确保多线程安全，避免数据竞争。


// 2. 运行期初始化的全局变量
// 编译期静态变量 无法包含运行期计算（如 Mutex::new(String::from("test"))）。
// 需要使用 运行期初始化，例如 全局锁、动态配置、缓存。

// 2.1 lazy_static! 宏
// lazy_static! 允许 在运行时初始化静态变量，适用于 全局锁、全局配置、缓存。


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
// 📌 特点

// 支持 Mutex<String> 作为全局变量。
// 初始化在运行期，不会在编译期检查。
// 惰性初始化，仅在 首次访问时 进行。
// ⚠ 注意

// lazy_static! 每次访问时会有轻微性能损耗（使用 std::sync::Once）。


// 2.2 Box::leak 让变量变为 'static
// 如果不使用 lazy_static!，可以用 Box::leak 手动提升变量的生命周期：


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
// 📌 特点

// Box::leak(c) 让变量永远不会被释放，等同于 'static。
// 适用于 全局动态配置，但 需手动管理内存。
// ⚠ 注意

// Box::leak 会导致内存泄漏（Rust 不能自动回收）。
// 适用于 整个程序生命周期都需要的数据。



// 2.3 OnceCell 和 OnceLock
// Rust 1.70 以上提供了 OnceCell 和 OnceLock，是 更现代的 lazy_static! 替代品。


use std::sync::OnceLock;

static LOGGER: OnceLock<Logger> = OnceLock::new();

#[derive(Debug)]
struct Logger;

impl Logger {
    fn global() -> &'static Logger {
        LOGGER.get_or_init(|| {
            println!("Logger is being created...");
            Logger
        })
    }

    fn log(&self, message: &str) {
        println!("{}", message)
    }
}

fn main() {
    let logger = Logger::global();
    logger.log("Hello, Rust!");

    let logger2 = Logger::global();
    logger2.log("Another message");
}
// 📌 特点

// OnceLock<T> 只初始化一次，适用于 全局日志、数据库连接 等。
// 多线程安全，自动管理初始化状态。
// ⚠ Rust 1.70+

// OnceCell<T> 适用于单线程。
// OnceLock<T> 适用于多线程（替代 lazy_static!）。


// 3. 总结
// 📌 全局变量的选择


// 方式	特点	适用场景
// const	编译期常量，不可变	配置、数学常量
// static	静态变量，需 unsafe	全局状态、计数器
// Atomic	线程安全，适合计数	计数器、ID 生成
// lazy_static!	运行期初始化	线程安全全局变量
// Box::leak	手动提升生命周期	全局动态配置
// OnceLock	只初始化一次，线程安全	日志、数据库连接

// ✅ 编译期初始化： const、static、Atomic

// ✅ 运行期初始化： lazy_static!、Box::leak、OnceLock

// 🚀 Rust 提供多种方式管理全局变量，合理选择，确保线程安全！