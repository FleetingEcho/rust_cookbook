# Rust 核心概念速查手册

> 覆盖：**内存管理**（栈/堆·所有权·借用·生命周期）·**类型系统**（trait·泛型·dyn）·**错误处理**（Result·Option·?·thiserror·anyhow）·**智能指针**（Box·Rc·Arc·RefCell·Mutex·RwLock）·**并发**（thread·channel·async/tokio）
> 数据类型方法见配套文档《rust_types_cheatsheet.md》

---

## 完整的知识全景图

```
Rust 两个根本决策：不用 GC + 零成本抽象
            │
            ├──────────────────┬──────────────────────┬──────────────────
            │                  │                      │                  │
         内存管理            类型系统              错误处理            unsafe
            │                  │                      │                  │
     ┌──────┴──────┐    ┌──────┴──────┐          ┌────┴────┐        ┌───┴───┐
     │             │    │             │           │         │        │       │
   栈 vs 堆    所有权   多态         泛型        Result   Option   裸指针  FFI
     │             │    │             │           │         │
     │         ┌───┴───┐  │      ┌────┴────┐      ? 运算符  │
     │         │       │  │      │         │      │         │
     │       借用    Copy  │   泛型函数  泛型结构体  │     ok_or
     │         │          │    │         │       │         │
     │     ┌───┴───┐      │  monomorphization  ┌─┴──────────────┐
     │     │       │      │                    │                │
     │   &T   &mut T    trait bound        thiserror/anyhow  Box<dyn Error>
     │         │          │                    │
     │     ┌───┴───┐      │                  from / transpose
     │     │       │      │
     │  生命周期  借用检查  ├──────────────┐
     │     │              │              │
     │   'a标注          trait       impl Trait / dyn Trait
     │                    │              │
     │                Associated     静态派发 vs 动态派发
     │                 types
     │
     ├──────────────────┬──────────────────────┐
     │                  │                      │
   容器选择          智能指针                 并发
     │                  │                      │
   Vec  HashMap      Box   Rc               ┌──┴──────────────┐
   BTreeMap  HashSet  Arc  RefCell          │                 │
   BTreeSet  VecDeque  Mutex  RwLock     线程模型           async
   BinaryHeap          │                   │                  │
     │            Rc<RefCell>           thread            tokio运行时
     │            Arc<Mutex>            channel           join!/select!
     │                  │              Arc<Mutex>              │
  迭代器管线        内存安全保证          │               Send / Sync
  map/filter/collect  Drop/RAII      Mutex poisoning
  Entry API
```


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

**Trait vs TypeScript interface 对比：**

| | interface（TS） | trait（Rust） |
|--|----------------|--------------|
| 可以有默认实现？ | ✅ | ✅ |
| 可以为已有类型实现？ | ❌ | ✅ |
| 关联类型？ | ❌ | ✅ |
| 条件实现？ | ❌ | ✅（`impl<T: Display> Trait for T`） |
| 编译期静态分发？ | ❌ | ✅（泛型 + trait bound） |
| 运行时动态分发？ | ✅ | ✅（`dyn Trait`） |

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

**四大黄金 derive 组合：**
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }          // ① 数据容器标配：可打印 + 可复制 + 可比较

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u32);                     // ② 排序标配：可 .sort() / .min() / .max()

#[derive(Hash, PartialEq, Eq)]
struct UserId(u64);                       // ③ 哈希标配：可做 HashMap / HashSet 的 key

// ④ 错误类型标配：Debug + Display + std::error::Error（见三、错误处理）
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

**三种多态方式对比：**
```
impl Trait / 泛型   → 编译期单态化，零运行时开销，类型编译期确定
dyn Trait          → 运行时虚表查询，支持异构集合，有间接调用开销
enum               → 编译期 match 展开，零运行时开销，变体数量固定（替代 dyn 的首选）
```

**对象安全（Object Safety）**：并非所有 trait 都能做 trait object：
```rust
// ❌ 不对象安全：方法返回 Self，或有泛型方法
trait Clone { fn clone(&self) -> Self; }  // 不能 dyn Clone

// ✅ 对象安全：方法只用 &self / &mut self，参数/返回值不含 Self 或泛型
trait Drawable { fn draw(&self); }        // 可以 dyn Drawable
```

