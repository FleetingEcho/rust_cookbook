# Rust 100 练习题精要总结 — 举一反三

> 基于 Rust 100 练习题项目，总结每章核心概念、代码模式和实战经验。
> 每个概念都力求：是什么 → 为什么 → 怎么用 → 和 JS/TS/Python/Java 的对比。

---

## 第一章：入门

### 核心概念

| 概念 | 代码 | 说明 |
|------|------|------|
| 函数签名 | `fn name(a: u32, b: u32) -> u32` | 参数**必须**写类型，返回值可推理 |
| 字符串字面量 | `"hello"` | 类型是 `&'static str`，程序全生命周期有效 |
| 测试模块 | `#[cfg(test)] mod tests { ... }` | 只在 `cargo test` 时编译，发布时被排除 |
| assert_eq! | `assert_eq!(a, b)` | 左右不相等则 panic，失败时自动打印两个值 |
| #[should_panic] | `#[should_panic(expected = "...")]` | 验证函数按预期 panic |

---

## 第二章：基础计算器

### 变量声明

```rust
let x = 5;       // 不可变（≈ const）
let mut y = 10;  // 可变（≈ let）
```

- Rust 默认不可变——不是限制，是**默认安全**。只有你需要修改时才加 `mut`
- TS/JS 类比：`let` ≈ `let mut`，`const` ≈ `let`

### if/else 是表达式

```rust
// if/else 可以赋值给变量——它是表达式，不是语句
let result = if n % 2 == 0 {
    12  // 无分号！这就是返回值
} else {
    17
};
```

- **不需要括号**包裹条件，但**必须花括号**包裹分支
- 这和 TS 的 ternary（`cond ? a : b`）类似，但 Rust 的 if/else 支持多行分支

### panic! — 不可恢复的错误

```rust
panic!("The journey took no time at all. That's impossible!");
// 程序立即终止，打印错误消息和回溯
```

- `panic!` 和 `println!` 一样支持格式化字符串
- 在测试中可以用 `#[should_panic(expected = "...")]` 验证
- 相当于其他语言的**抛异常但不捕获**——不适用于可恢复的错误

### 循环：3 种方式实现阶乘

```rust
// 1. 递归（有栈溢出风险，Rust 不保证 TCO）
fn factorial(n: u32) -> u32 {
    if n == 0 { 1 } else { n * factorial(n - 1) }
}

// 2. while（需要手动维护计数器）
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 1;
    while i <= n { result *= i; i += 1; }
    result
}

// 3. for + 范围表达式（最 Rust 风格，推荐）
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 1..=n { result *= i; }
    result
}
```

**范围表达式**：
- `1..=n` — 右闭区间 [1, n]，包含 n
- `1..n` — 右开区间 [1, n)，不包含 n
- `(0..n).rev()` — 倒序

**细节**：
- Rust **没有 `i++`**，只能用 `i += 1`
- for 循环比 while 安全——你**不可能忘记更新计数器**

### 整数安全三兄弟

Rust 在 `debug` 模式下整数溢出会 panic，`release` 模式下自动回绕（wrap）。但你可以**显式选择**想要的行为：

```rust
// 1. wrapping_* —— 回绕（wrap around）
// 类比：汽车里程表，到 999999 后变成 000000
result.wrapping_mul(i);  // u32::MAX + 1 = 0

// 2. saturating_* —— 饱和（saturate）
// 类比：温度计到了顶就不再上升
result.saturating_mul(i);  // 超过 u32::MAX 就停在 u32::MAX

// 3. checked_* —— 返回 Option
// 这是最安全的做法，需要你显式处理 None
match result.checked_mul(i) {
    Some(v) => result = v,
    None => panic!("overflow!"),
}

// 还有 overflowing_* —— 返回 (结果, 是否溢出)
// 适用于需要同时知道结果和溢出状态的场景
let (result, overflowed) = result.overflowing_mul(i);
```

**各方法的返回值对比**（以 `u8` 为例：`200u8.wrapping_add(200)`）：

| 方法 | 结果 | 返回值类型 |
|------|------|-----------|
| `wrapping_add(200)` | `144`（回绕） | `u8` |
| `saturating_add(200)` | `255`（饱和） | `u8` |
| `checked_add(200)` | `None` | `Option<u8>` |
| `overflowing_add(200)` | `(144, true)` | `(u8, bool)` |

### as 类型转换

```rust
let v: u32 = 47u16 as u32;  // 小→大，安全
let x: i8 = 255u8 as i8;    // 大→小，可能截断！255u8 as i8 == -1
let v: u8 = true as u8;     // bool→整数：true=1, false=0
```

- Rust **不允许隐式类型转换**——这是设计选择，避免了 JS/Python 中隐式转换的坑
- `as` 相当于 C 风格类型转换，大范围→小范围可能数据丢失

---

## 第三章：票据系统 V1

### struct + 方法

```rust
struct Ticket {
    title: String,      // 默认私有
    description: String,
}

impl Ticket {
    // 构造函数（关联函数，不是关键字）
    pub fn new(title: String, desc: String) -> Ticket {
        // 验证逻辑...
        Ticket { title, description: desc }
    }

    // getter — &self 是借用，不转移所有权
    pub fn title(&self) -> &str { &self.title }

    // setter — &mut self 可变借用
    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }
}
```

