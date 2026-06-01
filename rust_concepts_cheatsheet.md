# Rust 核心概念速查手册

> 覆盖：**内存管理**（栈/堆·所有权·借用·生命周期）·**类型系统**（trait·泛型·dyn）·**错误处理**（Result·Option·?·thiserror·anyhow）·**智能指针**（Box·Rc·Arc·RefCell·Mutex·RwLock）·**并发**（thread·channel·async/tokio）
> 数据类型方法见配套文档《rust_types_cheatsheet.md》

---

## 一、内存管理基础

### 1.1 栈 vs 堆

```
栈（Stack）                    堆（Heap）
──────────────────────         ──────────────────────
大小编译期已知                  大小运行时才知道
自动分配/释放（函数返回即清）    手动（Rust：所有者离开作用域自动 drop）
速度快                          速度较慢（需分配器）
──────────────────────         ──────────────────────
i32, bool, char, [T;N]         String, Vec<T>, Box<T>
结构体（所有字段都是栈类型时）  trait object, Rc, Arc
```

```rust
let x = 5;              // 栈：直接存值
let s = String::from("hello"); // 堆：栈上存 ptr/len/cap，数据在堆
```

---

### 1.2 所有权（Ownership）

**三条规则（整个系统的基础）：**
1. Rust 中每个值都有且仅有一个**所有者**（owner）
2. 同一时刻只能有一个所有者
3. 所有者离开作用域，值被自动 `drop`（释放内存）

```rust
// 移动（Move）：栈上的元数据被复制，但"所有权"转移，原变量失效
let s1 = String::from("hello");
let s2 = s1;            // s1 的所有权移动给 s2
// println!("{s1}");    // ❌ 编译错误：s1 已失效

// Copy：实现了 Copy trait 的类型直接位拷贝，原变量仍有效
let x = 5;
let y = x;              // 拷贝，x 和 y 都有效
println!("{x} {y}");    // ✅

// Clone：显式深拷贝（堆数据也复制一份）
let s1 = String::from("hello");
let s2 = s1.clone();    // 两份独立数据，两者都有效
```

**实现 Copy 的类型**（赋值/传参不移动）：
```rust
// 所有数值类型：i8~i128, u8~u128, f32, f64, usize, isize
// bool, char
// 引用：&T（不可变引用实现 Copy）
// 数组 [T; N] 当 T: Copy 时
// 元组 (T, U) 当 T, U: Copy 时
```

---

### 1.3 借用（Borrowing）与引用

**核心规则（借用检查器强制执行）：**
- 同一时刻：要么有**任意多个不可变引用 `&T`**，要么有**恰好一个可变引用 `&mut T`**，二者不能同时存在
- 引用必须始终有效（不能悬垂）

```rust
// 不可变引用：可以同时存在多个
let s = String::from("hello");
let r1 = &s;
let r2 = &s;            // ✅ 多个不可变引用可以共存
println!("{r1} {r2}");

// 可变引用：同一时刻只能有一个
let mut s = String::from("hello");
let r = &mut s;
r.push_str(" world");   // ✅
// let r2 = &mut s;     // ❌ 已有可变引用，不能再借用

// 不可变与可变不能同时存在
let mut s = String::from("hello");
let r1 = &s;            // 不可变借用开始
// let r2 = &mut s;     // ❌ r1 还在用，不能可变借用
println!("{r1}");       // r1 最后一次使用（Non-Lexical Lifetime：借用在此结束）
let r2 = &mut s;        // ✅ r1 已结束，可以开始可变借用
```

```rust
// 函数中的引用：借用不转移所有权
fn len(s: &String) -> usize {  // 借用，不消耗 s
    s.len()
}
let s = String::from("hi");
println!("{}", len(&s));
println!("{s}");        // ✅ s 依然有效
```

---

### 1.4 生命周期（Lifetime）

> 生命周期注解**不改变**引用的实际存活时长，只是告诉编译器"多个引用之间的寿命关系"，让借用检查器能验证安全性。