---

### 2.4 迭代器管线（Iterator）

**三种迭代方式：**

| 方法 | 元素类型 | 原集合 | 用途 |
|------|---------|--------|------|
| `.iter()` | `&T` | 保留 | 只读遍历 |
| `.iter_mut()` | `&mut T` | 保留 | 原地修改 |
| `.into_iter()` | `T` | 消耗 | 获取所有权，链式处理 |

```rust
let v = vec![1, 2, 3, 4, 5];

// iter()：元素是 &i32，filter 闭包再借一次得到 &&i32
let r: Vec<i32> = v.iter().filter(|&&x| x > 2).map(|&x| x * 10).collect();

// into_iter()：元素是 i32，最简洁（新手推荐）
let r: Vec<i32> = v.into_iter().filter(|&x| x > 2).map(|x| x * 10).collect();

// iter_mut()：原地修改
v.iter_mut().for_each(|x| *x *= 10);
```

**消费者（触发计算，返回最终值）：**
```rust
v.iter().count()                        // usize：元素个数
v.iter().sum::<i32>()                   // 求和
v.iter().product::<i32>()               // 求积
v.iter().max() / .min()                 // Option<&T>：最大/最小值
v.iter().any(|&x| x > 3)               // bool：有无满足条件的
v.iter().all(|&x| x > 0)               // bool：是否全部满足
v.iter().find(|&&x| x > 3)             // Option<&T>：第一个满足的元素
v.iter().position(|&x| x == 3)         // Option<usize>：第一个满足的下标
v.iter().fold(0, |acc, &x| acc + x)    // 带初始值的归约（最通用）
```

**适配器（返回新迭代器，惰性不执行）：**
```rust
.map(|x| ...)           // 变换每个元素
.filter(|x| ...)        // 过滤
.filter_map(|x| ...)    // filter + map 合一，None 直接跳过
.flat_map(|x| ...)      // 每个元素展开成多个，再拍平
.flatten()              // 展开嵌套：Vec<Vec<T>> → Vec<T>
.take(n) / .skip(n)    // 取前 n / 跳过前 n
.chain(other)           // 拼接两个迭代器
.enumerate()            // 带下标：(usize, T)
.zip(other)             // 两迭代器合并为 (T, U)
```

**collect() 必须有类型提示（同一迭代器可收集成不同容器）：**
```rust
let v: Vec<&str>           = "a,b,c".split(',').collect();
let s: String              = ['h','i'].into_iter().collect();
let set: HashSet<i32>      = vec![1,1,2,3].into_iter().collect();   // 自动去重
let map: HashMap<&str,i32> = vec![("a",1),("b",2)].into_iter().collect();
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

### 4.8 Cow\<T\>（写时复制·按需克隆）

```rust
use std::borrow::Cow;

// Cow<'a, str> 可以持有 &str（借用）或 String（拥有）
// 只在真正需要修改时才克隆，避免不必要的分配
fn ensure_uppercase(s: &str) -> Cow<str> {
    if s.chars().all(|c| c.is_uppercase()) {
        Cow::Borrowed(s)        // 无需修改，直接借用
    } else {
        Cow::Owned(s.to_uppercase()) // 需要修改，才分配新字符串
    }
}

let a = ensure_uppercase("HELLO"); // Borrowed，零分配
let b = ensure_uppercase("hello"); // Owned，分配一次
```

---

**智能指针选择速查：**
```
单线程，唯一所有权，栈太大       → Box<T>
单线程，共享不可变               → Rc<T>
单线程，共享且可变               → Rc<RefCell<T>>
多线程，共享不可变               → Arc<T>
多线程，共享且可变               → Arc<Mutex<T>>
多线程，读多写少                 → Arc<RwLock<T>>
读多写少，按需克隆               → Cow<T>
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

// thread::scope：限定作用域的线程，可借用局部变量（子线程在 scope 结束前必须结束）
let data = vec![1, 2, 3];
thread::scope(|s| {
    s.spawn(|| println!("借用: {:?}", data));  // 无需 move，可直接借用 data
    s.spawn(|| println!("also: {:?}", data));
});  // 所有子线程在此处 join，data 之后仍可用
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

**线程 vs async 对比：**