**三种 self 的区别**——这是 Rust 最独特也最重要的概念：

```rust
// 1. self —— 获取所有权（消费）
// 调用后原变量不再可用
fn consume(self) { /* self 被移动进来 */ }
// 调用：ticket.consume(); // 之后 ticket 不可用

// 2. &self —— 不可变借用（只读）
// 调用后原变量仍可用，可以多次调用
fn title(&self) -> &str { &self.title }
// 调用：ticket.title(); ticket.title(); // 可以调多次

// 3. &mut self —— 可变借用（读写）
// 同一时间只能有一个 &mut self
fn set_title(&mut self, title: String) {
    self.title = title;
}
```

**TS 类比**：
| Rust | TS |
|------|-----|
| `self` | `this` + 值传递（少见） |
| `&self` | `this: Readonly<Self>` |
| `&mut self` | 普通 `this` + 允许修改 |

**重要**：`&self` 返回 `&String` 还是 `&str`？**优先用 `&str`**——`&String` 可以通过 Deref 自动转为 `&str`，但反过来不行。函数参数也同理，用 `&str` 更通用。

### 模块与可见性

```rust
mod ticket {
    pub struct Ticket {
        pub title: String,       // pub：外部可访问
        description: String,     // 默认私有：外部不可访问
    }
}

// 使用 super:: 访问父模块
mod helpers {
    use super::Ticket;
}
```

**可见性规则**：
- 默认**私有**（private）
- `pub` — 完全公开
- `pub(crate)` — 仅当前 crate 可见
- `pub(super)` — 仅父模块可见

### 栈 vs 堆 — size_of!

```rust
use std::mem::size_of;

assert_eq!(size_of::<u16>(), 2);        // u16 = 2 字节
assert_eq!(size_of::<i32>(), 4);        // i32 = 4 字节
assert_eq!(size_of::<bool>(), 1);       // bool = 1 字节
assert_eq!(size_of::<String>(), 24);    // String = 指针8 + 长度8 + 容量8
assert_eq!(size_of::<&u16>(), 8);       // 引用 = 8 字节（64位系统）
assert_eq!(size_of::<&Ticket>(), 8);    // 无论被引用类型多大，引用都是 8 字节
```

**String 的内存布局**：
```
栈上（24 字节）:    堆上:
+--------+         +--------+
| ptr    | ──────→ | "hello"|
+--------+         +--------+
| len: 5 |
+--------+
| cap: 5 |
+--------+
```

**`type_id()` 验证返回类型**：
```rust
use std::any::{Any, TypeId};
assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
// 确认 title() 返回的是 &str，不是 &String
```

---

## 第四章：Trait

### trait 是什么

trait 是 Rust 的"接口"，定义了一组方法签名：

```rust
// 定义 trait
trait IsEven {
    fn is_even(&self) -> bool;
}

// 为不同类型实现
impl IsEven for u32 {
    fn is_even(&self) -> bool { self % 2 == 0 }
}

impl IsEven for i32 {
    fn is_even(&self) -> bool { self % 2 == 0 }
}

// 使用
assert!(42u32.is_even());
```

### 孤儿规则（Orphan Rule）

> 当实现 trait 时，trait 或类型**至少有一个**是在当前 crate 中定义的。

```rust
// ❌ 编译错误：PartialEq 和 u32 都是标准库的
impl PartialEq for u32 { /* ... */ }

// ✅ 可以的：你自定义的 trait 或你自己的类型
impl IsEven for u32 { /* ... */ }       // IsEven 是你定义的
impl PartialEq for Ticket { /* ... */ } // Ticket 是你定义的
```

这是 Rust 一致性系统的核心：**不会有两个 crate 同时为同一个类型实现同一个 trait**，避免了 C++ 的"符号冲突"问题。

### 运算符重载（以 PartialEq 为例）

```rust
// 手动实现 == / !=
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
            && self.description == other.description
            && self.status == other.status
    }
    // != 会自动根据 eq 取反，不用实现
}
```

之后就可以用 `ticket1 == ticket2` 比较了。

### derive 宏 — 自动实现 trait

```rust
#[derive(Debug, PartialEq, Clone, Copy)]
struct Ticket { /* ... */ }
```

`#[derive]` 自动生成 trait 的 impl 代码：

| derive | 作用 | TS 类比 |
|--------|------|---------|
| `Debug` | 支持 `{:?}` 格式化打印 | `.toString()` |
| `PartialEq` | 支持 `==` 和 `!=` | `.equals()` 或 `===` |
| `Clone` | 提供 `.clone()` 方法，显式复制 | 手动深拷贝 |
| `Copy` | 赋值时自动按位复制（不 move） | 基本类型赋值（number） |
| `Hash` | 可作为 HashMap 的键 | 对象做 map key |
| `Eq` | 完全等价（需先 PartialEq） | — |

**注意**：`derive` 是按字段处理的，要求每个字段也实现了相应的 trait。

### trait bound — 泛型约束

```rust
// 写法 A：尖括号内直接写（简洁）
pub fn min<T: std::cmp::PartialOrd>(left: T, right: T) -> T {
    if left <= right { left } else { right }
}

// 写法 B：where 子句（多个 bound 时更清晰）
pub fn min<T>(left: T, right: T) -> T
where T: PartialOrd
{ /* 同上 */ }
```