```rust
// 问题：编译器不知道返回的引用来自 x 还是 y，无法判断是否安全
fn longest(x: &str, y: &str) -> &str {   // ❌ 缺少生命周期注解
    if x.len() > y.len() { x } else { y }
}

// 解决：用 'a 声明"返回值的生命周期 ≤ 输入参数中较短的那个"
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // ✅
    if x.len() > y.len() { x } else { y }
}
```

```rust
// 结构体持有引用时必须标注
struct Important<'a> {
    text: &'a str,          // text 的存活时间不能短于 Important 实例
}

impl<'a> Important<'a> {
    fn content(&self) -> &str {
        self.text           // 省略规则：返回 &self 相关的引用可省略注解
    }
}
```

**生命周期省略规则（Elision Rules）**——满足以下规则时可省略：
1. 每个引用参数得到独立的生命周期
2. 只有一个输入引用 → 输出引用的生命周期 = 该输入
3. 方法中有 `&self` 或 `&mut self` → 输出引用的生命周期 = self 的生命周期

```rust
// 'static：整个程序运行期间都有效
let s: &'static str = "hello"; // 字符串字面量都是 'static
```

---

## 二、类型系统

### 2.1 Trait（接口/行为约束）

```rust
// 定义 trait
trait Animal {
    fn name(&self) -> &str;                     // 必须实现
    fn sound(&self) -> &str;                    // 必须实现
    fn description(&self) -> String {           // 默认实现（可覆盖）
        format!("{} says {}", self.name(), self.sound())
    }
}

// 实现 trait
struct Dog;
impl Animal for Dog {
    fn name(&self) -> &str { "Dog" }
    fn sound(&self) -> &str { "Woof" }
    // description 使用默认实现
}

struct Cat;
impl Animal for Cat {
    fn name(&self) -> &str { "Cat" }
    fn sound(&self) -> &str { "Meow" }
    fn description(&self) -> String {           // 覆盖默认实现
        "A mysterious cat".to_string()
    }
}
```

**常用标准库 trait：**
```rust
// Display：用于 {} 格式化（面向用户）
use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Debug：用于 {:?} 格式化（面向开发者，通常 derive）
#[derive(Debug)]
struct Point { x: f64, y: f64 }

// PartialEq / Eq：== 运算符
#[derive(PartialEq, Eq)]
struct Id(u32);

// PartialOrd / Ord：< > 运算符
#[derive(PartialOrd, Ord, PartialEq, Eq)]
struct Score(i32);

// Clone / Copy
#[derive(Clone, Copy)]
struct Vec2 { x: f32, y: f32 }

// Hash：可用作 HashMap 的 key
#[derive(Hash, PartialEq, Eq)]
struct UserId(u64);

// Default：提供默认值
#[derive(Default)]              // 所有字段各自的 default
struct Config { timeout: u32, retries: u32 }
let c = Config::default();      // Config { timeout: 0, retries: 0 }

// From / Into：类型转换
impl From<&str> for MyError {
    fn from(s: &str) -> Self { MyError(s.to_string()) }
}
let e: MyError = "oops".into(); // Into 由 From 自动推导

// Iterator：实现 next() 即获得所有迭代器方法
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> { ... }
}
```

---

### 2.2 泛型（Generics）

```rust
// 泛型函数：T 在调用时被具体类型替换（单态化，monomorphization）
fn largest<T: PartialOrd>(list: &[T]) -> &T {  // T 必须能比较大小
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}

// 使用
println!("{}", largest(&[34, 50, 25, 100])); // T = i32
println!("{}", largest(&["apple", "mango"])); // T = &str
```

```rust
// 泛型结构体
struct Pair<T> {
    first: T,
    second: T,
}

// 为特定 T 实现方法（只有 T: Display + PartialOrd 时才有这个方法）
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("first: {}", self.first);
        } else {
            println!("second: {}", self.second);
        }
    }
}
```

```rust
// 多重 trait bound（用 + 连接）
fn print_and_compare<T: fmt::Display + PartialOrd>(a: T, b: T) {
    println!("{} vs {}", a, b);
}

// where 从句（bound 复杂时更清晰）
fn complex<T, U>(t: &T, u: &U) -> String
where
    T: fmt::Display + Clone,
    U: fmt::Debug + Clone,
{
    format!("{} {:?}", t, u)
}
```