| | 线程（thread） | async（tokio） |
|--|--------------|--------------|
| 调度 | OS 内核抢占式 | 运行时协作式（.await 让出） |
| 切换成本 | 微秒级（内核态切换） | 纳秒级（函数调用级别） |
| 内存 | 每线程独立栈（MB 级） | 所有任务共享栈 |
| 适合 | CPU 密集、阻塞 IO | IO 密集、大量并发连接 |

**同步 vs 异步 选择原则：**
```
CPU 密集型任务（计算、压缩）      → 普通线程（thread::spawn）
IO 密集型任务（网络、文件、DB）   → async/await + Tokio
需要并行利用多核                  → rayon（数据并行库）或多线程
async 中调用阻塞/CPU 密集代码     → tokio::task::spawn_blocking
简单的后台任务                    → thread::spawn 足够
```

```rust
// async 中不能直接调用阻塞操作，用 spawn_blocking 扔到线程池
let result = tokio::task::spawn_blocking(|| {
    // 耗时的 CPU 计算或同步阻塞 IO
    heavy_computation()
}).await.unwrap();
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

### 6.5 format和print

**打印宏家族：**
```rust
println!("{}", x);              // stdout + 换行
print!("{x}");                  // stdout 无换行
eprintln!("{x:?}");             // stderr + 换行（错误日志用）
eprint!("...");                 // stderr 无换行
format!("{} {}", a, b);        // → String，不打印
write!(buf, "{}", x);          // 写入实现 Write 的目标（文件、Vec<u8>…）
writeln!(buf, "{}", x);        // 同上 + 换行
dbg!(&x);                       // 调试：打印 文件名:行号 变量名 = 值，并返回值
```

**基础占位符：**
```rust
{}          // Display（面向用户的输出，需实现 Display trait）
{:?}        // Debug（面向开发者，需 #[derive(Debug)] 或手动实现）
{:#?}       // Debug 美化缩进（pretty-print，结构体/嵌套容器一目了然）
```

**参数引用方式：**
```rust
println!("{0} {1} {0}", "a", "b");   // 按位置索引：a b a
println!("{x} {y}", x=1, y=2);       // 命名参数（旧式）
let name = "world";
println!("hello {name}");             // 直接捕获外部变量（Rust 1.58+，最常用）
```

**数字进制与科学记数：**
```rust
println!("{:b}",  42);    // 二进制：         101010
println!("{:o}",  42);    // 八进制：          52
println!("{:x}", 255);    // 十六进制小写：    ff
println!("{:X}", 255);    // 十六进制大写：    FF
println!("{:e}", 1234.5); // 科学记数法：      1.2345e3
println!("{:E}", 1234.5); // 科学记数法大写：  1.2345E3
println!("{:#b}", 42);    // 带前缀：          0b101010
println!("{:#x}", 255);   // 带前缀：          0xff
println!("{:+}", 42);     // 强制显示正负号：  +42
```

**宽度与对齐：**
```rust
//              ↓ 填充字符  ↓ 对齐  ↓ 宽度
println!("{:<10}",  "hi"); // 左对齐：  "hi        "
println!("{:>10}",  "hi"); // 右对齐：  "        hi"
println!("{:^10}",  "hi"); // 居中：    "    hi    "
println!("{:*^10}", "hi"); // 居中，* 填充："****hi****"
println!("{:0>5}",  42);   // 右对齐，0 填充："00042"
println!("{:05}",   42);   // 数字零填充简写（同上）："00042"
// 规律：数字默认右对齐，字符串默认左对齐
```

**精度：**
```rust
println!("{:.2}",  3.14159);       // 浮点小数位数：    3.14
println!("{:.5}",  "hello world"); // 字符串最大字符数：hello
println!("{:8.2}", 3.14159);       // 宽度 8，精度 2：  "    3.14"
```

**组合示例：**
```rust
println!("{:+.3e}",  1234.5);  // 带符号科学记数 3 位精度：+1.235e3
println!("{:#010x}", 255);     // 带前缀零填充十六进制：    0x000000ff
println!("{:>10.3}",  3.14);   // 右对齐宽度 10 精度 3：   "     3.140"
```

---

### 6.6 其他常用宏

```rust
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
anyhow!("描述 {x}");             // 构造一个 anyhow::Error

// todo! / unimplemented! / unreachable!
todo!("这里还没实现");           // panic，提醒开发者
unimplemented!();               // 同上，更明确表示"故意不实现"
unreachable!("不应该到这里");    // 到达则 panic
```

---

### 6.7 const / static / type

#### const：编译期常量

`const` 是**关键字**，不需要 `use` 导入。编译时会被内联到每个使用处，没有固定内存地址。

```rust
// ① 模块级（最常见）：任何地方都能用，命名约定全大写
const MAX_SIZE: usize = 1024;
const PI: f64 = 3.141_592_653_589_793;

// ② 函数内部：作用域仅限当前函数（用于避免魔法数字）
fn process(data: &[u8]) {
    const CHUNK: usize = 512;   // 仅在此函数内可见
    for chunk in data.chunks(CHUNK) { let _ = chunk; }
}

// ③ impl 块内：成为类型的关联常量
struct Circle { radius: f64 }
impl Circle {
    const DEFAULT_RADIUS: f64 = 1.0;  // 用 Circle::DEFAULT_RADIUS 访问
    fn unit() -> Self { Circle { radius: Self::DEFAULT_RADIUS } }
}

// ④ trait 中定义（可以有默认值）
trait HasMax {
    const MAX: usize;           // 实现者必须指定
    const MIN: usize = 0;       // 默认值，可覆盖
}
impl HasMax for Vec<u8> {
    const MAX: usize = 65536;
}
```

**跨模块使用 const**（需要路径，不需要 `use`，但 `use` 可以省路径）：
```rust
mod limits {
    pub const TIMEOUT_MS: u64 = 5000;    // pub 才能被外部看见
    pub(crate) const RETRY: u32 = 3;     // 仅 crate 内可见
}

// 直接用路径
let t = limits::TIMEOUT_MS;

// 或者 use 进来简化
use limits::TIMEOUT_MS;
let t = TIMEOUT_MS;
```

---

#### static：全局静态变量

有固定内存地址，程序整个生命周期存在。值必须是**编译期可确定**的（和 `const` 一样），但不会内联。

```rust
// ① 不可变 static（最安全，无需 unsafe）
static GREETING: &str = "hello";
static PRIMES: [u32; 5] = [2, 3, 5, 7, 11];

// ② 可变 static：读写都需要 unsafe（因为多线程下不安全）
static mut COUNTER: u32 = 0;
unsafe { COUNTER += 1; }           // ⚠️ 生产代码尽量避免
unsafe { println!("{COUNTER}"); }

// ③ 安全的全局只初始化一次：OnceLock（Rust 1.70+）
use std::sync::OnceLock;
static CONFIG: OnceLock<String> = OnceLock::new();

fn get_config() -> &'static str {
    CONFIG.get_or_init(|| {
        // 只在第一次调用时执行，之后直接返回已有值
        std::env::var("APP_ENV").unwrap_or_else(|_| "dev".to_string())
    })
}