- `PartialOrd` 是 `<`、`<=`、`>=`、`>` 对应的 trait
- `+` 可以组合多个 bound：`T: Debug + Clone + PartialEq`

### Clone vs Copy — 最常混淆的概念

```rust
// Clone: 显式复制（任何类型都可以）
#[derive(Clone)]
struct Ticket { title: String }
let t2 = t1.clone();  // 必须显式调用 .clone()

// Copy: 隐式复制（仅简单类型）
#[derive(Clone, Copy)]  // Copy 要求 Clone
struct WrappingU32 { value: u32 }  // 所有字段都必须是 Copy 的
let y = x;  // 不 move，直接复制！x 仍然可用
```

**核心区别**：

| | `Clone` | `Copy` |
|------|---------|--------|
| 如何调用 | 显式 `.clone()` | **赋值时自动** `let y = x` |
| 性能 | 可能很重（深拷贝 String 等） | 轻量（按位复制） |
| 谁可以实现 | 任何类型 | 仅**全部字段都是 Copy** 的类型 |
| 必须同时 | 无 | `Copy` 要求实现 `Clone` |
| TS 类比 | `structuredClone(obj)` | `let y = x`（对于 number/boolean） |

```rust
// 有了 Copy，可以连续用：
let x = WrappingU32::new(42);
let y = x;      // 复制，不是 move
let z = x;      // x 仍然有效！
assert_eq!(x + y + z, WrappingU32::new(126));
// 如果 WrappingU32 没有 Copy，x 在 let y = x 后就不能用了
```

### From/Into — 类型转换

```rust
pub struct WrappingU32 { value: u32 }

// 关键：实现了 From 就**自动**获得了 Into
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        WrappingU32 { value }
    }
}

// 三种使用方式，效果完全一样：
let a: WrappingU32 = WrappingU32::from(42);     // From trait
let b: WrappingU32 = 42.into();                  // Into trait（类型推理）
let c = WrappingU32::from(42);                   // 类型推理
```

**From vs Into 的选择**：
- **尽量实现 `From`**，编译器会自动生成 `Into`
- 只需要实现一个方向的转换（`From`），不用两个都写
- `Into` 在函数参数中特别好用：
```rust
fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
    self.tickets.push(ticket.into());
}
// 这样就可以接受 Ticket 和任何能转为 Ticket 的类型
```

**`"hello".into()` 是什么意思？**
- `String::from("hello")` 将 `&str` 转为 `String`
- 因为 `String` 实现了 `From<&str>`，所以 `"hello".into()` 也能得到 `String`
- 但要注意：`into()` 依赖类型推理，所以需要目标类型明确：
```rust
let s: String = "hello".into();  // ✅ 指定了类型
let s = "hello".into();          // ❌ 不明确，编译错误
```

### TryFrom — 可能失败的转换

```rust
use std::convert::TryFrom;

impl TryFrom<String> for Status {
    type Error = String;  // 失败时返回的类型

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("`{}` is not a valid status", value)),
        }
    }
}

// 使用
let status = Status::try_from("todo".to_string())?;  // Ok(Status::ToDo)
```

**TryFrom 的好处**：
- 编译期验证不通过？不，这是运行时验证。但它的价值在于：
  - **统一的转换接口**：`TryFrom` / `From` 标准模式
  - **自动获得 `try_into()`**：`"todo".try_into()`
  - **与 `?` 运算符完美配合**

### Error trait — Rust 的错误标准

```rust
use std::fmt;

#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

// 要实现 Error，需要先实现 Display（因为 Error: Display + Debug）
impl fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketNewError::TitleError(msg) => write!(f, "{}", msg),
            TicketNewError::DescriptionError(msg) => write!(f, "{}", msg),
        }
    }
}

// 实现 Error trait（最常见的空实现）
impl std::error::Error for TicketNewError {}

// 如果有 source（错误链），可以覆盖 source() 方法
impl Error for TicketNewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidStatus(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}
```

**为什么 `impl std::error::Error for TicketNewError {}` 是空的？**
- 因为 `Error` trait 只有可选方法（`source()` 有默认实现返回 `None`）
- 你只需要证明"这个类型是 Error"，Rust 的标准错误链机制就能工作
- `?` 运算符可以自动转换实现了 `std::error::Error` 的类型

### Drop trait — 析构函数

```rust
pub struct DropBomb {
    defused: bool,
}

impl DropBomb {
    pub fn new() -> Self { DropBomb { defused: false } }
    pub fn defuse(&mut self) { self.defused = true; }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if !self.defused {
            panic!("DropBomb exploded!");
        }
    }
}

// 测试
#[test]
#[should_panic]
fn test_drop_bomb() {
    let _bomb = DropBomb::new();  // 没有 defuse，离开作用域就 panic
}

#[test]
fn test_defused_drop_bomb() {
    let mut bomb = DropBomb::new();
    bomb.defuse();  // defuse 后安全退出
}
```

**Drop bomb 模式**：在 `drop()` 中检验"操作是否被执行"，这是一个经典的 RAII 守卫模式：
- 构造时创建"炸弹"
- 正常路径上"拆弹"（`defuse()`）
- 如果忘记走正常路径，`drop()` 时炸弹爆炸

### 静态大小类型（DST）和 Sized