```rust
// 关联类型（Associated Type）：比泛型参数更简洁
trait Container {
    type Item;                  // 关联类型
    fn first(&self) -> Option<&Self::Item>;
}

// impl 时指定具体类型，调用方不需要写 Container<Item=i32>
impl Container for Vec<i32> {
    type Item = i32;
    fn first(&self) -> Option<&i32> { self.get(0) }
}
```

**单态化（Monomorphization）**：
```
// 编译器为每个实际用到的类型生成独立的具体函数
largest::<i32>(...)   → 生成针对 i32 的机器码
largest::<&str>(...)  → 生成针对 &str 的机器码
// 结果：运行时零开销（但编译产物体积会增大）
```

---

### 2.3 impl Trait vs dyn Trait

```rust
trait Drawable { fn draw(&self); }

struct Circle;
struct Square;
impl Drawable for Circle { fn draw(&self) { println!("circle"); } }
impl Drawable for Square { fn draw(&self) { println!("square"); } }
```

**impl Trait（静态分发）**
```rust
// 函数参数：语法糖，等价于泛型 <T: Drawable>
fn render(shape: &impl Drawable) {   // 编译期确定具体类型，零开销
    shape.draw();
}

// 函数返回值：隐藏具体类型，但只能返回一种类型
fn make_shape() -> impl Drawable {
    Circle                           // 只能返回 Circle，不能根据条件返回不同类型
}
```

**dyn Trait（动态分发，trait object）**
```rust
// 编译期不知道具体类型，运行时通过虚表（vtable）调用
// 必须用引用或 Box 包裹（因为大小不确定）
fn render_all(shapes: &[Box<dyn Drawable>]) {
    for s in shapes { s.draw(); }    // 运行时查 vtable，有一点点开销
}

// 可以存放不同类型的集合（impl Trait 做不到）
let shapes: Vec<Box<dyn Drawable>> = vec![
    Box::new(Circle),
    Box::new(Square),               // ✅ 不同类型可以混放
];
render_all(&shapes);
```

**选择原则：**
```
impl Trait / 泛型   → 性能优先，返回类型单一，编译期已知
dyn Trait          → 需要运行时多态，存放不同类型的集合，插件架构
```

**对象安全（Object Safety）**：并非所有 trait 都能做 trait object：
```rust
// ❌ 不对象安全：方法返回 Self，或有泛型方法
trait Clone { fn clone(&self) -> Self; }  // 不能 dyn Clone

// ✅ 对象安全：方法只用 &self / &mut self，参数/返回值不含 Self 或泛型
trait Drawable { fn draw(&self); }        // 可以 dyn Drawable
```

---

## 三、错误处理

### 3.1 panic! 与不可恢复错误

```rust
panic!("something went wrong");   // 打印信息，展开栈，终止程序

// 常见隐式 panic 场景：
v[100]                            // 越界访问
opt.unwrap()                      // Option 为 None
res.unwrap()                      // Result 为 Err
// 生产代码应避免这些，用 .get() / match / ? 替代
```

---

### 3.2 Result\<T, E\> 与 ? 运算符

```rust
use std::fs;
use std::io;

// 基本用法
fn read_file(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?; // ? = 出错时提前 return Err
    Ok(content)
}

// ? 的完整展开等价于：
// match fs::read_to_string(path) {
//     Ok(v)  => v,
//     Err(e) => return Err(e.into()),  // .into() 允许自动类型转换
// }

// 链式调用
fn process(path: &str) -> Result<usize, io::Error> {
    let content = fs::read_to_string(path)?;
    let trimmed = content.trim();
    Ok(trimmed.len())
}
```

---

### 3.3 自定义错误类型

**方式一：手动实现（理解原理）**
```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
    Custom(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e)     => write!(f, "IO 错误: {e}"),
            AppError::Parse(e)  => write!(f, "解析错误: {e}"),
            AppError::Custom(s) => write!(f, "错误: {s}"),
        }
    }
}

// 实现 From 让 ? 自动转换错误类型
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}
impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self { AppError::Parse(e) }
}

fn run() -> Result<(), AppError> {
    let s = std::fs::read_to_string("data.txt")?;  // io::Error 自动转 AppError
    let n: i32 = s.trim().parse()?;                // ParseIntError 自动转 AppError
    println!("{n}");
    Ok(())
}
```