// ④ 安全的全局懒初始化：LazyLock（Rust 1.80+，推荐）
use std::sync::LazyLock;
static REGEX: LazyLock<String> = LazyLock::new(|| {
    "compiled_value".to_string()    // 第一次访问时才执行
});
// 直接当引用用，无需调用方法
println!("{}", *REGEX);

// ⑤ 安全的全局可变状态：static + Mutex
use std::sync::Mutex;
static GLOBAL_VEC: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn push_global(n: i32) {
    GLOBAL_VEC.lock().unwrap().push(n);   // 线程安全修改
}
```

---

#### type：类型别名

`type` 创建的是**透明别名**——和原类型完全等价，可以互换，不产生新类型（这与 `struct NewType(T)` 不同）。

```rust
// ① 语义化（让代码更易读，但类型检查不变）
type Meters = f64;
type Seconds = f64;
fn speed(d: Meters, t: Seconds) -> f64 { d / t }
// 注意：Meters 和 Seconds 是同一个类型，编译器不阻止你传错

// ② 简化复杂函数签名（最常用）
type Result<T> = std::result::Result<T, MyError>;
type Callback = Box<dyn Fn(i32) -> i32 + Send + Sync>;
type Matrix = Vec<Vec<f64>>;

fn parse(s: &str) -> Result<i32> { Ok(s.parse().unwrap()) }  // 不用写 MyError