```rust
// str 是动态大小类型（DST），编译时大小未知
// 以下代码编译不过：
// std::mem::size_of::<str>();  // ❌ 不能直接取 str 的大小

// 总是通过引用使用：
// &str 有固定大小（8 字节指针）
fn example() {
    // 注释掉上面那行即可编译
}
```

- DST 类型只能在引用（`&`、`Box`、`Rc` 等）后面使用
- 所有泛型默认有 `Sized` bound：`fn f<T>(t: T)` 等价于 `fn f<T: Sized>(t: T)`
- 如果想接受 DST：`fn f<T: ?Sized>(t: &T)`

### 关联类型 vs 泛型参数

```rust
// 泛型参数：同一类型可以有多个实现
trait Power<Exponent> {
    fn power(&self, exponent: Exponent) -> u32;
}
// 可以为 u32 实现 Power<u16>、Power<u32>、Power<&u32> 等

// 关联类型：每个实现只能有一个类型
trait Iterator {
    type Item;  // 固定为一种类型
    fn next(&mut self) -> Option<Self::Item>;
}
// 对于 Vec<i32> 的迭代器，Item 固定为 i32
```

**选择原则**：
- 如果类型和 trait 的实现**一一对应**（一个类型只有一个实现）→ **关联类型**
- 如果同一个类型需要**多种实现** → **泛型参数**

---

## 第五章：票据系统 V2

### enum — 枚举变体可以带数据

```rust
#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },  // 变体带命名字段
    Done,
}

// match 时提取数据
fn assigned_to(&self) -> &str {
    match &self.status {
        Status::InProgress { assigned_to } => assigned_to,
        _ => panic!("Not in progress"),
    }
}
```

**TS 类比**：
```typescript
// Rust enum ≈ TS 的 discriminated union
type Status =
    | { tag: 'ToDo' }
    | { tag: 'InProgress', assigned_to: string }
    | { tag: 'Done' }
```

区别在于 Rust 的 match **强制穷举**——你漏掉一个变体编译器就报错。

### Option<T> — 安全地表示"可能有值"

```rust
pub fn assigned_to(&self) -> Option<&String> {
    match &self.status {
        Status::InProgress { assigned_to } => Some(assigned_to),
        _ => None,  // 非 InProgress 返回 None，而非 panic
    }
}

// 使用
if let Some(assignee) = ticket.assigned_to() {
    println!("Assigned to: {}", assignee);
} else {
    println!("Not assigned");
}
```

**为什么比 `null` 好？**
- Rust 没有 `null` 关键字——`Option<T>` 是唯一的"可空"方式
- 编译器**强制**你处理 `None` 的情况——你不会忘记判空
- 类型签名就说明了"可能为空"：`fn assigned_to(&self) -> Option<&String>`

### Result<T, E> — 安全地表示"可能失败"

```rust
pub fn new(title: String, desc: String, status: Status) -> Result<Ticket, String> {
    if title.is_empty() {
        return Err("Title cannot be empty".to_string());
    }
    Ok(Ticket { title, description: desc, status })
}

// 调用方式
let ticket = Ticket::new("Hello".into(), "Desc".into(), Status::ToDo).unwrap();
// unwrap = 成功提取 Ok，失败则 panic

let ticket = Ticket::new(...).expect("Failed to create ticket");
// expect = unwrap + 自定义 panic 消息
```

### unwrap_err — 验证错误路径

```rust
#[test]
fn title_cannot_be_empty() {
    // unwrap_err 期望 Result 是 Err 变体
    // 如果是 Ok 反而会 panic
    let err = Ticket::new("".into(), valid_desc(), Status::ToDo).unwrap_err();
    assert_eq!(err, "Title cannot be empty");
}
```

| 方法 | 用于 | 成功时 | 失败时 |
|------|------|--------|--------|
| `unwrap()` | `Ok` 取值 | 返回 Ok 内部值 | panic |
| `unwrap_err()` | **验证错误** | panic | 返回 Err 内部值 |
| `expect(msg)` | `Ok` 取值 | 返回 Ok 内部值 | panic + 自定义消息 |
| `?` | 传播错误 | 提取 Ok 值 | 从函数返回 Err |

### TryFrom — 从字符串解析枚举

```rust
impl TryFrom<&str> for Status {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("`{}` is not a valid status", value)),
        }
    }
}

// 不区分大小写
assert_eq!(Status::try_from("ToDO").unwrap(), Status::ToDo);
assert_eq!((&"inproGress"[..]).try_into().unwrap(), Status::InProgress);
```

### Error::source() — 错误链

```rust
impl Error for TicketNewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidStatus(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

#[test]
fn invalid_status() {
    let err = Ticket::new(valid_title(), valid_desc(), "invalid".into()).unwrap_err();
    assert!(err.source().is_some());  // 还有下层错误
}
```

### 类型驱动设计 — 最终形态

```rust
struct TicketTitle(String);  // 只要存在就保证有效

impl TryFrom<String> for TicketTitle {
    type Error = TitleError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() { return Err(TitleError::Empty); }
        if value.len() > 50 { return Err(TitleError::TooLong); }
        Ok(Self(value))
    }
}

// Ticket 的字段直接用 TicketTitle
struct Ticket {
    pub title: TicketTitle,   // 不需要在 Ticket::new() 里再验证！
    pub description: TicketDescription,
    pub status: Status,
}
```