**方式二：thiserror（推荐，库代码）**
```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("IO 错误: {0}")]            // Display 自动生成
    Io(#[from] std::io::Error),         // From 自动生成（#[from]）

    #[error("解析失败: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("配置项 {key} 缺失")]
    MissingConfig { key: String },
}

// 使用完全一样，只是不用手写 Display 和 From
fn run() -> Result<(), AppError> {
    let s = std::fs::read_to_string("cfg.txt")?;   // 自动转换
    Ok(())
}
```

**方式三：anyhow（推荐，应用代码）**
```rust
use anyhow::{Context, Result, bail, ensure, anyhow};

// anyhow::Result<T> = Result<T, anyhow::Error>
// anyhow::Error 能包装任何 std::error::Error
fn run() -> Result<()> {
    let s = std::fs::read_to_string("data.txt")
        .context("读取配置文件失败")?;              // 附加上下文信息

    let n: i32 = s.trim().parse()
        .context("配置值必须是整数")?;

    ensure!(n > 0, "值必须为正数，实际为 {n}");    // 条件不满足则返回 Err
    bail!("直接返回错误");                          // 等价于 return Err(anyhow!("..."))

    Ok(())
}

// 打印完整错误链
fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {e:?}");   // {:?} 打印完整调用链
    }
}
```

**选择原则：**
```
库（发布给他人使用）  → thiserror：精确的类型化错误，调用方可 match
应用（最终可执行文件）→ anyhow：快速开发，丰富的错误上下文
```

---

## 四、智能指针

### 4.1 Box\<T\>（堆分配·唯一所有权）

```rust
// 用途1：在堆上分配，栈上只存指针（大型结构体或不知道大小时）
let b = Box::new(5);            // 5 存在堆上
println!("{}", *b);             // 解引用用 *

// 用途2：递归类型（大小不确定，用 Box 打破循环）
enum List {
    Cons(i32, Box<List>),       // 没有 Box 编译器算不出大小 ❌
    Nil,
}
let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

// 用途3：trait object
let shape: Box<dyn Draw> = Box::new(Circle::new());

// Box 离开作用域自动 drop，同时释放堆内存
```

---

### 4.2 Rc\<T\>（引用计数·共享所有权·单线程）

```rust
use std::rc::Rc;

// 多个所有者共享同一份数据（单线程）
let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);          // 引用计数 +1（克隆的是指针，不是数据）
let c = Rc::clone(&a);

println!("引用计数: {}", Rc::strong_count(&a)); // 3
// a, b, c 都离开作用域后，引用计数降至 0，数据才被 drop

// Rc 只提供不可变访问
// 需要可变时：配合 RefCell（见下文）
```

---

### 4.3 Arc\<T\>（原子引用计数·共享所有权·多线程）

```rust
use std::sync::Arc;
use std::thread;

// Arc = Atomic Rc，线程安全，但稍慢于 Rc（原子操作）
let data = Arc::new(vec![1, 2, 3]);

let data_clone = Arc::clone(&data);
let handle = thread::spawn(move || {
    println!("{:?}", data_clone);   // 子线程安全访问
});

println!("{:?}", data);             // 主线程也能用
handle.join().unwrap();
```

---

### 4.4 RefCell\<T\>（内部可变性·运行时借用检查·单线程）

```rust
use std::cell::RefCell;

// 正常借用规则在编译期检查；RefCell 把检查推迟到运行时
// 允许在不可变引用的情况下修改内部数据
let data = RefCell::new(vec![1, 2, 3]);

{
    let mut v = data.borrow_mut();  // 运行时获取可变借用（违规则 panic）
    v.push(4);
}   // 可变借用在此释放

let v = data.borrow();              // 运行时获取不可变借用
println!("{:?}", *v);               // [1, 2, 3, 4]

// 同时借用 borrow 和 borrow_mut 会 panic（运行时！不是编译时）
```

---

### 4.5 Rc\<RefCell\<T\>\>（共享 + 可变·单线程）