// ③ 简化泛型写法
use std::collections::HashMap;
type StrMap<V> = HashMap<String, V>;   // 带泛型参数的别名

let m: StrMap<i32> = HashMap::new();

// ④ trait 中的关联类型（见 2.2 关联类型）
trait Parser {
    type Output;
    fn parse(&self, s: &str) -> Self::Output;
}
```

**三者对比一览：**
```
const   → 编译期内联，无内存地址，任意作用域，适合魔法数字/数组大小/关联常量
static  → 运行时存在，有固定地址，生命周期 'static，适合全局状态/FFI导出
type    → 透明别名，不产生新类型，适合简化复杂签名/语义化命名
```

---

### 6.8 Deref 自动解引用

> 编译器在类型不匹配时自动插入 `*`（解引用），可连续多次，直到类型匹配。


用 as_ref 获得 &T，用 as_deref 获得进一步解引用后的 &U（比如 &str），两者都不移动所有权。
具体对应关系：
想要 Option<&String> → 用 as_ref
想要 Option<&str> → 用 as_deref
两者都不会动原 Option 的所有权，原变量之后还能用。


```rust
// 常见的自动 deref 链：
// &String  →  &str     （String 实现了 Deref<Target=str>）
// &Vec<T>  →  &[T]     （Vec 实现了 Deref<Target=[T]>）
// &Box<T>  →  &T
// &Arc<T>  →  &T

fn takes_str(s: &str) { println!("{s}"); }
fn takes_slice(s: &[i32]) { println!("{:?}", s); }

let owned: String = String::from("hello");
takes_str(&owned);          // ✅ &String → &str，自动 deref

let v: Vec<i32> = vec![1, 2, 3];
takes_slice(&v);            // ✅ &Vec<i32> → &[i32]，自动 deref

let boxed: Box<String> = Box::new(String::from("hi"));
takes_str(&boxed);          // ✅ &Box<String> → &String → &str，两次 deref
```

```rust
// 自定义 Deref（智能指针实现的核心）
use std::ops::Deref;
struct MyBox<T>(T);
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T { &self.0 }
}
let b = MyBox(String::from("hello"));
takes_str(&b);              // ✅ MyBox → String → &str
```

**记住：** `&String` 和 `&str` 不是同一类型，但 `&String` 可以当 `&str` 用；函数参数写 `&str` 比 `&String` 适用范围更广。

---

### 6.9 常用属性（#[...]）

```rust
// ─── derive：自动生成 trait 实现 ───
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default, PartialOrd, Ord)]
struct Point { x: i32, y: i32 }

// ─── 编译条件 ───
#[cfg(test)]                          // 只在 cargo test 时编译
#[cfg(target_os = "windows")]         // 平台条件
#[cfg(feature = "serde")]             // feature flag 条件
#[cfg(debug_assertions)]              // 只在 debug 模式

// ─── 编译器提示 ───
#[allow(dead_code)]                   // 允许未使用的代码（不报警告）
#[allow(unused_variables)]
#[deny(unsafe_code)]                  // 把警告升级为错误（禁止 unsafe）
#[warn(missing_docs)]

// ─── 函数/方法属性 ───
#[inline]                             // 建议编译器内联
#[inline(always)]                     // 强制内联
#[must_use]                           // 返回值必须被使用，否则警告
#[must_use = "忘记使用会导致 XXX"]
#[deprecated(since = "2.0.0", note = "请用 new_fn()")]

// ─── 测试 ───
#[test]                               // 标记为测试函数
#[should_panic]                       // 期望 panic 的测试
#[should_panic(expected = "overflow")] // 期望特定 panic 信息
#[ignore]                             // 跳过（cargo test -- --ignored 才运行）