**好处**：
- 不再需要运行时验证——**T 类型本身保证了有效性**
- 字段甚至可以 `pub`——因为这不是"原始字符串"，而是一个已经过验证的类型
- 验证逻辑集中到类型自身的 `TryFrom` 实现

---

## 第六章：票据管理（集合与迭代器）

### Vec — 动态数组

```rust
let mut v = Vec::new();
v.push(1);
v.push(2);
v.push(3);  // 自动扩容，容量翻倍

// 预分配容量（减少扩容次数）
let mut v = Vec::with_capacity(100);
assert_eq!(v.capacity(), 100);
```

**扩容策略**：Vec 满时容量翻倍（2 → 4 → 8 → 16...），均摊 O(1) 插入。

### IntoIterator — 让你的类型支持 for 循环

核心概念：**Rust 的 `for` 循环是通过 `IntoIterator` trait 实现的**。

```rust
// for x in collection 实际上等价于：
let mut iter = collection.into_iter();
while let Some(x) = iter.next() { /* ... */ }
```

有**3 种迭代方式**，对应 3 种 `IntoIterator` 实现：

```rust
struct TicketStore { tickets: Vec<Ticket> }

// 1. 消费（ownership）：for x in store
impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

// 2. 不可变引用：for x in &store
impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = std::slice::Iter<'a, Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}

// 3. 可变引用：for x in &mut store
impl<'a> IntoIterator for &'a mut TicketStore {
    type Item = &'a mut Ticket;
    type IntoIter = std::slice::IterMut<'a, Ticket>;
    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter_mut()
    }
}
```

**为什么 `impl IntoIterator for &'a TicketStore` 中用 `impl<'a>`？**
- 生命周期参数 `'a` 告诉 Rust："返回的引用和 `self` 活得一样久"
- 这保证迭代器在使用期间，store 不会被 drop

**为自定义类型提供迭代器有什么好处？**
- ✅ 可以用 `for` 循环直接遍历
- ✅ 可以和 `collect()`、`filter()`、`map()` 等适配器一起用
- ✅ 和标准库生态无缝对接

### Index / IndexMut — [] 运算符重载

```rust
use std::ops::{Index, IndexMut};

// 只读索引
impl Index<TicketId> for TicketStore {
    type Output = Ticket;
    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}

// 也支持 &TicketId
impl Index<&TicketId> for TicketStore {
    type Output = Ticket;
    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}

// 可变索引
impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}

// 使用
let ticket = &store[id];      // Index
let ticket = &store[&id];     // Index (over &TicketId)
store[id].status = Status::Done;  // IndexMut
```

**Index 的好处**：
- `store[id]` 比 `store.get(id).unwrap()` 更简洁直观
- 类似数组/Map 的访问语法
- 不过 `Index` 会 panic（如果索引不存在）

### 迭代器适配器

```rust
// filter — 过滤
pub fn to_dos(&self) -> Vec<&Ticket> {
    self.tickets.iter()
        .filter(|t| t.status == Status::ToDo)
        .collect()
}

// map — 映射
let titles: Vec<&str> = store.iter()
    .map(|t| t.title.as_str())
    .collect();

// chain — 链式调用，和 TS 的 array chain 一模一样
```

### impl Trait 语法

```rust
// 返回迭代器而不暴露具体类型
pub fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
    self.tickets.iter().filter(|t| t.status == Status::InProgress)
}
```

**好处**：
- 调用方不需要知道具体返回的是 `Filter<Iter<Ticket>, ...>` 这种复杂类型
- 编译期仍然是**静态分发**，没有虚函数开销

### 切片 &[T] / &mut [T]

```rust
// &[u32] 可以接受 Vec<u32>、[u32; 5]、&[u32] 的切片
fn sum(values: &[u32]) -> u32 {
    values.iter().sum()
}

// &mut [i32] 可以修改元素
fn squared(slice: &mut [i32]) {
    for val in slice.iter_mut() {
        *val = (*val) * (*val);
    }
}
```

**切片的好处**：极其灵活的 API——接受任何连续元素序列，不关心是 Vec 还是数组。

### HashMap / BTreeMap

```rust
use std::collections::HashMap;

// HashMap —— 无序，要求键实现 Hash + Eq
let mut map = HashMap::new();
map.insert(key, value);

// BTreeMap —— 按键排序，要求键实现 Ord
use std::collections::BTreeMap;
let mut map = BTreeMap::new();
map.insert(key, value);
// 遍历时按键的顺序返回
```

**选择**：
- HashMap：O(1) 查找，无序，大多数场景用这个
- BTreeMap：O(log n) 查找，有序，需要范围查询或顺序遍历时用

---

## 第七章：并发

### thread::spawn + move

```rust
use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let left = v[..mid].to_vec();   // 必须复制！因为...
    let right = v[mid..].to_vec();

    let handle1 = thread::spawn(move || left.iter().sum::<i32>());
    //                        ^^^^ 为什么要有 move？
    let handle2 = thread::spawn(move || right.iter().sum::<i32>());

    handle1.join().unwrap() + handle2.join().unwrap()
}
```

**为什么 `thread::spawn` 的闭包要有 `move`？**

因为 Rust 不知道线程会活多久——如果不用 `move`，闭包中的引用可能在线程执行之前就失效了（即 Rust 的生命周期检查通不过）。