```rust
use std::rc::Rc;
use std::cell::RefCell;

// 需要多个所有者且需要修改数据时的单线程方案
let shared = Rc::new(RefCell::new(0));

let a = Rc::clone(&shared);
let b = Rc::clone(&shared);

*a.borrow_mut() += 1;
*b.borrow_mut() += 1;

println!("{}", shared.borrow()); // 2
```

---

### 4.6 Mutex\<T\> 与 RwLock\<T\>（多线程可变访问）

```rust
use std::sync::{Mutex, RwLock, Arc};

// Mutex：同一时刻只允许一个线程访问（读写都独占）
let m = Arc::new(Mutex::new(0));

let m_clone = Arc::clone(&m);
let handle = thread::spawn(move || {
    let mut val = m_clone.lock().unwrap(); // 获取锁，返回 MutexGuard
    *val += 1;
    // MutexGuard 离开作用域自动释放锁
});
handle.join().unwrap();
println!("{}", *m.lock().unwrap()); // 1

// RwLock：允许多个并发读，或独占一个写（读多写少时性能更好）
let rw = Arc::new(RwLock::new(vec![1, 2, 3]));

// 多个读锁可以同时持有
let r1 = rw.read().unwrap();
let r2 = rw.read().unwrap();
println!("{:?} {:?}", *r1, *r2);
drop(r1); drop(r2);

// 写锁独占
let mut w = rw.write().unwrap();
w.push(4);
```

---

### 4.7 Arc\<Mutex\<T\>\>（最常用多线程共享可变数据模式）

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let h = thread::spawn(move || {
        *counter.lock().unwrap() += 1; // 加锁、修改、自动释放
    });
    handles.push(h);
}

for h in handles { h.join().unwrap(); }
println!("结果: {}", *counter.lock().unwrap()); // 10
```

**智能指针选择速查：**
```
单线程，唯一所有权，栈太大       → Box<T>
单线程，共享不可变               → Rc<T>
单线程，共享且可变               → Rc<RefCell<T>>
多线程，共享不可变               → Arc<T>
多线程，共享且可变               → Arc<Mutex<T>>
多线程，读多写少                 → Arc<RwLock<T>>
```

---

## 五、并发

### 5.1 线程（thread）

```rust
use std::thread;
use std::time::Duration;

// 创建线程
let handle = thread::spawn(|| {         // 闭包在新线程中运行
    println!("子线程");
    thread::sleep(Duration::from_millis(100));
});

// 等待线程结束
handle.join().unwrap();                 // 阻塞直到该线程结束

// move 闭包：将所有权转移进线程（因为新线程生命周期可能更长）
let v = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", v);                // v 的所有权移入线程
});
handle.join().unwrap();
```

---

### 5.2 消息传递（Channel）

```rust
use std::sync::mpsc;  // multi-producer, single-consumer

// mpsc channel：多发送者，单接收者
let (tx, rx) = mpsc::channel();

let tx_clone = tx.clone();             // 可以克隆发送端
thread::spawn(move || {
    tx.send("第一条消息").unwrap();
});
thread::spawn(move || {
    tx_clone.send("第二条消息").unwrap();
});

// 接收（阻塞直到有消息）
let msg1 = rx.recv().unwrap();
let msg2 = rx.recv().unwrap();
println!("{msg1} {msg2}");

// 迭代接收（直到所有发送端 drop）
for msg in rx { println!("{msg}"); }
```

---

### 5.3 Send 与 Sync trait

```rust
// Send：类型可以安全地在线程间转移所有权
// Sync：类型可以安全地被多线程共享引用（&T 是 Send）
// 几乎所有基础类型自动实现；Rc、RefCell、裸指针不实现

// Arc<T> 要求 T: Send + Sync
// Mutex<T> 实现 Send（即使 T 不是 Send）
// 违反规则的代码不会通过编译
```

---

### 5.4 async/await 与 Tokio

**核心概念：**
```
Future：代表一个尚未完成的计算，实现 .poll() 方法
async fn：返回一个 Future（惰性，不调用则不执行）
await：等待 Future 完成（让出控制权给运行时，不阻塞线程）
运行时（Runtime）：驱动 Future 执行，Tokio 是最流行的运行时
```

```rust
// Cargo.toml 添加：tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