// ─── 模块/可见性 ───
#[doc(hidden)]                        // 隐藏于文档
#[path = "other_file.rs"]             // 指定模块对应的文件路径
```

---

### 6.10 impl 完整指南

`impl` 是 Rust 中**为类型附加行为**的唯一方式，有多种用法，覆盖了面向对象语言中"方法"、"接口实现"、"条件实现"等所有场景。

---

#### ① 为结构体添加方法（最基本用法）

```rust
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // ── 关联函数（没有 self）：相当于"静态方法"/"构造函数" ──
    fn new(width: f64, height: f64) -> Self {   // Self = Rectangle
        Rectangle { width, height }
    }
    fn square(size: f64) -> Self {
        Rectangle { width: size, height: size }
    }

    // ── 不可变方法（&self）：只读访问 ──
    fn area(&self) -> f64 {
        self.width * self.height
    }
    fn is_square(&self) -> bool {
        self.width == self.height
    }

    // ── 可变方法（&mut self）：修改自身 ──
    fn scale(&mut self, factor: f64) {
        self.width  *= factor;
        self.height *= factor;
    }

    // ── 消耗自身的方法（self）：调用后原值失效 ──
    fn into_tuple(self) -> (f64, f64) {
        (self.width, self.height)
    }
}

// 调用
let mut r = Rectangle::new(3.0, 4.0);  // 关联函数用 ::
println!("{}", r.area());              // 方法用 .
r.scale(2.0);
let (w, h) = r.into_tuple();           // r 消耗，之后不可用
```

---

#### ② 为枚举添加方法（同样用 impl）

```rust
#[derive(Debug)]
enum Direction { North, South, East, West }

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East  => Direction::West,
            Direction::West  => Direction::East,
        }
    }
    fn is_horizontal(&self) -> bool {
        matches!(self, Direction::East | Direction::West)
    }
}

let d = Direction::North;
println!("{:?}", d.opposite()); // South
```

---

#### ③ 实现 trait（impl Trait for Type）

```rust
use std::fmt;

struct Point { x: f64, y: f64 }

// 为 Point 实现标准库 trait
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:.1}, {:.1})", self.x, self.y)
    }
}

// 实现自定义 trait
trait Translate {
    fn translate(&mut self, dx: f64, dy: f64);
    fn translated(mut self, dx: f64, dy: f64) -> Self     // 默认实现
    where Self: Sized {
        self.translate(dx, dy);
        self
    }
}
impl Translate for Point {
    fn translate(&mut self, dx: f64, dy: f64) {
        self.x += dx;
        self.y += dy;
    }
    // translated 使用 trait 的默认实现，无需重写
}

let p = Point { x: 1.0, y: 2.0 };
println!("{p}");   // (1.0, 2.0)  — 用了 Display
```

---

#### ④ 泛型 impl（为泛型类型实现方法）

```rust
struct Stack<T> {
    data: Vec<T>,
}

// impl 和类型定义都要写 <T>
impl<T> Stack<T> {
    fn new() -> Self {
        Stack { data: Vec::new() }
    }
    fn push(&mut self, item: T) { self.data.push(item); }
    fn pop(&mut self) -> Option<T> { self.data.pop() }
    fn peek(&self) -> Option<&T> { self.data.last() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
}

// 只对特定约束的 T 添加额外方法（条件 impl）
impl<T: fmt::Display> Stack<T> {
    fn print_top(&self) {
        match self.peek() {
            Some(v) => println!("top: {v}"),
            None    => println!("empty"),
        }
    }
}

// 使用
let mut s: Stack<i32> = Stack::new();
s.push(1);
s.push(2);
s.print_top(); // top: 2
```

---

#### ⑤ 条件 impl / where 从句

只有当 `T` 满足某些约束时，才为类型实现某个 trait：

```rust
use std::fmt;

struct Wrapper<T>(T);

// 只有当 T 实现了 Display，Wrapper<T> 才实现 Display
impl<T: fmt::Display> fmt::Display for Wrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

// 约束复杂时用 where 从句（可读性更好）
impl<T> Wrapper<T>
where
    T: fmt::Display + fmt::Debug + Clone,
{
    fn describe(&self) -> String {
        format!("display={} debug={:?}", self.0, self.0)
    }
}

println!("{}", Wrapper(42));   // [42]
println!("{}", Wrapper("hi")); // [hi]
// println!("{}", Wrapper(vec![1])); // ❌ Vec 没实现 Display，编译报错
```

---

#### ⑥ 毯子实现（Blanket impl）—— 为"所有满足条件的类型"批量实现

```rust
trait Printable {
    fn print(&self);
}

// 为所有实现了 Display 的类型自动实现 Printable
impl<T: fmt::Display> Printable for T {
    fn print(&self) { println!("{self}"); }
}

// 现在 i32、String、f64、自定义类型…只要实现了 Display 都有 .print()
42.print();
"hello".print();
3.14_f64.print();
```

标准库最著名的毯子实现：`impl<T, U: Into<T>> From<U> for T`，这就是为什么实现 `From` 后 `Into` 自动可用。

---

#### ⑦ 为外部类型实现自定义 trait（孤儿规则）

Rust 的**孤儿规则（Orphan Rule）**：`impl Trait for Type` 中，`Trait` 和 `Type` 至少有一个必须是**当前 crate 定义的**。

```rust
// ✅ 自定义 trait + 外部类型
trait Summarize { fn summary(&self) -> String; }
impl Summarize for Vec<i32> {              // Summarize 是我们定义的
    fn summary(&self) -> String {
        format!("Vec with {} items", self.len())
    }
}

// ✅ 外部 trait + 自定义类型
struct MyNum(i32);
impl fmt::Display for MyNum {             // MyNum 是我们定义的
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyNum({})", self.0)
    }
}