```rust
// 假设没有 move（实际上编译不过）：
let left = v[..mid].to_vec();
thread::spawn(|| left.iter().sum::<i32>());
// drop(left) 可能在线程结束之前发生！—— 这就是生命周期问题
```

`move` 关键字**强制闭包拿走变量的所有权**，这样变量就不会在线程执行期间被 drop。

而 `v[..mid].to_vec()` 必须要 clone 一份数据，因为 `Vec<i32>` 不是 `Copy` 类型——它拥有堆上的数据，不能同时被两个线程拥有。

### std::thread::scope — 借用式的线程

```rust
pub fn sum(v: Vec<i32>) -> i32 {
    std::thread::scope(|s| {
        let mid = v.len() / 2;
        let (left, right) = v.split_at(mid);  // 不需要 clone！

        // scope 内的线程可以借用局部变量
        let handle1 = s.spawn(|| left.iter().sum::<i32>());
        let handle2 = s.spawn(|| right.iter().sum::<i32>());

        handle1.join().unwrap() + handle2.join().unwrap()
    })  // 所有子线程在这里之前必须结束
    // 因为 scope 结束时会自动 join 所有子线程
}
```

**`scope` vs `spawn` 的区别**：

| | `thread::spawn` | `thread::scope` |
|---|---|---|
| 变量捕获 | 必须 `move`（获取所有权） | 可以**借用** |
| 生命周期 | `'static` | 借用和 scope 一样长 |
| 自动 join | 否，手动 join | **是**，scope 结束前自动 join |
| 需要 clone | 经常需要 | 通常不需要 |
| 适用场景 | 长期运行的后台线程 | 短期并行计算 |

**`scope` 为什么能借用？**
因为编译器能证明：scope 结束时会**等待所有子线程完成**再返回。所以局部变量在 scope 期间一定有效，不存在"线程还在跑但变量被 drop了"的问题。

### mpsc 通道 — 多生产者单消费者

```rust
use std::sync::mpsc::{channel, Sender, Receiver};

// 创建通道
let (sender, receiver) = channel();

// 发送端（可以克隆以多生产者）
sender.send("hello").unwrap();

// 接收端（阻塞等待）
let msg = receiver.recv().unwrap();  // 阻塞直到有消息
```

**基本模式**：
```rust
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));  // 服务器线程
    sender  // 把发送端返回给调用方
}

fn server(receiver: Receiver<Command>) {
    loop {
        match receiver.recv() {
            Ok(cmd) => { /* 处理命令 */ }
            Err(_) => break,  // 所有发送端被 drop，通道关闭
        }
    }
}
```

### sync_channel — 有界通道

```rust
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

// 创建容量为 5 的有界通道
let (sender, receiver) = sync_channel(5);

// try_send — 满时立即返回错误，不阻塞
sender.try_send(cmd).map_err(|_| "Overloaded")?;

// recv — 接收端仍然是阻塞的
```

**`channel()` vs `sync_channel(n)`**：

| | unbounded `channel()` | bounded `sync_channel(n)` |
|---|---|---|
| 缓冲区 | 无限制 | 最多 n 条 |
| 发送 | `send()` 永不阻塞 | `send()` 满时阻塞 |
| 非阻塞发送 | 不需要 | `try_send()` 满时返回错误 |
| 适用场景 | 生产者远快于消费者时可能 OOM | 防止生产者过度，施加背压 |

### 请求-响应模式

```rust
enum Command {
    Insert {
        draft: String,
        response_channel: SyncSender<u64>,  // 每个命令自带响应通道
    },
    Get {
        id: u64,
        response_channel: SyncSender<Option<String>>,
    },
}

impl TicketStoreClient {
    pub fn insert(&self, draft: String) -> Result<u64, ()> {
        let (resp_s, resp_r) = sync_channel(1);
        self.sender.try_send(Command::Insert {
            draft,
            response_channel: resp_s,
        }).map_err(|_| ())?;
        Ok(resp_r.recv().unwrap())  // 等待服务器响应
    }
}
```

**好处**：
- 客户端和服务器通过消息通信，无需直接共享状态
- 响应通道确保每个请求都能得到对应的响应
- 天然线程安全（通道是线程安全的）

### Mutex — 互斥锁

```rust
use std::sync::{Arc, Mutex};

// Arc<Mutex<T>> 是 Rust 并发共享数据的标准模式
let data = Arc::new(Mutex::new(String::from("hello")));

let data_clone = data.clone();
thread::spawn(move || {
    let mut guard = data_clone.lock().unwrap();
    *guard = String::from("world");  // 修改需要锁
    // guard 离开作用域自动释放锁
});

// 读取
let guard = data.lock().unwrap();
assert_eq!(*guard, "world");
// guard drop，锁释放
```

**Mutex 的优点（相比其他语言）**：
- Mutex **持有数据**（`Mutex<T>`），数据和锁绑定一起，不会被"忘了加锁"
- 锁通过 RAII 自动释放——不会忘记 `unlock()`
- 加锁返回 `MutexGuard`——这是一个智能指针，离开作用域自动释放锁

### RwLock — 读写锁

```rust
use std::sync::{Arc, RwLock};

let data = Arc::new(RwLock::new(String::from("hello")));

// 多个线程可以同时读
let reader = data.read().unwrap();
println!("{}", *reader);

// 写锁是独占的
let mut writer = data.write().unwrap();
*writer = String::from("world");
```