// async fn 返回 impl Future<Output = T>
async fn fetch_data(id: u32) -> String {
    sleep(Duration::from_millis(100)).await;  // 异步等待（不阻塞线程）
    format!("data_{id}")
}

// #[tokio::main] 创建运行时并在其中执行 async main
#[tokio::main]
async fn main() {
    let result = fetch_data(1).await;         // .await 等待完成
    println!("{result}");
}
```

```rust
// 并发执行多个 Future（不是顺序等待）
use tokio::join;

#[tokio::main]
async fn main() {
    // join! 并发运行，等全部完成
    let (a, b, c) = join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3),
    );
    println!("{a} {b} {c}");

    // spawn 产生独立任务（类似线程，但更轻量）
    let handle = tokio::spawn(async {
        fetch_data(4).await
    });
    let result = handle.await.unwrap();
    println!("{result}");
}
```

```rust
// select! 等待多个 Future，哪个先完成用哪个（其余取消）
use tokio::select;

select! {
    result = fetch_data(1) => println!("1先完成: {result}"),
    result = fetch_data(2) => println!("2先完成: {result}"),
}
```

```rust
// 异步 channel（tokio::sync::mpsc）
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);   // 有界 channel，容量 32

    tokio::spawn(async move {
        tx.send("hello").await.unwrap();
    });

    while let Some(msg) = rx.recv().await {
        println!("{msg}");
    }
}
```

**同步 vs 异步 选择原则：**
```
CPU 密集型任务（计算、压缩）      → 普通线程（thread::spawn）
IO 密集型任务（网络、文件、DB）   → async/await + Tokio
需要并行利用多核                  → rayon（数据并行库）或多线程
简单的后台任务                    → thread::spawn 足够
```

---

## 六、常用语法补充

### 6.1 闭包（Closure）

```rust
// 基本语法：|参数| 表达式
let add = |x, y| x + y;
println!("{}", add(1, 2));      // 3

// 捕获环境变量
let offset = 10;
let add_offset = |x| x + offset; // 不可变借用 offset
println!("{}", add_offset(5));   // 15

// move 闭包：转移所有权（常用于线程）
let s = String::from("hello");
let print_s = move || println!("{s}");  // s 的所有权移入闭包
print_s();

// 闭包作为函数参数（Fn、FnMut、FnOnce）
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
fn apply_once<F: FnOnce() -> String>(f: F) -> String { f() }
fn apply_mut<F: FnMut()>(mut f: F) { f(); f(); }

// Fn：可多次调用，不可变捕获
// FnMut：可多次调用，可变捕获
// FnOnce：只能调用一次（转移了捕获变量的所有权）
```

---

### 6.2 模式匹配（Pattern Matching）

```rust
// match：穷举所有分支（编译器强制覆盖所有情况）
let x = 3;
match x {
    1       => println!("one"),
    2 | 3   => println!("two or three"),  // | 表示或
    4..=10  => println!("4 to 10"),       // 范围
    _       => println!("other"),          // 通配符
}

// 解构结构体
struct Point { x: i32, y: i32 }
let p = Point { x: 1, y: 2 };
let Point { x, y } = p;         // 解构绑定
println!("{x} {y}");

// 解构枚举
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
match msg {
    Message::Quit            => println!("退出"),
    Message::Move { x, y }  => println!("移动到 {x},{y}"),
    Message::Write(text)     => println!("写入: {text}"),
}

// if let：只关心一个分支时
if let Some(v) = opt { println!("{v}"); }

// while let：循环直到不匹配
while let Some(top) = stack.pop() { println!("{top}"); }

// @ 绑定：匹配同时绑定值
match x {
    n @ 1..=12  => println!("月份: {n}"),
    _           => println!("无效"),
}

// 守卫（guard）：额外的条件
match pair {
    (x, y) if x == y => println!("相等"),
    (x, y)           => println!("不等: {x} {y}"),
}
```

---

### 6.3 结构体与枚举

```rust
// 普通结构体
#[derive(Debug, Clone)]
struct User {
    username: String,
    email: String,
    active: bool,
}