// ❌ 外部 trait + 外部类型：编译报错
// impl fmt::Display for Vec<i32> {}       // 两个都不是本 crate 的
```

---

#### ⑧ impl Trait 在函数参数和返回值中

```rust
// 参数位置：impl Trait 是泛型的语法糖
fn print_it(item: &impl fmt::Display) {   // 等价于 fn print_it<T: Display>(item: &T)
    println!("{item}");
}

// 多个参数用同一 impl Trait：它们可以是不同的具体类型
fn compare(a: &impl fmt::Display, b: &impl fmt::Display) { println!("{a} vs {b}"); }

// 如果要求两个参数是同一类型，必须用泛型：
fn same_type<T: fmt::Display>(a: &T, b: &T) { println!("{a} {b}"); }

// ─────────────────────────────────────────────

// 返回位置：隐藏具体类型（调用方只知道它实现了某 trait）
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y                       // 返回闭包，不用写闭包的具体类型
}
let add5 = make_adder(5);
println!("{}", add5(3));  // 8

// ⚠️ 限制：返回位置的 impl Trait 只能返回一种具体类型（编译期确定）
fn bad(flag: bool) -> impl fmt::Display {
    if flag { "str" } else { 42 }  // ❌ 两个分支类型不同，编译报错
}
// 解决：用 Box<dyn Trait>（动态派发）
fn good(flag: bool) -> Box<dyn fmt::Display> {
    if flag { Box::new("str") } else { Box::new(42) }  // ✅
}
```

---

#### ⑨ 多个 impl 块（同一类型可以有多个）

```rust
struct Matrix { data: Vec<Vec<f64>> }

// 可以把方法按功能分组写在不同 impl 块里（编译器会合并）
impl Matrix {
    fn new(rows: usize, cols: usize) -> Self {
        Matrix { data: vec![vec![0.0; cols]; rows] }
    }
}

impl Matrix {
    fn rows(&self) -> usize { self.data.len() }
    fn cols(&self) -> usize { self.data.first().map_or(0, |r| r.len()) }
}

// 同一类型多次 impl 同一 trait 是不允许的，但可以 impl 不同 trait
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data { writeln!(f, "{:?}", row)?; }
        Ok(())
    }
}
```

---

#### ⑩ Self 关键字

在 `impl` 块内，`Self`（大写）始终指代**当前正在实现的具体类型**：

```rust
#[derive(Clone)]
struct Config { debug: bool, level: u32 }

impl Config {
    // Self 代替 Config，重构时改类型名不用改方法内部
    fn new() -> Self { Self { debug: false, level: 1 } }
    fn with_debug(mut self) -> Self { self.debug = true; self }
    fn with_level(mut self, level: u32) -> Self { self.level = level; self }
}

// Builder 模式（方法链）
let cfg = Config::new()
    .with_debug()
    .with_level(3);