**Mutex vs RwLock**：

| | Mutex | RwLock |
|---|---|---|
| 读时 | 互斥（读和写不能同时） | **多个读者可以同时读** |
| 写时 | 独占 | 独占 |
| 适用场景 | 读写都频繁 | 读多写少 |

### 共享状态（无通道）

```rust
use std::sync::{Arc, RwLock};

struct TicketStore { tickets: Vec<String> }
impl TicketStore {
    fn add_ticket(&mut self, draft: String) -> u64 {
        self.tickets.push(draft);
        self.tickets.len() as u64 - 1
    }
}

// 线程间直接共享 TicketStore
let store = Arc::new(RwLock::new(TicketStore { tickets: Vec::new() }));

let s1 = store.clone();
let t1 = thread::spawn(move || s1.write().unwrap().add_ticket("hello".into()));

let s2 = store.clone();
let t2 = thread::spawn(move || s2.write().unwrap().add_ticket("world".into()));
```

**通道 vs 共享状态**：

| | 通道 (Channel) | 共享状态 (Arc<Lock>) |
|---|---|---|
| 同步方式 | 消息传递 | 锁同步 |
| 耦合度 | 低（通过消息通信） | 高（共享数据） |
| 死锁风险 | 低 | 较高（需要小心锁顺序） |
| 适用场景 | Actor 模式、流水线 | 简单共享数据 |

### Send / Sync — 并发安全的基石

- **Send**：类型可以**跨线程转移所有权**（`Rc<T>` 不是 Send，`Arc<T>` 是）
- **Sync**：类型可以**跨线程共享引用**（即 `&T: Send`）

```rust
// Rc<T>: ❌ 不是 Send（引用计数非线程安全）
// Arc<T>: ✅ Send + Sync（原子引用计数）

// Mutex<T>: ✅ Send（锁可以跨线程移动）
// RefCell<T>: ❌ 不是 Sync（运行时借用检查非线程安全）
```

大多数类型自动实现 Send + Sync，你基本不需要手动实现它们。

---

## 第八章：异步编程（08_futures）

第八章的练习**已全部实现并通过测试**。以下是每个文件的实现要点和关键概念。

### 01_async_fn — 第一个异步函数

```rust
use tokio::net::TcpListener;

// 实现 echo server：接受连接，原样返回数据
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (mut stream, _) = listener.accept().await?;
        // 每个连接开一个独立任务，支持并发
        tokio::spawn(async move {
            let (mut reader, mut writer) = stream.split();
            tokio::io::copy(&mut reader, &mut writer).await.unwrap();
        });
    }
}
```

**关键模式**：`tokio::spawn` 每个连接一个任务，实现并发处理。

### 02_spawn — tokio::select! 多路监听

```rust
use tokio::net::TcpListener;

pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        // tokio::select! 同时等待两个 listener，谁先来就处理谁
        tokio::select! {
            r = first.accept() => {
                let (mut stream, _) = r?;
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    tokio::io::copy(&mut reader, &mut writer).await.unwrap();
                });
            }
            r = second.accept() => {
                let (mut stream, _) = r?;
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    tokio::io::copy(&mut reader, &mut writer).await.unwrap();
                });
            }
        }
    }
}
```

**`tokio::select!` 宏**：同时 await 多个 future，先完成的那个被选中，另一个被取消。相当于 `Promise.race()`。

### 03_runtime — Arc 共享数据 + tokio::spawn

```rust
pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    T: Display + Send + Sync + 'static,
{
    // tokio::spawn 要求 'static，所以用 Arc 共享
    let reply = Arc::new(reply);
    loop {
        tokio::select! {
            r = first.accept() => {
                let (mut stream, _) = r.unwrap();
                let reply = Arc::clone(&reply);
                tokio::spawn(async move {
                    let msg = format!("{}", reply);
                    stream.write_all(msg.as_bytes()).await.unwrap();
                });
            }
            // ...
        }
    }
}
```

**关键**：`tokio::spawn` 的闭包必须 `Send + 'static`。`Arc<T>` 让引用计数数据能在多个任务间共享。

### 04_future — Rc 不能跨 .await

```rust
async fn example() {
    // 用块 {} 限制 Rc 的作用域，使 Rc 不在 .await 时持有
    {
        let non_send = Rc::new(1);
        println!("{}", non_send);
    }  // Rc 在这里 drop
    yield_now().await;  // 不再持有 Rc，编译通过
}
```

**为什么**：`tokio::spawn` 要求 `Send`，但 `Rc` 不是 `Send`（引用计数非原子操作）。如果 `Rc` 跨越 `.await` 点，编译器报错。

### 05_blocking — spawn_blocking 解决同步阻塞

```rust
pub async fn echo(listener: TcpListener) -> Result<(), anyhow::Error> {
    loop {
        let (socket, _) = listener.accept().await?;
        // 把同步 IO（std::io::Read/Write）移到专用线程池
        tokio::task::spawn_blocking(move || {
            let mut socket = socket.into_std()?;
            socket.set_nonblocking(false)?;
            let mut buffer = Vec::new();
            socket.read_to_end(&mut buffer)?;
            socket.write_all(&buffer)?;
            Ok::<_, anyhow::Error>(())
        })
        .await
        .unwrap()?;
    }
}
```