// 创建与更新语法
let u1 = User { username: "alice".to_string(), email: "a@b.com".to_string(), active: true };
let u2 = User { email: "c@d.com".to_string(), ..u1 }; // 其余字段来自 u1

// 方法（impl 块）
impl User {
    // 关联函数（不带 self，类似"构造函数"）
    fn new(username: &str, email: &str) -> Self {
        User { username: username.to_string(), email: email.to_string(), active: true }
    }
    // 方法（带 &self 或 &mut self）
    fn is_active(&self) -> bool { self.active }
    fn deactivate(&mut self) { self.active = false; }
}

// 枚举（可以携带数据）
#[derive(Debug)]
enum Shape {
    Circle(f64),                    // 元组变体
    Rectangle { width: f64, height: f64 }, // 结构体变体
    Triangle(f64, f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r)              => std::f64::consts::PI * r * r,
            Shape::Rectangle { width: w, height: h } => w * h,
            Shape::Triangle(a, b, c)      => {
                let s = (a + b + c) / 2.0;
                (s * (s-a) * (s-b) * (s-c)).sqrt()
            }
        }
    }
}
```

---

### 6.4 模块与可见性

```rust
// mod 定义模块（可嵌套）
mod network {
    pub mod http {                      // pub：对外可见
        pub fn get(url: &str) -> String {
            format!("GET {url}")
        }
        fn internal() {}               // 无 pub：模块内私有
    }
}

// use 引入路径
use network::http;
use network::http::get;
use std::collections::{HashMap, HashSet}; // 花括号同时引入多个

// 调用
http::get("https://example.com");
get("https://example.com");

// pub(crate)：只在当前 crate 内可见
pub(crate) fn crate_only() {}

// super：引用父模块
mod parent {
    pub fn hello() {}
    mod child {
        fn call() { super::hello(); }  // super 指向 parent
    }
}
```

---

### 6.5 常用宏

```rust
// 打印
println!("{}", x);              // 标准输出 + 换行
print!("{x}");                  // 无换行（支持 {变量名} 简写）
eprintln!("{x:?}");             // 标准错误输出
dbg!(&x);                       // 调试打印，输出文件/行号/值，并返回值

// 断言
assert!(condition);
assert_eq!(a, b);               // 不等则 panic 并打印两者的值
assert_ne!(a, b);

// 构造
vec![1, 2, 3];
format!("{}_{}", a, b);         // → String

// 错误处理（anyhow）
bail!("错误信息 {x}");           // 直接 return Err(...)
ensure!(x > 0, "x 必须为正");   // 条件不满足则 return Err(...)
anyhow!("描述 {x}")              // 构造一个 anyhow::Error

// todo! / unimplemented! / unreachable!
todo!("这里还没实现");           // panic，提醒开发者
unimplemented!();               // 同上，更明确表示"故意不实现"
unreachable!("不应该到这里");    // 到达则 panic
```

---

## 七、所有权·借用·生命周期 常见编译错误速查

```rust
// ❌ 错误1：移动后使用
let s = String::from("hi");
let s2 = s;
println!("{s}"); // use of moved value: `s`
// ✅ 修复：用 clone，或传引用

// ❌ 错误2：可变与不可变借用同时存在
let mut v = vec![1, 2];
let r = &v[0];
v.push(3);      // cannot borrow `v` as mutable because also borrowed as immutable
println!("{r}");
// ✅ 修复：在 push 之前不再使用 r，或不持有引用

// ❌ 错误3：悬垂引用
fn dangling() -> &String {  // missing lifetime specifier
    let s = String::from("hi");
    &s  // s 在函数结束时被 drop，引用失效
}
// ✅ 修复：返回 String（转移所有权），不返回引用

// ❌ 错误4：跨线程发送不实现 Send 的类型
let rc = Rc::new(5);
thread::spawn(move || { println!("{rc}"); }); // Rc is not Send
// ✅ 修复：改用 Arc

// ❌ 错误5：在不可变引用上调用可变方法
let v = vec![1, 2, 3];
v.push(4);      // cannot borrow `v` as mutable, as it is not declared as mutable
// ✅ 修复：let mut v = ...
```
