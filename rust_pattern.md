# Rust 固定写法模式大全

> 涵盖 Trait 实现、生命周期、错误处理、并发、异步、设计模式等常见固定写法

---

## 目录

1. [Trait 实现（基础）](#1-trait-实现基础)
2. [错误处理模式](#2-错误处理模式)
3. [生命周期标注](#3-生命周期标注)
4. [泛型约束](#4-泛型约束)
5. [智能指针模式](#5-智能指针模式)
6. [并发与线程](#6-并发与线程)
7. [异步编程](#7-异步编程)
8. [Builder 模式](#8-builder-模式)
9. [NewType 模式](#9-newtype-模式)
10. [状态机模式](#10-状态机模式)
11. [枚举进阶用法](#11-枚举进阶用法)
12. [闭包与函数指针](#12-闭包与函数指针)
13. [迭代器链式调用](#13-迭代器链式调用)
14. [字符串处理](#14-字符串处理)
15. [集合操作](#15-集合操作)
16. [模块与可见性](#16-模块与可见性)
17. [测试写法](#17-测试写法)
18. [宏编写](#18-宏编写)
19. [文件与IO](#19-文件与io)
20. [序列化（Serde）](#20-序列化serde)
21. [常用惯用法速查](#21-常用惯用法速查)

---

## 1. Trait 实现（基础）

### Display（用户友好输出）

```rust
use std::fmt;

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Package(id={}, name={})", self.id, self.name)
    }
}
```

### Debug（调试输出，优先派生）

```rust
#[derive(Debug)]
struct Package {
    id: u32,
    name: String,
}

// 手写 Debug（自定义格式）
impl fmt::Debug for Package {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Package")
            .field("id", &self.id)
            .field("name", &self.name)
            .finish()
    }
}
```

### Default

```rust
// 方式1：派生（字段全部使用零值）
#[derive(Default)]
struct Config {
    port: u16,      // 0
    enabled: bool,  // false
    name: String,   // ""
}

// 方式2：手写（自定义默认值）
impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8080,
            enabled: true,
            name: "localhost".to_string(),
        }
    }
}

// 使用
let cfg = Config::default();
let cfg = Config { port: 9090, ..Config::default() }; // 部分覆盖
```

### Clone / Copy

```rust
#[derive(Clone, Copy)]  // Copy：栈上按位复制（适合简单类型）
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]        // Clone：深拷贝（含堆数据）
struct Person {
    name: String,
}

// 手写 Clone（需要特殊逻辑）
impl Clone for Buffer {
    fn clone(&self) -> Self {
        Buffer { data: self.data.clone() }
    }
}
```

### PartialEq / Eq / PartialOrd / Ord

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Version {
    major: u32,
    minor: u32,
    patch: u32,
}

// 手写 PartialEq（自定义逻辑）
impl PartialEq for Package {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id  // 只按 id 比较
    }
}
impl Eq for Package {}

// 使用
if v1 > v2 { println!("newer"); }
versions.sort(); // Ord 让排序自动可用
```

### Hash

```rust
use std::hash::{Hash, Hasher};

#[derive(Hash, PartialEq, Eq)]  // Hash 必须配合 Eq
struct UserId(u32);

// 手写 Hash（当 PartialEq 手写时必须手写 Hash）
impl Hash for Package {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);  // 与 eq 的字段保持一致！
    }
}

// 用作 HashMap key
use std::collections::HashMap;
let mut map: HashMap<UserId, String> = HashMap::new();
```

### From / Into / TryFrom / TryInto

```rust
// From：无失败的类型转换
impl From<u32> for UserId {
    fn from(id: u32) -> Self {
        UserId(id)
    }
}

// 实现了 From<T> for U，自动获得 Into<U> for T
let id = UserId::from(42);
let id: UserId = 42.into();

// TryFrom：可能失败的转换
use std::convert::TryFrom;

impl TryFrom<i32> for UserId {
    type Error = String;

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value < 0 {
            Err(format!("负数 ID 无效: {}", value))
        } else {
            Ok(UserId(value as u32))
        }
    }
}

let id = UserId::try_from(-1)?; // 返回 Err
```

### AsRef / AsMut（零拷贝引用转换）

```rust
impl AsRef<str> for MyString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

// 函数接受多种字符串类型的惯用写法
fn print_name(name: impl AsRef<str>) {
    println!("{}", name.as_ref());
}

print_name("hello");           // &str
print_name(String::from("hi")); // String
```

### Iterator

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 实现 Iterator 后自动获得 map/filter/collect 等方法
let sum: u32 = Counter { count: 0, max: 5 }.sum();
```

### IntoIterator（让自定义类型支持 for 循环）

```rust
struct Grid {
    data: Vec<Vec<i32>>,
}

impl IntoIterator for Grid {
    type Item = Vec<i32>;
    type IntoIter = std::vec::IntoIter<Vec<i32>>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

for row in grid { /* ... */ }
```

### Add / Sub / Mul 等运算符重载

```rust
use std::ops::{Add, Sub, Neg};

#[derive(Clone, Copy)]
struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vec2 { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl Neg for Vec2 {
    type Output = Self;
    fn neg(self) -> Self {
        Vec2 { x: -self.x, y: -self.y }
    }
}

// AddAssign（+=）
use std::ops::AddAssign;
impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
```

### Index / IndexMut

```rust
use std::ops::{Index, IndexMut};

struct Matrix {
    data: Vec<Vec<f64>>,
}

impl Index<(usize, usize)> for Matrix {
    type Output = f64;
    fn index(&self, (row, col): (usize, usize)) -> &f64 {
        &self.data[row][col]
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut f64 {
        &mut self.data[row][col]
    }
}

let val = matrix[(0, 1)];
matrix[(0, 1)] = 3.14;
```

### Deref / DerefMut（智能指针解引用）

```rust
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T { &mut self.0 }
}
```

### Drop（析构 / 资源释放）

```rust
struct Connection {
    name: String,
}

impl Drop for Connection {
    fn drop(&mut self) {
        println!("关闭连接: {}", self.name);
        // 释放资源、关闭文件、断开连接等
    }
}

// 提前释放
drop(conn); // 显式调用，而不是等作用域结束
```

---

## 2. 错误处理模式

### 自定义错误类型

```rust
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    ParseError(String),
    IoError(std::io::Error),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::NotFound(msg)   => write!(f, "未找到: {}", msg),
            AppError::ParseError(msg) => write!(f, "解析错误: {}", msg),
            AppError::IoError(e)      => write!(f, "IO错误: {}", e),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::IoError(e) => Some(e),
            _ => None,
        }
    }
}
```

### From 实现：让 ? 自动转换错误

```rust
// 让 std::io::Error 自动转为 AppError
impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e)
    }
}

impl From<std::num::ParseIntError> for AppError {
    fn from(e: std::num::ParseIntError) -> Self {
        AppError::ParseError(e.to_string())
    }
}

// 使用：? 自动触发 From 转换
fn read_port(path: &str) -> Result<u16, AppError> {
    let content = std::fs::read_to_string(path)?; // io::Error -> AppError
    let port = content.trim().parse::<u16>()?;    // ParseIntError -> AppError
    Ok(port)
}
```

### thiserror（推荐库）

```rust
// Cargo.toml: thiserror = "1"
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("未找到: {0}")]
    NotFound(String),

    #[error("IO错误")]
    Io(#[from] std::io::Error),

    #[error("解析失败: {msg}")]
    Parse { msg: String },
}
```

### anyhow（应用层推荐）

```rust
// Cargo.toml: anyhow = "1"
use anyhow::{Context, Result, bail, anyhow};

fn main() -> Result<()> {
    let content = std::fs::read_to_string("config.txt")
        .context("读取配置文件失败")?;

    if content.is_empty() {
        bail!("配置文件为空");
    }

    let n: i32 = content.trim().parse()
        .map_err(|e| anyhow!("解析失败: {}", e))?;

    Ok(())
}
```

### Result 处理固定写法

```rust
// 取值或默认
let val = result.unwrap_or(0);
let val = result.unwrap_or_else(|_| compute_default());
let val = result.unwrap_or_default(); // 用 Default::default()

// 转换错误类型
let val = result.map_err(|e| format!("出错: {}", e))?;

// 转换成功值
let doubled = result.map(|x| x * 2);

// 链式处理
let val = result
    .map(|x| x + 1)
    .and_then(|x| if x > 0 { Ok(x) } else { Err("负数") })
    .unwrap_or(0);

// 忽略错误
let _ = std::fs::remove_file("tmp.txt"); // 明确忽略

// 打印错误并继续
if let Err(e) = do_something() {
    eprintln!("错误: {}", e);
}
```

### Option 处理固定写法

```rust
// 取值
let val = opt.unwrap_or(42);
let val = opt.unwrap_or_else(|| expensive_default());
let val = opt.unwrap_or_default();

// 转换
let doubled = opt.map(|x| x * 2);
let found = opt.filter(|x| *x > 0);
let flat = nested_opt.flatten(); // Option<Option<T>> -> Option<T>

// Option <-> Result 互转
let result = opt.ok_or("没有值")?;
let result = opt.ok_or_else(|| expensive_error())?;
let opt = result.ok(); // Result -> Option（丢弃错误）

// 条件处理
if let Some(x) = opt { println!("{}", x); }
let Some(x) = opt else { return; }; // let-else（Rust 1.65+）

// ? 在返回 Option 的函数中使用
fn first_even(nums: &[i32]) -> Option<i32> {
    let x = nums.first()?; // None 则提前返回 None
    if x % 2 == 0 { Some(*x) } else { None }
}
```

---

## 3. 生命周期标注

### 函数生命周期

```rust
// 返回引用的生命周期必须与某个参数绑定
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// 多个生命周期参数
fn first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();
    for (i, &b) in bytes.iter().enumerate() {
        if b == b' ' { return &s[..i]; }
    }
    &s[..]
}
```

### 结构体生命周期

```rust
// 含引用的结构体必须标注生命周期
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn announce(&self, announcement: &str) -> &str {
        println!("注意: {}", announcement);
        self.part
    }
}
```

### 静态生命周期

```rust
// 'static：整个程序运行期间有效
let s: &'static str = "我是静态字符串";

// 特征对象中的 'static 约束
fn returns_closure() -> Box<dyn Fn(i32) -> i32 + 'static> {
    Box::new(|x| x + 1)
}
```

### 生命周期省略规则（三条规则）

```rust
// 规则1：每个引用参数有自己的生命周期
// 规则2：只有一个输入引用时，输出与其相同
// 规则3：有 &self 时，输出生命周期与 self 相同

// 这些写法等价：
fn first(s: &str) -> &str { &s[..1] }
fn first<'a>(s: &'a str) -> &'a str { &s[..1] }
```

---

## 4. 泛型约束

### 基本约束写法

```rust
// where 子句（推荐，清晰）
fn print_all<T>(items: &[T])
where
    T: fmt::Display + fmt::Debug,
{
    for item in items {
        println!("{:?} -> {}", item, item);
    }
}

// 内联写法（简单情况）
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest { largest = item; }
    }
    largest
}
```

### impl Trait（返回值或参数中使用）

```rust
// 参数位置：语法糖，等同于泛型
fn notify(item: &impl fmt::Display) { println!("{}", item); }

// 返回位置：隐藏具体类型
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

// 注意：返回位置只能有一种具体类型
// 多种类型需用 Box<dyn Trait>
fn make_shape(circle: bool) -> Box<dyn Shape> {
    if circle { Box::new(Circle) } else { Box::new(Square) }
}
```

### 关联类型 vs 泛型

```rust
// 关联类型（每个类型只有一种实现，如 Iterator）
trait Container {
    type Item;
    fn first(&self) -> Option<&Self::Item>;
}

// 泛型（一个类型可以有多种实现）
trait Converter<T> {
    fn convert(&self) -> T;
}
```

### 泛型结构体与 impl

```rust
struct Pair<T> {
    first: T,
    second: T,
}

// 无约束的方法
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

// 有约束的方法
impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("first = {}", self.first);
        } else {
            println!("second = {}", self.second);
        }
    }
}
```

---

## 5. 智能指针模式

### Box<T>（堆分配）

```rust
// 递归数据结构必须用 Box
enum List {
    Cons(i32, Box<List>),
    Nil,
}

let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

// 特征对象
let shapes: Vec<Box<dyn Shape>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),
];
```

### Rc<T>（引用计数，单线程共享）

```rust
use std::rc::Rc;

let a = Rc::new(String::from("hello"));
let b = Rc::clone(&a);  // 增加引用计数，不拷贝数据

println!("引用计数: {}", Rc::strong_count(&a)); // 2
```

### RefCell<T>（内部可变性）

```rust
use std::cell::RefCell;

let data = RefCell::new(vec![1, 2, 3]);

// 运行时借用检查（而非编译时）
data.borrow_mut().push(4);
println!("{:?}", data.borrow());

// Rc<RefCell<T>>：共享且可变（单线程）
use std::rc::Rc;
let shared = Rc::new(RefCell::new(0));
let clone = Rc::clone(&shared);
*clone.borrow_mut() += 10;
```

### Arc<T> + Mutex<T>（多线程共享可变数据）

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let c = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = c.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for h in handles { h.join().unwrap(); }
println!("结果: {}", *counter.lock().unwrap());
```

### Cell<T>（Copy 类型的内部可变性）

```rust
use std::cell::Cell;

struct Config {
    debug: Cell<bool>,
    name: String,
}

let cfg = Config { debug: Cell::new(false), name: "app".into() };
cfg.debug.set(true);
println!("{}", cfg.debug.get());
```

---

## 6. 并发与线程

### 基本线程

```rust
use std::thread;
use std::time::Duration;

// 创建线程
let handle = thread::spawn(|| {
    println!("子线程");
});
handle.join().unwrap(); // 等待完成

// 移动所有权到线程
let data = vec![1, 2, 3];
let handle = thread::spawn(move || {
    println!("{:?}", data);
});
handle.join().unwrap();
```

### 消息传递（mpsc 通道）

```rust
use std::sync::mpsc;

// 单生产者单消费者
let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send("消息").unwrap();
});

let msg = rx.recv().unwrap();

// 多生产者
let tx2 = tx.clone();

// 遍历消息（直到通道关闭）
for msg in rx {
    println!("{}", msg);
}
```

### RwLock（读写锁）

```rust
use std::sync::{Arc, RwLock};

let lock = Arc::new(RwLock::new(5));

// 多个读锁可同时持有
let r1 = lock.read().unwrap();
let r2 = lock.read().unwrap();
println!("{} {}", *r1, *r2);
drop((r1, r2));

// 写锁排他
let mut w = lock.write().unwrap();
*w += 1;
```

### Once（只执行一次的初始化）

```rust
use std::sync::Once;

static INIT: Once = Once::new();
static mut CONFIG: Option<String> = None;

fn get_config() -> &'static str {
    INIT.call_once(|| {
        unsafe { CONFIG = Some("初始化配置".to_string()); }
    });
    unsafe { CONFIG.as_deref().unwrap() }
}
```

---

## 7. 异步编程

### 基本 async/await

```rust
// Cargo.toml: tokio = { version = "1", features = ["full"] }
use tokio::time::{sleep, Duration};

async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data("https://example.com").await?;
    println!("{}", data);
    Ok(())
}
```

### 并发执行（join!）

```rust
use tokio::join;

async fn main_task() {
    let (r1, r2) = join!(task1(), task2());
    // 两个任务并发执行，都完成后继续
}
```

### select!（等待最先完成的）

```rust
use tokio::select;

async fn main_task() {
    select! {
        result = task1() => println!("task1 完成: {:?}", result),
        result = task2() => println!("task2 完成: {:?}", result),
    }
}
```

### spawn（后台任务）

```rust
let handle = tokio::spawn(async {
    sleep(Duration::from_secs(1)).await;
    42
});
let result = handle.await.unwrap();
```

### 异步 Trait（async-trait）

```rust
// Cargo.toml: async-trait = "0.1"
use async_trait::async_trait;

#[async_trait]
trait DataFetcher {
    async fn fetch(&self, id: u32) -> Result<String, String>;
}

struct HttpFetcher;

#[async_trait]
impl DataFetcher for HttpFetcher {
    async fn fetch(&self, id: u32) -> Result<String, String> {
        Ok(format!("data_{}", id))
    }
}
```

---

## 8. Builder 模式

```rust
#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    timeout: u32,
    headers: Vec<(String, String)>,
}

#[derive(Default)]
struct RequestBuilder {
    url: String,
    method: String,
    timeout: u32,
    headers: Vec<(String, String)>,
}

impl RequestBuilder {
    pub fn new() -> Self {
        RequestBuilder {
            method: "GET".to_string(),
            timeout: 30,
            ..Default::default()
        }
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = url.into();
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    pub fn timeout(mut self, secs: u32) -> Self {
        self.timeout = secs;
        self
    }

    pub fn header(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.headers.push((key.into(), val.into()));
        self
    }

    pub fn build(self) -> Result<Request, String> {
        if self.url.is_empty() {
            return Err("URL 不能为空".to_string());
        }
        Ok(Request {
            url: self.url,
            method: self.method,
            timeout: self.timeout,
            headers: self.headers,
        })
    }
}

// 使用
let req = RequestBuilder::new()
    .url("https://api.example.com")
    .method("POST")
    .timeout(60)
    .header("Content-Type", "application/json")
    .build()
    .unwrap();
```

---

## 9. NewType 模式

```rust
// 类型安全包装：防止混用不同含义的同类型值
struct Meters(f64);
struct Kilograms(f64);

// 不能把 Meters 传给需要 Kilograms 的函数！

// 添加方法
impl Meters {
    pub fn value(&self) -> f64 { self.0 }
    pub fn to_cm(&self) -> f64 { self.0 * 100.0 }
}

// 实现 Display
impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}m", self.0)
    }
}

// 用于绕过孤儿规则（为外部类型实现外部 Trait）
struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

---

## 10. 状态机模式

```rust
// 用类型系统表达状态（编译时保证状态转换合法）
struct Order<S> {
    id: u32,
    state: std::marker::PhantomData<S>,
}

struct Draft;
struct Submitted;
struct Shipped;

impl Order<Draft> {
    pub fn new(id: u32) -> Self {
        Order { id, state: std::marker::PhantomData }
    }

    pub fn submit(self) -> Order<Submitted> {
        println!("订单 {} 已提交", self.id);
        Order { id: self.id, state: std::marker::PhantomData }
    }
}

impl Order<Submitted> {
    pub fn ship(self) -> Order<Shipped> {
        println!("订单 {} 已发货", self.id);
        Order { id: self.id, state: std::marker::PhantomData }
    }
}

// 只有 Draft 可以 submit，只有 Submitted 可以 ship
// 错误的状态转换在编译期报错
let order = Order::<Draft>::new(1)
    .submit()
    .ship();
```

---

## 11. 枚举进阶用法

### 枚举方法

```rust
#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
    Custom(u8, u8, u8),
}

impl Color {
    pub fn to_hex(&self) -> String {
        match self {
            Color::Red          => "#FF0000".to_string(),
            Color::Green        => "#00FF00".to_string(),
            Color::Blue         => "#0000FF".to_string(),
            Color::Custom(r, g, b) => format!("#{:02X}{:02X}{:02X}", r, g, b),
        }
    }

    pub fn is_primary(&self) -> bool {
        matches!(self, Color::Red | Color::Green | Color::Blue)
    }
}
```

### 枚举作为错误码

```rust
#[derive(Debug, PartialEq)]
enum Status {
    Ok,
    NotFound,
    Unauthorized,
    InternalError(String),
}

impl Status {
    pub fn code(&self) -> u16 {
        match self {
            Status::Ok              => 200,
            Status::NotFound        => 404,
            Status::Unauthorized    => 401,
            Status::InternalError(_)=> 500,
        }
    }
}
```

### matches! 宏

```rust
let x = Some(42);
assert!(matches!(x, Some(n) if n > 0));

let status = Status::NotFound;
if matches!(status, Status::NotFound | Status::Unauthorized) {
    println!("客户端错误");
}
```

---

## 12. 闭包与函数指针

### 闭包类型

```rust
// Fn：不可变借用捕获（可多次调用）
fn apply<F: Fn(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }

// FnMut：可变借用捕获（可多次调用，但需要 mut）
fn apply_mut<F: FnMut(i32) -> i32>(mut f: F, x: i32) -> i32 { f(x) }

// FnOnce：移动捕获（只能调用一次）
fn apply_once<F: FnOnce(i32) -> i32>(f: F, x: i32) -> i32 { f(x) }
```

### 返回闭包

```rust
// 必须用 Box<dyn Fn> 或 impl Fn
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
```

### 函数指针

```rust
fn double(x: i32) -> i32 { x * 2 }

// fn 类型（不捕获）
let f: fn(i32) -> i32 = double;
let result = f(5);

// 函数指针作为参数
fn transform(data: &[i32], f: fn(i32) -> i32) -> Vec<i32> {
    data.iter().map(|&x| f(x)).collect()
}
```

---

## 13. 迭代器链式调用

```rust
let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

// 常用组合
let result: Vec<i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .take(3)
    .collect();

// 展开嵌套
let nested = vec![vec![1, 2], vec![3, 4]];
let flat: Vec<i32> = nested.into_iter().flatten().collect();

// 分区
let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
    .iter()
    .partition(|&&x| x % 2 == 0);

// 去重（需先排序）
let mut v = vec![3, 1, 2, 1, 3];
v.sort();
v.dedup();

// zip（合并两个迭代器）
let names = vec!["Alice", "Bob"];
let scores = vec![95, 87];
let pairs: Vec<_> = names.iter().zip(scores.iter()).collect();

// scan（带状态的 map）
let running_sum: Vec<i32> = numbers.iter()
    .scan(0, |acc, &x| { *acc += x; Some(*acc) })
    .collect();

// chain（连接迭代器）
let a = vec![1, 2];
let b = vec![3, 4];
let combined: Vec<i32> = a.iter().chain(b.iter()).copied().collect();

// fold（通用归约）
let sum = numbers.iter().fold(0, |acc, &x| acc + x);

// find / position
let first_even = numbers.iter().find(|&&x| x % 2 == 0);
let pos = numbers.iter().position(|&x| x == 5);

// any / all
let has_negative = numbers.iter().any(|&x| x < 0);
let all_positive = numbers.iter().all(|&x| x > 0);

// collect 到不同类型
let set: std::collections::HashSet<i32> = numbers.iter().copied().collect();
let map: std::collections::HashMap<i32, i32> = numbers.iter()
    .map(|&x| (x, x * x))
    .collect();
```

---

## 14. 字符串处理

```rust
// &str vs String
let s: &str = "静态字符串";           // 借用，不拥有
let s: String = String::from("堆字符串"); // 拥有

// 常见转换
let owned: String = "hello".to_string();
let owned: String = "hello".to_owned();
let borrowed: &str = &owned;
let borrowed: &str = owned.as_str();

// 拼接
let s = format!("{} {}", "hello", "world");
let s = ["hello", " ", "world"].concat();
let s = ["a", "b", "c"].join(", ");

// 查找与分割
let contains = s.contains("ell");
let starts = s.starts_with("hel");
let ends = s.ends_with("rld");
let idx = s.find("ll").unwrap_or(0);

let parts: Vec<&str> = s.split(',').collect();
let parts: Vec<&str> = s.splitn(3, ',').collect();

// 修剪
let trimmed = s.trim();
let trimmed = s.trim_start();
let trimmed = s.trim_end();
let trimmed = s.trim_matches('"');

// 替换
let replaced = s.replace("hello", "hi");
let replaced = s.replacen("l", "L", 1); // 只替换第一个

// 大小写
let upper = s.to_uppercase();
let lower = s.to_lowercase();

// 解析
let n: i32 = "42".parse().unwrap();
let n: i32 = "42".parse::<i32>().unwrap();

// 遍历
for c in s.chars() { /* 字符 */ }
for b in s.bytes() { /* 字节 */ }
for line in s.lines() { /* 行 */ }

// 字节切片与字符串
let bytes: &[u8] = s.as_bytes();
let s = std::str::from_utf8(bytes).unwrap();
let s = String::from_utf8(bytes.to_vec()).unwrap();
```

---

## 15. 集合操作

```rust
use std::collections::{HashMap, HashSet, BTreeMap, VecDeque, BinaryHeap};

// HashMap
let mut map: HashMap<String, i32> = HashMap::new();
map.insert("a".to_string(), 1);
map.entry("b".to_string()).or_insert(0);     // 不存在时插入
map.entry("b".to_string()).or_insert_with(|| expensive()); // 懒计算
*map.entry("a".to_string()).or_insert(0) += 1; // 计数器惯用法

let val = map.get("a");
let val = map.get_mut("a");
map.remove("a");
let contains = map.contains_key("b");

// 遍历
for (k, v) in &map { println!("{}: {}", k, v); }
for (k, v) in &mut map { *v += 1; }
let keys: Vec<_> = map.keys().collect();
let vals: Vec<_> = map.values().collect();

// HashSet
let mut set: HashSet<i32> = HashSet::new();
set.insert(1);
set.remove(&1);
let contains = set.contains(&1);

let a: HashSet<_> = [1, 2, 3].iter().collect();
let b: HashSet<_> = [2, 3, 4].iter().collect();
let union: HashSet<_> = a.union(&b).collect();
let inter: HashSet<_> = a.intersection(&b).collect();
let diff: HashSet<_> = a.difference(&b).collect();

// BTreeMap（有序）
let mut btree: BTreeMap<i32, &str> = BTreeMap::new();
for (k, v) in btree.range(1..=10) { /* 范围查询 */ }

// VecDeque（双端队列）
let mut deque: VecDeque<i32> = VecDeque::new();
deque.push_front(1);
deque.push_back(2);
deque.pop_front();
deque.pop_back();

// BinaryHeap（最大堆）
let mut heap: BinaryHeap<i32> = BinaryHeap::new();
heap.push(3);
heap.push(1);
heap.push(4);
let max = heap.pop(); // Some(4)
```

---

## 16. 模块与可见性

```rust
// src/lib.rs 或 src/main.rs
mod utils {
    pub fn helper() {}  // pub：对外公开

    pub(crate) fn internal() {} // 仅在 crate 内公开
    pub(super) fn parent_only() {} // 仅父模块可见

    fn private() {} // 私有（默认）
}

// 使用
use utils::helper;
use utils::{helper, internal};
use utils::*; // 不推荐

// 重导出
pub use utils::helper; // 让外部直接访问

// 文件模块
// src/
// ├── main.rs
// ├── models/
// │   ├── mod.rs    <- mod models; 的入口
// │   └── user.rs   <- pub mod user; 在 mod.rs 中声明
// └── utils.rs      <- mod utils; 即可使用
```

---

## 17. 测试写法

```rust
// 单元测试（与代码同文件）
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn test_with_message() {
        assert_eq!(result, expected, "不符合预期: {:?}", result);
    }

    #[test]
    #[should_panic(expected = "除零")]
    fn test_panic() {
        divide(1, 0);
    }

    #[test]
    fn test_result() -> Result<(), String> {
        let val = parse_number("42")?;
        assert_eq!(val, 42);
        Ok(())
    }

    #[ignore]
    #[test]
    fn slow_test() {
        // cargo test -- --ignored 才运行
    }
}

// 集成测试（tests/ 目录）
// tests/integration_test.rs
use my_crate::public_api;

#[test]
fn test_public_api() {
    assert!(public_api().is_ok());
}

// 文档测试
/// 计算两数之和
///
/// # 示例
/// ```
/// let result = my_crate::add(2, 3);
/// assert_eq!(result, 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 { a + b }
```

---

## 18. 宏编写

### 声明宏（macro_rules!）

```rust
// 简单宏
macro_rules! say_hello {
    () => { println!("Hello!"); };
    ($name:expr) => { println!("Hello, {}!", $name); };
}

say_hello!();
say_hello!("Alice");

// 可变参数宏
macro_rules! log {
    ($fmt:expr) => { println!("[LOG] {}", $fmt); };
    ($fmt:expr, $($arg:expr),*) => {
        println!("[LOG] {}", format!($fmt, $($arg),*));
    };
}

// vec! 类似宏
macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut v = Vec::new();
            $(v.push($x);)*
            v
        }
    };
}

let v = my_vec![1, 2, 3];
```

### 常用元变量类型

```rust
// $name:expr   表达式
// $name:ident  标识符
// $name:ty     类型
// $name:stmt   语句
// $name:pat    模式
// $name:block  代码块
// $name:tt     token 树（最通用）
// $name:literal 字面量
```

---

## 19. 文件与IO

```rust
use std::fs;
use std::io::{self, BufRead, Write};

// 读整个文件
let content = fs::read_to_string("file.txt")?;
let bytes = fs::read("file.bin")?;

// 写文件
fs::write("file.txt", "内容")?;

// 追加写入
use fs::OpenOptions;
let mut file = OpenOptions::new()
    .append(true)
    .open("log.txt")?;
writeln!(file, "新日志行")?;

// 逐行读取（大文件推荐）
use std::io::BufReader;
let file = fs::File::open("file.txt")?;
let reader = BufReader::new(file);
for line in reader.lines() {
    let line = line?;
    println!("{}", line);
}

// 路径操作
use std::path::Path;
let path = Path::new("dir/file.txt");
println!("{:?}", path.extension());    // Some("txt")
println!("{:?}", path.file_name());   // Some("file.txt")
println!("{:?}", path.parent());       // Some("dir")
println!("{}", path.exists());

use std::path::PathBuf;
let mut path = PathBuf::from("/home");
path.push("user");
path.push("file.txt");
// => /home/user/file.txt
```

---

## 20. 序列化（Serde）

```rust
// Cargo.toml:
// serde = { version = "1", features = ["derive"] }
// serde_json = "1"

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    id: u32,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(rename = "created_at")]
    created: String,
    #[serde(skip)]
    password: String,  // 不序列化
}

// 序列化
let user = User { id: 1, name: "Alice".into(), email: None,
                  created: "2024".into(), password: "secret".into() };
let json = serde_json::to_string(&user)?;
let json = serde_json::to_string_pretty(&user)?;

// 反序列化
let user: User = serde_json::from_str(&json)?;
let user: User = serde_json::from_value(value)?;

// 宽松反序列化（未知字段不报错）
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]  // 严格模式
struct StrictConfig { /* ... */ }

// 自定义序列化
#[serde(with = "chrono::serde::ts_seconds")]
created: chrono::DateTime<Utc>,
```

---

## 21. 常用惯用法速查

### 类型转换速查


```rs
let a: u32 = 42;
let b: u64 = a as u64;        // u32 → u64（安全，不会丢失）
let c: u8 = a as u8;           // u32 → u8（可能截断：42 % 256 = 42）
let d: i32 = a as i32;         // u32 → i32（可能溢出符号）

// 浮点数转换
let x = 3.14_f64;
let y = x as f32;               // f64 → f32（可能精度损失）
let z = x as i32;               // f64 → i32（截断小数：3）


// From：类型明确转换（不会失败）
let a: u32 = 42;
let b: u64 = u64::from(a);      // u32 → u64
let c: i32 = i32::from(a);      // 编译错误！u32 可能超过 i32 范围

// Into：自动推导（通常配合类型标注）
let b: u64 = a.into();           // 自动推导为 u64::from(a)
let c: u128 = a.into();          // 自动推导为 u128::from(a)

```

```rust
// 数字转换
let f = 3.14_f64;
let i = f as i32;          // 截断
let i = f.round() as i32;  // 四舍五入

// 字符串 <-> 数字
let s = 42.to_string();
let n: i32 = "42".parse().unwrap();

// Vec <-> slice
let v: Vec<i32> = vec![1, 2, 3];
let s: &[i32] = &v;
let v: Vec<i32> = s.to_vec();

// &str <-> String
let s: String = "hello".to_string();
let s: &str = &string;

// Vec<u8> <-> String
let v = b"hello".to_vec();
let s = String::from_utf8(v).unwrap();
let v = s.into_bytes();
```

### 常用模式匹配

```rust
// 解构
let (a, b, c) = (1, 2, 3);
let Point { x, y } = point;
let [first, .., last] = array.as_slice() else { return; };

// 嵌套解构
let ((a, b), c) = ((1, 2), 3);

// 守卫
match x {
    n if n < 0  => println!("负数"),
    0           => println!("零"),
    n if n < 10 => println!("个位数"),
    _           => println!("大数"),
}

// @ 绑定
match x {
    n @ 1..=9 => println!("个位: {}", n),
    _ => {},
}
```

### 作用域与所有权惯用法

```rust
// 提前结束借用
let result = {
    let guard = mutex.lock().unwrap();
    guard.value.clone()
}; // guard 在这里释放，不占用后续代码

// 影子变量（shadow）
let x = "42";
let x: i32 = x.parse().unwrap(); // 重用变量名，类型改变

// 临时可变性
let v = {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v  // 返回不可变的 v
};
```

### Trait 对象与动态分发

```rust
// dyn Trait（运行时多态）
fn process(items: &[Box<dyn Shape>]) {
    for item in items {
        item.draw(); // 动态分发
    }
}

// 带生命周期的 Trait 对象
fn make_iter<'a>(data: &'a [i32]) -> Box<dyn Iterator<Item = &'a i32> + 'a> {
    Box::new(data.iter())
}

// 对象安全（dyn 要求）：
// - 方法不能有泛型参数
// - 不能返回 Self
// - 第一个参数必须是 &self / &mut self / self
```

---

## 快速记忆表

| 场景 | 写法 |
|------|------|
| 调试输出 | `#[derive(Debug)]` |
| 用户输出 | `impl fmt::Display` |
| 默认值 | `#[derive(Default)]` 或 `impl Default` |
| 深拷贝 | `#[derive(Clone)]` |
| 按位复制 | `#[derive(Clone, Copy)]` |
| 比较相等 | `#[derive(PartialEq, Eq)]` |
| 排序比较 | `#[derive(PartialOrd, Ord)]` |
| HashMap key | `#[derive(Hash, PartialEq, Eq)]` |
| 类型转换 | `impl From<T> for U` |
| 可失败转换 | `impl TryFrom<T> for U` |
| 自定义迭代 | `impl Iterator`（实现 `next`）|
| 运算符 | `impl Add/Sub/Mul...` |
| 解引用 | `impl Deref` |
| 析构 | `impl Drop` |
| 自定义错误 | `impl Display + Error` + `From` |
| 共享所有权 | `Rc<T>` / `Arc<T>` |
| 内部可变 | `RefCell<T>` / `Mutex<T>` |
| Builder | 链式方法返回 `Self` |
| NewType | 元组结构体 `struct Meters(f64)` |
| 状态机 | `PhantomData<State>` |

---

*参考：Rust 官方文档、The Book、Rust Reference、rustlings*