**为什么需要 `spawn_blocking`**：
- tokio 的线程数 = CPU 核数（通常 8 个）
- 如果在一个 tokio 线程里做 `read_to_end`（同步阻塞），**这个线程就不能处理任何其他任务**
- `spawn_blocking` 把阻塞操作交给独立的线程池，不阻塞 tokio 的事件循环

### 06_async_aware_primitives — 同步通道 vs 异步通道

```rust
// ❌ 原来的代码（会死锁）：在 async 函数中用 std::sync::mpsc
async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Ok(msg) = receiver.recv() {  // 同步阻塞！
            // ...
        }
    }
}

// ✅ 改写后：用 tokio::sync::mpsc
use tokio::sync::mpsc;

async fn pong(mut receiver: mpsc::Receiver<Message>) {
    loop {
        if let Some(msg) = receiver.recv().await {  // 异步等待，不阻塞
            println!("Pong received: {}", msg.payload);
            let (sender, new_receiver) = mpsc::channel(1);
            msg.response_channel
                .send(Message {
                    payload: "pong".into(),
                    response_channel: sender,
                })
                .await
                .unwrap();
            receiver = new_receiver;
        }
    }
}
```

**死锁原因**：`std::sync::mpsc::recv()` 是同步阻塞的，它会**卡住整个 tokio 线程**。如果所有 tokio 线程都被卡住，异步任务就无法推进，形成死锁。

### 07_cancellation — 超时与协作式取消

```rust
pub async fn run(listener: TcpListener, n_messages: usize, timeout: Duration) -> Vec<u8> {
    let mut buffer = Vec::new();
    for _ in 0..n_messages {
        let (mut stream, _) = listener.accept().await.unwrap();
        // 给每个连接的读取操作设置 20ms 超时
        let _ = tokio::time::timeout(timeout, async {
            stream.read_to_end(&mut buffer).await.unwrap();
        })
        .await;
    }
    buffer  // 返回部分读取的数据
}

// 测试：客户端每次只发一半数据，然后等 40ms（> 超时 20ms）
// 所以 server 每次只读到前半段就被取消了
// 4 条消息 × 前半段 → 结果 = "hefrthta"
```

**协作式取消含义**：
- Future 被取消不是"线程被强杀"，而是在下一个 `.await` 点**停止轮询**
- 已经读到的数据还在 `buffer` 里
- 取消后可以继续执行其他逻辑

### 08_outro — 自由项目

最后的练习是开放式的：建议用 `axum` 或 `actix-web` 结合 Ticket 系统构建一个 REST API。

### 异步编程核心原则总结

| 概念 | 说明 |
|------|------|
| `async fn` 返回 Future | Future 是惰性的，需要执行器（如 tokio）驱动 |
| `.await` 让出线程 | 类似 TS 的 `await`，但不阻塞线程 |
| `tokio::spawn` 并发 | 类似 `thread::spawn`，但用于异步任务，要求 `Send + 'static` |
| `tokio::select!` 多路复用 | 同时 await 多个 future，先到先得，类似 `Promise.race()` |
| `spawn_blocking` 隔离阻塞 | 同步 IO / CPU 密集操作用它，否则阻塞整个运行时 |
| 异步通道 | `tokio::sync::mpsc` 替代 `std::sync::mpsc`，避免死锁 |
| `timeout` 协作式取消 | 超时后 Future 不再被 poll，但已读数据不丢失 |
| Rc 不能跨 `.await` | `Rc` 不是 `Send`，确保在 `.await` 前 drop |

---

## 知识全景图

```
第一章：  函数、字符串、测试基础
    ↓
第二章：  变量、if 表达式、循环、整数安全三兄弟、类型转换
    ↓
第三章：  struct、方法(三种self)、可见性、所有权、栈堆(size_of)、Drop
    ↓
第四章：  trait、孤儿规则、运算符重载、derive、泛型bound、
           &str vs &String、DST、关联类型vs泛型、
           Clone vs Copy、From/Into、TryFrom、Error trait、Drop bomb
    ↓
第五章：  enum、match(穷举)、if let、Option、Result(unwrap/unwrap_err/?)、
           自定义错误枚举、Error::source()、类型驱动设计
    ↓
第六章：  Vec、IntoIterator(3种实现)、Index/IndexMut、迭代器适配器、
           impl Trait、切片、HashMap/BTreeMap
    ↓
第七章：  thread::spawn(move)、scope(借用)、mpsc通道、sync_channel、
           请求-响应模式、Mutex、RwLock、Send/Sync
    ↓
第八章：  async/await、tokio::spawn、spawn_blocking、异步通道、
           超时与协作式取消、Rc跨await陷阱
```

### 核心理念

1. **所有权是 Rust 的灵魂** — 理解 `self` / `&self` / `&mut self` 的区别就理解了 80% 的 Rust
2. **类型系统 = 编译期测试** — 很多其他语言运行时才暴露的问题，Rust 在编译期就能捕获
3. **Error 处理演进**：`panic!` → `Result<_, String>` → 自定义 `Error` trait → 类型驱动设计
4. **并发是工具箱**：线程 + 通道 + 锁 + async/await，各有用处，没有银弹
5. **零成本抽象**：迭代器、trait、async 在编译后和手写底层代码一样高效