```

---

**impl 用法速查：**

```
impl Type { }                       → 为类型添加方法（关联函数 / &self / &mut self / self）
impl Trait for Type { }             → 为类型实现某个 trait
impl<T> Type<T> { }                 → 泛型类型的 impl
impl<T: Bound> Type<T> { }          → 条件：只有 T 满足约束时才有这些方法
impl<T: Bound> Trait for Type<T> { }→ 条件 trait 实现
impl<T: Bound> Trait for T { }      → 毯子实现：为所有满足约束的类型批量实现
fn f(x: impl Trait)                 → 参数位置 impl Trait（泛型语法糖）
fn f() -> impl Trait                → 返回位置 impl Trait（隐藏具体类型）
```

---

## 七、实践速查

### 7.1 看返回值类型决定怎么用

```
方法返回值
├── 直接值（String / usize / bool）   → 直接用
├── Option<T>                         → unwrap_or / if let / match / ?（返回 Option 的函数中）
├── Result<T, E>                      → ? / unwrap_or / match
├── &T / &str                         → 注意生命周期，跨作用域存活要 .to_string() / .clone()
├── impl Iterator                     → 惰性，链式处理后 .collect() / .sum() / .count()
└── impl Trait                        → 能用不能命名，想存储用 Box<dyn Trait>
```

```rust
// Option → Result：有 None 则 Err
opt.ok_or("没有值")          // Result<T, &str>
opt.ok_or_else(|| compute()) // 错误值懒求值

// Result → Option：丢弃错误
res.ok()   // Ok → Some，Err → None
res.err()  // Err → Some，Ok → None

// 在返回 Option 的函数里处理 Result
fn first_number(s: &str) -> Option<i32> {
    s.split(',').next()?.parse().ok()  // 找不到或解析失败都返回 None
}
```

---

### 7.2 类型推导规律

```rust
// 函数体内基本不用写，函数签名必须写
fn add(x: i32, y: i32) -> i32 { x + y }  // 签名：必须写

let name = "hello";       // 推导为 &str
let nums = vec![1, 2, 3]; // 推导为 Vec<i32>

// 必须手动指定的情况：
let r: Vec<&str> = "a,b".split(',').collect();  // collect 容器不唯一
let n = 1_i64;                                  // 数字默认 i32，需其他类型加后缀
let mut v: Vec<i32> = Vec::new();               // 空容器，后续无 push 则无法推导

// _ 占位让编译器填具体类型
let map: HashMap<_, _> = vec![("a", 1)].into_iter().collect();
```

---

### 7.3 Vec 存放不同类型

**方式一：enum（推荐，类型安全）**
```rust
enum Value { Int(i32), Str(String), Bool(bool) }

let mut v = vec![Value::Int(1), Value::Str("hi".to_string())];
v.push(Value::Bool(true));

for item in &v {
    match item {
        Value::Int(n)  => println!("int: {n}"),
        Value::Str(s)  => println!("str: {s}"),
        Value::Bool(b) => println!("bool: {b}"),
    }
}
```

**方式二：Box\<dyn Any\>（万不得已，类型信息丢失）**
```rust
use std::any::Any;
let v: Vec<Box<dyn Any>> = vec![Box::new(1_i32), Box::new("hello"), Box::new(true)];
if let Some(n) = v[0].downcast_ref::<i32>() { println!("{n}"); }
```

实际开发 99% 用 enum，`dyn Any` 难以维护，仅极少数通用库使用。

---

## 八、所有权·借用·生命周期 常见编译错误速查

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

---

## 九、遇到问题顺着链条想

```
1. 编译器报 borrow 错
   → 检查：是不是同时有 &mut 和 &？是不是引用比原值活得久？

2. 编译器报 lifetime 错
   → 检查：返回的引用来自哪个参数？函数签名需要加 'a 标注

3. 不知道用什么类型存数据
   → Vec → HashMap/BTreeMap → 自定义 enum → 智能指针

4. 不知道怎么处理错误
   → Option → Result → ? → thiserror（库）→ anyhow（应用）

5. 不知道怎么组织多态代码
   → enum（变体固定）→ 泛型 + trait bound（编译期）→ dyn Trait（运行时）

6. 不知道用线程还是 async
   → CPU 密集 → thread / rayon
   → IO 密集 → async / tokio
   → async 中有阻塞代码 → tokio::task::spawn_blocking

7. 不知道用哪个智能指针
   → 单一所有权 → Box
   → 单线程共享 → Rc（只读）/ Rc<RefCell<T>>（可变）
   → 多线程共享 → Arc（只读）/ Arc<Mutex<T>>（可变）
```
