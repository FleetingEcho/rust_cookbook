# Rust 类型系统精讲：struct · enum · trait · impl

> 这四个关键字是 Rust 类型系统的全部骨架。读完本文你将彻底理解它们各自的用法、
> 它们之间如何协作，以及每种写法背后的设计意图。

---

## 目录

- [一、struct：数据的形状](#一struct数据的形状)
- [二、enum：有限状态的集合](#二enum有限状态的集合)
- [三、trait：行为的契约](#三trait行为的契约)
- [四、impl：为类型附加行为](#四impl为类型附加行为)
- [五、四者协作：综合实战](#五四者协作综合实战)

---

## 一、struct：数据的形状

### 1.1 三种写法

```rust
// ① 具名字段结构体（最常用）
struct User {
    name: String,
    age: u32,
    active: bool,
}

// ② 元组结构体（字段没有名字，只有位置）
struct Point(f64, f64);
struct Color(u8, u8, u8);

// ③ 单元结构体（零大小，无字段）
struct Marker;          // 常用于 trait 的标记类型、泛型占位
struct AlwaysEq;
```

### 1.2 创建与访问

```rust
// 具名字段：逐字段赋值
let u = User {
    name: String::from("Alice"),
    age: 30,
    active: true,
};
println!("{}", u.name);   // 字段访问
println!("{}", u.age);

// 简写：变量名与字段名相同时可省略
let name = String::from("Bob");
let age = 25;
let u2 = User { name, age, active: false };  // name: name 可简写成 name

// 结构体更新语法：只改部分字段，其余来自另一个实例
let u3 = User {
    email: String::from("carol@example.com"),  // 假设有 email 字段
    ..u2    // 其余字段从 u2 复制（注意：移动语义！u2 含 String 字段后不可用）
};

// 元组结构体：用 .0 .1 访问
let p = Point(3.0, 4.0);
println!("{} {}", p.0, p.1);

let Color(r, g, b) = Color(255, 128, 0);  // 解构
println!("{r} {g} {b}");
```

### 1.3 解构（Destructuring）

```rust
struct Point { x: f64, y: f64 }

let p = Point { x: 1.0, y: 2.0 };

// 解构绑定
let Point { x, y } = p;
println!("{x} {y}");

// 解构并重命名
let Point { x: px, y: py } = Point { x: 3.0, y: 4.0 };

// 只关心部分字段
let Point { x, .. } = Point { x: 5.0, y: 6.0 };  // .. 忽略其余

// 函数参数中直接解构
fn print_point(&Point { x, y }: &Point) {
    println!("({x}, {y})");
}
```

### 1.4 字段可见性

```rust
// 默认：字段是私有的（模块外不可见）
pub struct Config {
    pub host: String,       // pub：外部可读写
    pub port: u16,
    timeout: u64,           // 无 pub：只在本模块可见
}

// 外部代码只能用 Config::new() 构造，不能直接赋值 timeout
impl Config {
    pub fn new(host: &str, port: u16) -> Self {
        Config { host: host.to_string(), port, timeout: 30 }
    }
    pub fn timeout(&self) -> u64 { self.timeout }  // 通过方法暴露只读访问
}
```

### 1.5 Newtype 模式（元组结构体包装）

> 用一个只有一个字段的元组结构体包装已有类型，目的是**让类型不再混用**或**为外部类型实现 trait**。

```rust
// 问题：Meters 和 Seconds 都是 f64，容易传错
// 解决：用 newtype 让编译器帮你检查
struct Meters(f64);
struct Seconds(f64);

fn speed(dist: Meters, time: Seconds) -> f64 {
    dist.0 / time.0
}

let d = Meters(100.0);
let t = Seconds(9.58);
speed(d, t);       // ✅
// speed(t, d);    // ❌ 编译报错：类型不对，传反了

// ─────────────────────────────────────────────
// newtype 的另一个用途：为外部类型实现 trait（绕过孤儿规则）
use std::fmt;

struct Wrapper(Vec<String>);      // 包装外部类型 Vec

impl fmt::Display for Wrapper {   // ✅ Wrapper 是我们的类型
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

println!("{}", Wrapper(vec!["a".to_string(), "b".to_string()]));
// [a, b]
```

### 1.6 泛型结构体

```rust
// T 是类型参数，创建时具体化
struct Pair<T> {
    first: T,
    second: T,
}

// 持有引用的结构体必须标注生命周期
struct StrSlice<'a> {
    data: &'a str,   // data 的存活时间不能短于 StrSlice 本身
}

// 多个类型参数
struct Map<K, V> {
    key: K,
    value: V,
}
```

### 1.7 常用 derive

```rust
// derive 让编译器自动生成常见 trait 的实现
#[derive(
    Debug,          // {:?} 打印，开发调试必备
    Clone,          // .clone() 深拷贝
    PartialEq,      // == 运算符
    Eq,             // 全序等价（PartialEq 的加强版，无 NaN 问题）
    Hash,           // 可做 HashMap/HashSet 的 key（需同时 PartialEq+Eq）
    Default,        // Config::default() 零值构造
    PartialOrd,     // < > 运算符
    Ord,            // 全序比较，.sort() 需要（需同时 PartialOrd+Eq）
)]
struct Point {
    x: i32,
    y: i32,
}

// 常用组合速查
// 数据容器：      #[derive(Debug, Clone, PartialEq)]
// 可排序类型：    #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
// HashMap key：  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// 配置/默认值：  #[derive(Debug, Clone, Default)]
```

---

## 二、enum：有限状态的集合

### 2.1 三种变体写法

```rust
enum Command {
    // ① 单元变体：无附带数据
    Quit,
    Pause,

    // ② 元组变体：有位置数据
    Move(i32, i32),          // 移动到 (x, y)
    Color(u8, u8, u8),       // RGB

    // ③ 结构体变体：有命名字段
    Resize { width: u32, height: u32 },
    Login { username: String, password: String },
}
```

### 2.2 match：必须穷举所有变体

```rust
fn handle(cmd: Command) {
    match cmd {
        Command::Quit          => println!("退出"),
        Command::Pause         => println!("暂停"),
        Command::Move(x, y)    => println!("移动到 ({x},{y})"),
        Command::Color(r,g,b)  => println!("颜色 #{r:02x}{g:02x}{b:02x}"),
        Command::Resize { width, height } => println!("调整为 {width}x{height}"),
        Command::Login { username, .. }   => println!("登录: {username}"),
    }
    // 编译器强制覆盖所有分支，漏掉一个就报错 → 这是 enum 最大的安全价值
}

// 只关心一个分支：if let
if let Command::Move(x, y) = cmd { println!("{x} {y}"); }

// 只关心一个分支并处理其余：if let + else
if let Command::Quit = cmd {
    println!("quit");
} else {
    println!("not quit");
}

// 多个分支一样的处理：| 合并
match cmd {
    Command::Quit | Command::Pause => println!("停止类指令"),
    _ => {}
}
```

### 2.3 Option\<T\>：Rust 版"可能为空"

`Option<T>` 就是内置的枚举，定义如下：
```rust
// 标准库定义（无需 use，自动引入）
enum Option<T> {
    Some(T),   // 有值
    None,      // 无值
}
```

```rust
// 创建
let a: Option<i32> = Some(42);
let b: Option<i32> = None;

// 取值的各种方式（从最不安全到最安全）
a.unwrap();                     // 有值返回值，None 时 panic ⚠️
a.expect("应该有值");           // 同上但 panic 信息更好
a.unwrap_or(0);                 // None 时给默认值
a.unwrap_or_default();          // None 时用类型的 Default 值
a.unwrap_or_else(|| compute()); // None 时懒求值

// 变换
a.map(|x| x * 2);              // Some(84) — Some 时变换，None 保持 None
a.and_then(|x| if x > 0 { Some(x) } else { None }); // flatMap
a.filter(|x| x > &10);         // 不满足条件变 None

// 组合
a.or(Some(0));                  // a 是 None 时用 Some(0)
a.zip(b);                       // (Some(42), None) → None；两者都 Some → Some((42, _))

// 转 Result
a.ok_or("没有值");              // None → Err("没有值")

// 常用 if let 模式
if let Some(v) = a { println!("{v}"); }

// match 模式
match a {
    Some(v) if v > 10 => println!("大于10: {v}"),  // 守卫条件
    Some(v)           => println!("不大于10: {v}"),
    None              => println!("空"),
}
```

### 2.4 Result\<T, E\>：Rust 版"可能失败"

```rust
// 标准库定义（自动引入）
enum Result<T, E> {
    Ok(T),    // 成功，携带值
    Err(E),   // 失败，携带错误
}
```

```rust
use std::num::ParseIntError;

fn parse(s: &str) -> Result<i32, ParseIntError> {
    s.trim().parse::<i32>()
}

// 取值
let r = parse("42");
r.unwrap();                          // Ok → 值，Err → panic
r.unwrap_or(0);                      // Err 时给默认值
r.unwrap_or_else(|e| { eprintln!("{e}"); 0 });

// 变换
r.map(|n| n * 2);                    // Ok(84)
r.map_err(|e| format!("解析失败: {e}")); // 变换错误类型
r.and_then(|n| if n > 0 { Ok(n) } else { Err("负数".parse::<i32>().unwrap_err()) });

// ? 运算符：Ok 时取值继续，Err 时提前 return
fn double_parse(s: &str) -> Result<i32, ParseIntError> {
    let n = parse(s)?;   // 等价于：match parse(s) { Ok(v) => v, Err(e) => return Err(e.into()) }
    Ok(n * 2)
}

// 转 Option（丢弃错误信息）
r.ok();    // Ok → Some，Err → None
r.err();   // Err → Some，Ok → None
```

### 2.5 枚举的方法

```rust
#[derive(Debug, PartialEq)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    // 关联函数
    fn initial() -> Self { Self::Red }

    // 方法
    fn duration_secs(&self) -> u32 {
        match self {
            Self::Red    => 60,
            Self::Yellow => 5,
            Self::Green  => 45,
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Red    => Self::Green,
            Self::Green  => Self::Yellow,
            Self::Yellow => Self::Red,
        }
    }

    fn is_stop(&self) -> bool {
        matches!(self, Self::Red | Self::Yellow)
    }
}

let mut light = TrafficLight::initial();
println!("{:?} — {}秒", light, light.duration_secs());
light = light.next();  // Green
```

### 2.6 泛型枚举

```rust
// 类似标准库的 Option 和 Result
enum Tree<T> {
    Leaf(T),
    Node { value: T, left: Box<Tree<T>>, right: Box<Tree<T>> },
    //             ^^^ 递归类型必须用 Box，否则大小无限
}

let tree = Tree::Node {
    value: 1,
    left:  Box::new(Tree::Leaf(2)),
    right: Box::new(Tree::Leaf(3)),
};
```

### 2.7 用 enum 建模状态机

```rust
// enum 天然适合表达"只能处于有限状态之一"
#[derive(Debug)]
enum OrderState {
    Pending { created_at: u64 },
    Paid { amount: f64 },
    Shipped { tracking_id: String },
    Delivered,
    Cancelled { reason: String },
}

impl OrderState {
    fn can_cancel(&self) -> bool {
        matches!(self, Self::Pending { .. } | Self::Paid { .. })
    }

    fn pay(self, amount: f64) -> Result<Self, &'static str> {
        match self {
            Self::Pending { .. } => Ok(Self::Paid { amount }),
            _ => Err("只有待支付的订单才能付款"),
        }
    }
}
```

### 2.8 enum vs struct：何时选哪个

```
用 struct：数据的各个字段同时存在（AND 关系）
           User { name, age, email } — 同时有名字、年龄、邮件

用 enum：数据只能是若干形态之一（OR 关系）
          Shape = Circle | Rectangle | Triangle — 只能是其中一种
          Result = Ok | Err                    — 要么成功要么失败
```

---

## 三、trait：行为的契约

### 3.1 定义与实现

```rust
// 定义：声明一组方法签名（+可选的默认实现）
trait Greet {
    // 必须实现的方法（无默认值）
    fn name(&self) -> &str;

    // 有默认实现（可选覆盖）
    fn greeting(&self) -> String {
        format!("Hello, I'm {}!", self.name())
    }
}

// 实现
struct Person { name: String }
struct Robot  { id: u32 }

impl Greet for Person {
    fn name(&self) -> &str { &self.name }
    // greeting 使用默认实现
}

impl Greet for Robot {
    fn name(&self) -> &str { "Robot" }
    fn greeting(&self) -> String {       // 覆盖默认实现
        format!("BEEP BOOP. ID: {}", self.id)
    }
}

// 使用
let p = Person { name: "Alice".to_string() };
let r = Robot { id: 42 };
println!("{}", p.greeting()); // Hello, I'm Alice!
println!("{}", r.greeting()); // BEEP BOOP. ID: 42
```

### 3.2 关联类型（Associated Type）

> 关联类型让 trait 的"输出类型"成为实现的一部分，调用者无需每次都写泛型参数。

```rust
// 用泛型参数的版本（繁琐）
trait ConverterGeneric<T> { fn convert(&self) -> T; }
// 调用方必须写：fn foo<T, C: ConverterGeneric<T>>(c: &C) → 难看

// ─── 用关联类型（推荐）───
trait Converter {
    type Output;                        // 关联类型声明
    fn convert(&self) -> Self::Output;  // 使用 Self::Output
}

struct Celsius(f64);

impl Converter for Celsius {
    type Output = f64;                  // 指定关联类型
    fn convert(&self) -> f64 { self.0 * 9.0 / 5.0 + 32.0 }
}

// 调用方只需写 Converter，不用 Converter<f64>
fn print_converted(c: &impl Converter<Output = f64>) {  // 如果需要约束 Output
    println!("{}", c.convert());
}
```

**关联类型 vs 泛型参数：**
```
泛型参数 trait Foo<T>：同一类型可对不同 T 实现多次（如 From<i32> + From<String>）
关联类型 type Item：同一类型只能实现一次（如 Iterator 的 Item 唯一确定）

经验法则：
  "一种类型只有一种合理的实现"        → 关联类型（Iterator, Deref, Add）
  "同一类型需要对多种目标类型实现"     → 泛型参数（From, Into, PartialEq）
```

### 3.3 关联常量

```rust
trait Shape {
    const DIMENSIONS: u32;             // 关联常量（无默认值，必须实现）
    const NAME: &'static str = "shape"; // 有默认值（可覆盖）

    fn area(&self) -> f64;
}

struct Circle { radius: f64 }
impl Shape for Circle {
    const DIMENSIONS: u32 = 2;
    const NAME: &'static str = "circle";
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
}

println!("{}", Circle::DIMENSIONS); // 通过类型访问
println!("{}", Circle::NAME);
```

### 3.4 Trait Bound：用 trait 约束泛型

```rust
use std::fmt;

// 方式一：冒号语法（简洁，bound 少时用）
fn print<T: fmt::Display>(item: T) { println!("{item}"); }

// 多个 bound：+ 连接
fn print_debug<T: fmt::Display + fmt::Debug>(item: T) {
    println!("{item} / {:?}", item);
}

// 方式二：where 从句（bound 多或复杂时用，更清晰）
fn complex_fn<T, U>(t: T, u: U) -> String
where
    T: fmt::Display + Clone + Send,
    U: fmt::Debug  + Clone + Sync,
{
    format!("{t} {:?}", u)
}

// 方式三：impl Trait（最简洁，函数参数首选）
fn show(item: &impl fmt::Display) { println!("{item}"); }
```

### 3.5 Supertrait：继承另一个 trait

```rust
// 要实现 Animal，必须先实现 fmt::Display
trait Animal: fmt::Display {        // fmt::Display 是 Animal 的 supertrait
    fn name(&self) -> &str;
    fn sound(&self) -> &str;

    fn introduce(&self) {
        // supertrait 的方法在这里可以直接用
        println!("我是 {}，{}", self, self.sound());  // 用了 Display
    }
}

struct Cat;
impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "Cat") }
}
impl Animal for Cat {
    fn name(&self) -> &str { "Cat" }
    fn sound(&self) -> &str { "Meow" }
}

Cat.introduce();  // 我是 Cat，Meow
```

### 3.6 常用标准库 trait 一览

```rust
// ─── 格式化 ───
// Display：面向用户的 {} 输出
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "...") }
}
// Debug：面向开发者的 {:?}，通常直接 derive
#[derive(Debug)] struct Foo;

// ─── 所有权与复制 ───
// Clone：.clone() 显式深拷贝
#[derive(Clone)] struct MyVec(Vec<i32>);
// Copy：赋值/传参时自动位拷贝（只有所有字段都是 Copy 时才能 derive）
#[derive(Clone, Copy)] struct Point { x: f32, y: f32 };

// ─── 比较 ───
// PartialEq：== !=（允许 NaN != NaN 这种情况）
// Eq：全序相等（加在 PartialEq 上保证 a==a 总成立）
#[derive(PartialEq, Eq)] struct Id(u32);
// PartialOrd / Ord：< > <= >=（Ord 要求 Eq）
#[derive(PartialEq, Eq, PartialOrd, Ord)] struct Score(i32);

// ─── 哈希 ───
#[derive(Hash, PartialEq, Eq)] struct UserId(u64);
// 实现 Hash 后可作 HashMap / HashSet 的 key

// ─── 默认值 ───
#[derive(Default)] struct Config { timeout: u32, retries: u32 }
// Config::default() → Config { timeout: 0, retries: 0 }

// ─── 类型转换 ───
// From/Into：无损转换（必然成功）
struct Wrapper(i32);
impl From<i32> for Wrapper {
    fn from(n: i32) -> Self { Wrapper(n) }
}
// From<i32> 自动提供 Into<Wrapper>
let w: Wrapper = 42.into();
let w = Wrapper::from(42);

// TryFrom/TryInto：可能失败的转换
use std::convert::TryFrom;
impl TryFrom<i32> for u8 {
    type Error = &'static str;
    fn try_from(n: i32) -> Result<u8, Self::Error> {
        if n >= 0 && n <= 255 { Ok(n as u8) } else { Err("超出范围") }
    }
}

// ─── 迭代器 ───
// 只需实现 next()，其余 100+ 方法全部免费获得
struct Counter { count: u32 }
impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> {
        self.count += 1;
        if self.count <= 5 { Some(self.count) } else { None }
    }
}
// 现在 Counter 自动拥有 .map() .filter() .sum() .collect() 等

// ─── 运算符重载 ───
use std::ops::Add;
#[derive(Debug, Clone, Copy)]
struct Vec2 { x: f64, y: f64 }
impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}
let v = Vec2 { x: 1.0, y: 2.0 } + Vec2 { x: 3.0, y: 4.0 };
// v = Vec2 { x: 4.0, y: 6.0 }

// ─── Drop：析构时自动执行 ───
struct Resource { name: String }
impl Drop for Resource {
    fn drop(&mut self) { println!("释放资源: {}", self.name); }
}
// Resource 离开作用域时自动调用
```

### 3.7 对象安全（Object Safety）与 dyn Trait

不是所有 trait 都能做 `dyn Trait`，必须满足**对象安全规则**：

```rust
// ❌ 不对象安全的 trait（不能 dyn）
trait NotObjectSafe {
    fn clone_self(&self) -> Self;      // 返回 Self 大小不确定
    fn compare<T>(&self, other: T);   // 泛型方法
}

// ✅ 对象安全的 trait（可以 dyn）
trait Draw {
    fn draw(&self);                    // 只用 &self，不返回 Self，无泛型
    fn bounding_box(&self) -> (f64, f64, f64, f64);
}

// dyn Trait：运行时通过虚表（vtable）动态派发
let shapes: Vec<Box<dyn Draw>> = vec![
    Box::new(Circle { radius: 1.0 }),
    Box::new(Square { side: 2.0 }),   // 不同类型混放
];
for s in &shapes { s.draw(); }

// 如果 trait 不对象安全但你需要异构集合：用 enum 代替
enum Shape { Circle(Circle), Square(Square) }
// enum 是更好的选择（零开销，类型安全）
```

### 3.8 impl Trait vs dyn Trait vs 泛型三者对比

```
泛型 fn foo<T: Draw>(t: &T)
  → 编译期单态化，每种类型生成独立代码
  → 零运行时开销
  → 编译时类型确定，不能在运行时改变
  → 二进制体积可能增大（但 LTO 可优化）

impl Trait fn foo(t: &impl Draw)
  → 是泛型的语法糖，本质相同
  → 更简洁，适合参数只出现一次的场景

dyn Trait fn foo(t: &dyn Draw) / Box<dyn Draw>
  → 运行时通过虚表调用，有一次间接开销
  → 可以存储不同类型的混合集合
  → 类型可在运行时决定

实际建议：
  默认用泛型/impl Trait（零开销）
  需要混合不同类型的集合 → 先考虑 enum，再考虑 dyn Trait
  回调/插件/动态加载 → dyn Trait
```

---

## 四、impl：为类型附加行为

### 4.1 impl Type：自有方法块

```rust
struct Stack<T> { data: Vec<T> }

impl<T> Stack<T> {
    // 关联函数（无 self）：通过 Stack::new() 调用
    pub fn new() -> Self { Stack { data: Vec::new() } }

    // &self：只读，不消耗，可多次调用
    pub fn peek(&self) -> Option<&T> { self.data.last() }
    pub fn len(&self)  -> usize      { self.data.len()  }
    pub fn is_empty(&self) -> bool   { self.data.is_empty() }

    // &mut self：可修改自身，不消耗
    pub fn push(&mut self, item: T) { self.data.push(item); }
    pub fn pop(&mut self) -> Option<T> { self.data.pop() }

    // self（消耗）：调用后原值失效
    pub fn into_vec(self) -> Vec<T> { self.data }
}
```

### 4.2 impl Trait for Type：实现 trait

```rust
use std::fmt;

// 为自定义类型实现标准库 trait
impl<T: fmt::Debug> fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Stack{:?}", self.data)
    }
}

// 为自定义类型实现自定义 trait
trait Clearable { fn clear(&mut self); }
impl<T> Clearable for Stack<T> {
    fn clear(&mut self) { self.data.clear(); }
}
```

### 4.3 泛型 impl + 条件 impl

```rust
// 泛型 impl：T 没有约束，所有 Stack<T> 都有这些方法
impl<T> Stack<T> {
    fn capacity(&self) -> usize { self.data.capacity() }
}

// 条件 impl：只有 T: Clone 时 Stack<T> 才有这些方法
impl<T: Clone> Stack<T> {
    fn duplicate_top(&mut self) {
        if let Some(top) = self.peek().cloned() {
            self.push(top);
        }
    }
}

// 条件 trait 实现：只有 T: PartialEq 时才实现 PartialEq
impl<T: PartialEq> PartialEq for Stack<T> {
    fn eq(&self, other: &Self) -> bool { self.data == other.data }
}
```

### 4.4 毯子实现（Blanket impl）

> 为**所有满足某约束的类型**批量实现 trait，一次覆盖无数类型。

```rust
trait Describable {
    fn describe(&self) -> String;
}

// 为所有实现了 Display 的类型自动实现 Describable
impl<T: fmt::Display> Describable for T {
    fn describe(&self) -> String {
        format!("值为: {self}")
    }
}

// 现在 i32、f64、String、自定义实现了 Display 的类型…都有 .describe()
println!("{}", 42.describe());          // 值为: 42
println!("{}", "hello".describe());     // 值为: hello
println!("{}", 3.14_f64.describe());    // 值为: 3.14
```

### 4.5 孤儿规则（Orphan Rule）

```rust
// 规则：impl Trait for Type 中，Trait 和 Type 至少有一个是本 crate 定义的

// ✅ 本 crate 的 trait + 外部类型
trait MyTrait { fn do_it(&self); }
impl MyTrait for Vec<i32> { fn do_it(&self) { println!("{:?}", self); } }

// ✅ 外部 trait + 本 crate 的类型
struct MyType(i32);
impl fmt::Display for MyType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "{}", self.0) }
}

// ❌ 外部 trait + 外部类型：编译报错！
// impl fmt::Display for Vec<i32> {}   // 两者都不是本 crate 的

// 绕过孤儿规则：用 newtype
struct MyVec(Vec<i32>);
impl fmt::Display for MyVec {           // ✅ MyVec 是本 crate 的
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
```

### 4.6 函数签名中的 impl Trait

```rust
// ─── 参数位置 ───
// 这两种写法完全等价，impl Trait 是语法糖
fn print_a(item: &impl fmt::Display) { println!("{item}"); }
fn print_b<T: fmt::Display>(item: &T) { println!("{item}"); }

// 参数有多个 impl Trait：可以是不同的具体类型
fn mix(a: impl fmt::Display, b: impl fmt::Debug) {
    println!("{a} {:?}", b);
}

// 要求两个参数是同一类型：必须用泛型
fn same<T: fmt::Display>(a: T, b: T) { println!("{a} {b}"); }

// ─── 返回位置 ───
// 隐藏具体类型，调用方只知道"返回某个实现了 Trait 的东西"
fn make_greeting(name: &str) -> impl fmt::Display {
    format!("Hello, {name}!")      // 返回 String，但调用方看不到 String 类型
}

// 常见：返回闭包（闭包类型无法命名）
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}
let add10 = make_adder(10);
println!("{}", add10(5));  // 15

// ⚠️ 返回位置的限制：所有分支必须返回同一具体类型
fn bad(flag: bool) -> impl fmt::Display {
    if flag { return "str"; }
    42   // ❌ 编译报错：&str 和 i32 不是同一类型
}

// 解决：Box<dyn Trait>（动态分发）
fn good(flag: bool) -> Box<dyn fmt::Display> {
    if flag { Box::new("str") } else { Box::new(42) }  // ✅
}
```

### 4.7 Self 关键字

```rust
// Self 始终指代"当前正在实现的类型"，重构时改类型名不用改方法内部
#[derive(Clone)]
struct Builder {
    value: i32,
    name: String,
}

impl Builder {
    fn new() -> Self { Self { value: 0, name: String::new() } }

    // Builder 模式：方法返回 Self，支持链式调用
    fn value(mut self, v: i32) -> Self    { self.value = v; self }
    fn name(mut self, n: &str) -> Self    { self.name = n.to_string(); self }

    fn build(self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

let result = Builder::new()
    .name("count")
    .value(42)
    .build();   // "count=42"
```

### 4.8 多个 impl 块

```rust
// 同一类型可以写多个 impl 块（编译器会合并），常用来按功能分组
struct Image { width: u32, height: u32, pixels: Vec<u8> }

// 构造相关
impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image { width, height, pixels: vec![0; (width * height * 4) as usize] }
    }
}

// 查询相关
impl Image {
    pub fn width(&self)  -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn size(&self)   -> u32 { self.width * self.height }
}

// 操作相关
impl Image {
    pub fn clear(&mut self) { self.pixels.fill(0); }
    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8, a: u8) {
        let idx = ((y * self.width + x) * 4) as usize;
        self.pixels[idx..idx+4].copy_from_slice(&[r, g, b, a]);
    }
}
```

---

## 五、四者协作：综合实战

用一个"图形面积计算器"例子展示四者如何配合：

```rust
use std::fmt;

// ─── 1. trait：定义行为契约 ───
trait Shape: fmt::Display {             // supertrait：实现 Shape 必须实现 Display
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
    fn describe(&self) -> String {      // 默认实现
        format!("{self}：面积={:.2}，周长={:.2}", self.area(), self.perimeter())
    }
}

// ─── 2. struct：定义数据 ───
#[derive(Debug, Clone)]
struct Circle { radius: f64 }

#[derive(Debug, Clone)]
struct Rectangle { width: f64, height: f64 }

#[derive(Debug, Clone)]
struct Triangle { a: f64, b: f64, c: f64 }

// ─── 3. impl：为 struct 实现方法 ───
impl Circle {
    pub fn new(radius: f64) -> Self { Circle { radius } }
}
impl Rectangle {
    pub fn new(width: f64, height: f64) -> Self { Rectangle { width, height } }
}
impl Triangle {
    pub fn new(a: f64, b: f64, c: f64) -> Option<Self> {
        if a + b > c && b + c > a && a + c > b {
            Some(Triangle { a, b, c })
        } else {
            None  // 无效三角形
        }
    }
}

// ─── 4. impl Trait for Struct：实现 trait ───
impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Circle(r={})", self.radius)
    }
}
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rect({}×{})", self.width, self.height)
    }
}
impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Triangle({},{},{})", self.a, self.b, self.c)
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn perimeter(&self) -> f64 { 2.0 * std::f64::consts::PI * self.radius }
}
impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn perimeter(&self) -> f64 { 2.0 * (self.width + self.height) }
}
impl Shape for Triangle {
    fn area(&self) -> f64 {
        let s = (self.a + self.b + self.c) / 2.0;
        (s * (s - self.a) * (s - self.b) * (s - self.c)).sqrt()
    }
    fn perimeter(&self) -> f64 { self.a + self.b + self.c }
}

// ─── 5. enum：表达"是若干图形之一"的状态 ───
#[derive(Debug, Clone)]
enum AnyShape {
    Circle(Circle),
    Rectangle(Rectangle),
    Triangle(Triangle),
}

impl AnyShape {
    fn area(&self) -> f64 {
        match self {
            Self::Circle(c)    => c.area(),
            Self::Rectangle(r) => r.area(),
            Self::Triangle(t)  => t.area(),
        }
    }
}

impl fmt::Display for AnyShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Circle(c)    => write!(f, "{c}"),
            Self::Rectangle(r) => write!(f, "{r}"),
            Self::Triangle(t)  => write!(f, "{t}"),
        }
    }
}

// ─── 6. 泛型函数用 trait bound ───

// 接受任何实现了 Shape 的类型（静态分发，零开销）
fn print_shape_info(shape: &impl Shape) {
    println!("{}", shape.describe());
}

// 接受异构集合（动态分发，可混放不同类型）
fn total_area(shapes: &[Box<dyn Shape>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// ─── 7. 综合使用 ───
fn main() {
    // 用 struct 的关联函数构造
    let c = Circle::new(5.0);
    let r = Rectangle::new(4.0, 6.0);
    let t = Triangle::new(3.0, 4.0, 5.0).expect("有效三角形");

    // 调用 trait 方法
    print_shape_info(&c);
    print_shape_info(&r);
    print_shape_info(&t);

    // dyn Trait：异构集合
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle::new(1.0)),
        Box::new(Rectangle::new(2.0, 3.0)),
        Box::new(Triangle::new(3.0, 4.0, 5.0).unwrap()),
    ];
    println!("总面积: {:.2}", total_area(&shapes));

    // enum：统一处理
    let all = vec![
        AnyShape::Circle(c),
        AnyShape::Rectangle(r),
        AnyShape::Triangle(t),
    ];
    for s in &all { println!("{s} → 面积 {:.2}", s.area()); }
}
```

---

## 速查：四者职责一览

```
struct   → 组织数据（AND 关系：同时有 x、y、z）
enum     → 建模状态（OR 关系：只能是 A、B 或 C 之一）
trait    → 定义行为（能做什么）
impl     → 附加实现（它实际怎么做）

─────────────────────────────────────────
常见四者协作模式：

① 基础模式
   struct Data + impl Data { 方法 }

② 多态模式
   trait Behavior + impl Behavior for TypeA + impl Behavior for TypeB
   → 静态：fn foo(x: &impl Behavior) / fn foo<T: Behavior>(x: &T)
   → 动态：fn foo(x: &dyn Behavior) / Vec<Box<dyn Behavior>>

③ 类型安全的变体
   enum Variant { A(TypeA), B(TypeB) } + impl Variant { fn dispatch() → match }
   → 比 dyn Trait 零开销，比裸 if/else 安全

④ 条件扩展
   impl<T: Bound> MyTrait for T  （毯子实现）
   impl<T: Bound> MyType<T> { 额外方法 }

─────────────────────────────────────────
选择 enum 还是 dyn Trait？

enum：变体数量固定，编译期已知，零开销 match，首选
dyn：变体由外部扩展（插件/回调/跨 crate 扩展），运行时才知类型
```
