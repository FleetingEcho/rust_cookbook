# Rust 常见错误：新手到中级

> 按出现频率和"坑的深度"排列。每条都附有错误写法、正确写法和原因说明。

---

## 目录

**所有权 / 借用**
1. [Move 之后继续使用值](#1-move-之后继续使用值)
2. [同时持有可变和不可变引用](#2-同时持有可变和不可变引用)
3. [在循环中借用集合又修改它](#3-在循环中借用集合又修改它)
4. [返回局部变量的引用](#4-返回局部变量的引用)
5. [到处 .clone() 逃避借用检查器](#5-到处-clone-逃避借用检查器)

**类型 / 转换**
6. [混淆 &str 和 String](#6-混淆-str-和-string)
7. [用 as 做类型转换导致截断](#7-用-as-做类型转换导致截断)
8. [整数溢出在 release 模式下静默回绕](#8-整数溢出在-release-模式下静默回绕)
9. [collect() 忘记标注类型](#9-collect-忘记标注类型)
10. [iter() / iter_mut() / into_iter() 用错](#10-iter--iter_mut--into_iter-用错)

**错误处理**
11. [unwrap() 满天飞](#11-unwrap-满天飞)
12. [用嵌套 match 替代 ? 运算符](#12-用嵌套-match-替代--运算符)
13. [在库代码中 panic](#13-在库代码中-panic)

**Async**
14. [在 async 函数里调用阻塞操作](#14-在-async-函数里调用阻塞操作)
15. [忘记 .await 导致 Future 不执行](#15-忘记-await-导致-future-不执行)

**模式 / 结构**
16. [用 if-else 链替代 match](#16-用-if-else-链替代-match)
17. [混淆 Arc/Mutex 和 Rc/RefCell 的使用场景](#17-混淆-arcmutex-和-rcrefcell-的使用场景)
18. [字符串拼接用 + 导致意外的所有权转移](#18-字符串拼接用--导致意外的所有权转移)
19. [结构体实现方法时 self 选错](#19-结构体实现方法时-self-选错)
20. [忽略 clippy 警告](#20-忽略-clippy-警告)

---

## 所有权 / 借用

### 1. Move 之后继续使用值

```rust
// ❌ 错误：String 被 move 进函数后不能再用
fn print_name(name: String) {
    println!("{}", name);
}

let name = String::from("Alice");
print_name(name);
println!("{}", name); // 编译错误：value borrowed here after move
```

```rust
// ✅ 方法一：传引用（大多数情况应该这样做）
fn print_name(name: &str) {
    println!("{}", name);
}

let name = String::from("Alice");
print_name(&name);
println!("{}", name); // OK，name 还在

// ✅ 方法二：函数需要所有权时，调用方 clone
print_name(name.clone());
println!("{}", name); // OK

// ✅ 方法三：返回所有权（函数"借走"再还回来）
fn print_and_return(name: String) -> String {
    println!("{}", name);
    name  // 还给调用者
}
```

> **规则**：函数参数几乎总是用 `&T` 或 `&mut T`，只有真正需要拥有值时才用 `T`。

---

### 2. 同时持有可变和不可变引用

```rust
// ❌ 错误：不可变引用存在期间不能创建可变引用
let mut v = vec![1, 2, 3];
let first = &v[0];       // 不可变借用开始
v.push(4);               // 编译错误：可变借用与 first 冲突
println!("{}", first);   // first 还活着
```

```rust
// ✅ 方法一：不可变引用用完后再修改
let mut v = vec![1, 2, 3];
let first = v[0];        // Copy 类型，直接拿值，不借用
v.push(4);               // OK
println!("{}", first);

// ✅ 方法二：先复制需要的值，再修改
let mut v = vec![1, 2, 3];
let first_val = v[0].clone(); // 拿副本，借用立刻结束
v.push(4);
println!("{}", first_val);

// ✅ 方法三：缩短不可变借用的生命周期（NLL 作用域）
let mut v = vec![1, 2, 3];
{
    let first = &v[0];
    println!("{}", first); // 借用在这里结束
}
v.push(4); // OK，之前的借用已经结束
```

---

### 3. 在循环中借用集合又修改它

```rust
// ❌ 错误：遍历的同时修改 Vec
let mut v = vec![1, 2, 3, 4, 5];
for x in &v {
    if *x == 3 {
        v.push(99); // 编译错误：cannot borrow `v` as mutable
    }
}
```

```rust
// ✅ 方法一：retain 过滤
let mut v = vec![1, 2, 3, 4, 5];
v.retain(|&x| x != 3); // 原地删除不符合条件的元素

// ✅ 方法二：先收集要添加的，再统一处理
let mut v = vec![1, 2, 3, 4, 5];
let to_add: Vec<i32> = v.iter()
    .filter(|&&x| x == 3)
    .map(|_| 99)
    .collect();
v.extend(to_add);

// ✅ 方法三：索引遍历
let mut v = vec![1, 2, 3, 4, 5];
let len = v.len();
for i in 0..len {
    if v[i] == 3 {
        v.push(99);
    }
}
```

---

### 4. 返回局部变量的引用

```rust
// ❌ 错误：返回指向函数内部 String 的引用
//    函数结束时 String 被 drop，引用悬空
fn get_name() -> &str {   // 编译错误：missing lifetime specifier
    let name = String::from("Alice");
    &name  // name 即将被销毁！
}
```

```rust
// ✅ 方法一：返回 String（转移所有权出去）
fn get_name() -> String {
    String::from("Alice")
}

// ✅ 方法二：引用输入参数（生命周期合法）
fn first_word(s: &str) -> &str {
    // 返回的 &str 和输入的 s 生命周期相同，合法
    s.split_whitespace().next().unwrap_or("")
}

// ✅ 方法三：返回 'static 字符串字面量
fn get_greeting() -> &'static str {
    "你好"  // 字符串字面量生命周期是整个程序
}
```

---

### 5. 到处 `.clone()` 逃避借用检查器

```rust
// ❌ 能编译，但过度 clone 浪费内存和时间
fn process(users: Vec<User>) {
    for user in users.clone() {    // 克隆整个 Vec
        println!("{}", user.name.clone()); // 多余的 clone
        save_to_db(user.name.clone());
    }
    println!("共 {} 个用户", users.len());
}
```

```rust
// ✅ 用引用，只在真正需要时 clone
fn process(users: &[User]) {       // 接受切片引用
    for user in users {
        println!("{}", user.name);
        save_to_db(&user.name);    // 传引用给函数
    }
    println!("共 {} 个用户", users.len());
}

// ✅ 如果函数确实需要拥有数据，在调用处决定是否 clone
process(&users);         // 不消耗 users
process(&users.clone()); // 调用者决定 clone
```

> **判断要不要 clone**：先问"能不能传引用？"，如果能，就不 clone。

---

## 类型 / 转换

### 6. 混淆 `&str` 和 `String`

```rust
// ❌ 常见困惑：函数参数用了 &String，限制了调用方
fn greet(name: &String) {
    println!("你好，{}", name);
}

greet(&"Alice".to_string()); // 必须先创建 String 再取引用，多此一举
```

```rust
// ✅ 参数用 &str，String 和 &str 都能传
fn greet(name: &str) {
    println!("你好，{}", name);
}

greet("Alice");               // &str 直接传
greet(&my_string);            // &String 自动 deref 成 &str
greet(&my_string[..]);        // 明确写法
```

**速记：**

| 场景 | 用这个 |
|------|-------|
| 函数参数（只读） | `&str` |
| 函数参数（需要所有权） | `String` |
| 结构体字段（自己拥有） | `String` |
| 结构体字段（借用，有生命周期） | `&'a str` |
| 返回值（从输入派生） | `&str` |
| 返回值（新创建的） | `String` |

---

### 7. 用 `as` 做类型转换导致截断

```rust
// ❌ as 转换会静默截断，不会报错
let big: i32 = 300;
let small: u8 = big as u8; // 300 % 256 = 44，不是你期望的结果！
println!("{}", small); // 44

let neg: i32 = -1;
let u: u32 = neg as u32; // 4294967295，绕回了
```

```rust
// ✅ 用 try_from / try_into，失败时得到 Err
use std::convert::TryFrom;

let big: i32 = 300;
match u8::try_from(big) {
    Ok(val) => println!("转换成功: {}", val),
    Err(e)  => println!("转换失败（值太大）: {}", e),
}

// 或者用 try_into（需要 use std::convert::TryInto）
use std::convert::TryInto;
let result: Result<u8, _> = big.try_into();

// ✅ as 只在这些场景安全使用
let x: f64 = 3.14;
let y: f32 = x as f32; // 浮点降精度，可接受
let idx: usize = 42_i32 as usize; // 已知非负，转 usize 下标
```

---

### 8. 整数溢出在 release 模式下静默回绕

```rust
// ❌ debug 模式会 panic，release 模式会静默回绕！
let x: u8 = 255;
let y = x + 1; // debug: panic；release: 0（回绕）
```

```rust
// ✅ 用显式的溢出处理方法，行为在两种模式下一致
let x: u8 = 255;

x.checked_add(1)    // -> Option<u8>，溢出返回 None
x.saturating_add(1) // -> u8，溢出时钳制到最大值 255
x.wrapping_add(1)   // -> u8，溢出时回绕，明确表达意图
x.overflowing_add(1) // -> (u8, bool)，返回结果和是否溢出
```

---

### 9. `collect()` 忘记标注类型

```rust
// ❌ 编译错误：无法推断 collect 的目标类型
let numbers = vec![1, 2, 3, 4, 5];
let evens = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .collect(); // 错误：type annotations needed
```

```rust
// ✅ 方法一：在变量上标注类型
let evens: Vec<&i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .collect();

// ✅ 方法二：用 turbofish 语法直接在 collect 上标注
let evens = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .collect::<Vec<_>>(); // _ 让编译器推断元素类型

// ✅ 方法三：收集为 HashSet、String 等其他类型
use std::collections::HashSet;
let unique: HashSet<i32> = vec![1, 2, 2, 3].into_iter().collect();

let chars: String = vec!['a', 'b', 'c'].into_iter().collect();
```

---

### 10. `iter()` / `iter_mut()` / `into_iter()` 用错

```rust
let v = vec![1, 2, 3];

// iter()      → 产生 &T，不消耗集合，可以继续使用 v
// iter_mut()  → 产生 &mut T，可修改元素，不消耗集合
// into_iter() → 产生 T，消耗集合，v 之后不可用
```

```rust
// ❌ 常见：into_iter 后还想用原 Vec
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.into_iter().map(|x| x * 2).collect();
println!("{:?}", v); // 编译错误：v 已被 move

// ✅ 用 iter() 保留所有权
let v = vec![1, 2, 3];
let doubled: Vec<i32> = v.iter().map(|&x| x * 2).collect();
println!("{:?}", v); // OK，v 还在

// ❌ 想修改元素却用了 iter()
let mut v = vec![1, 2, 3];
for x in v.iter() {
    *x += 1; // 编译错误：x 是 &i32，不可变
}

// ✅ 修改元素用 iter_mut()
for x in v.iter_mut() {
    *x += 1; // OK，x 是 &mut i32
}
```

---

## 错误处理

### 11. `unwrap()` 满天飞

```rust
// ❌ 任何地方都 unwrap()，生产代码 panic 风险高
fn load_user(id: &str) -> User {
    let file = std::fs::read_to_string("users.json").unwrap(); // 文件不存在直接 panic
    let users: Vec<User> = serde_json::from_str(&file).unwrap(); // 解析失败直接 panic
    users.into_iter().find(|u| u.id == id).unwrap() // 找不到直接 panic
}
```

```rust
// ✅ 用 ? 传播错误，让调用方决定怎么处理
fn load_user(id: &str) -> Result<User, Box<dyn std::error::Error>> {
    let file = std::fs::read_to_string("users.json")?;
    let users: Vec<User> = serde_json::from_str(&file)?;
    users.into_iter()
        .find(|u| u.id == id)
        .ok_or_else(|| format!("用户 {} 不存在", id).into())
}

// ✅ unwrap() / expect() 合理的使用场景
// 1. 测试代码中（panic 就是测试失败，可以接受）
// 2. 逻辑上确实不可能失败，并且能说清楚原因
let re = regex::Regex::new(r"^\d+$").expect("正则表达式是硬编码的，不会失败");
let mutex = std::sync::Mutex::new(0);
let val = mutex.lock().expect("锁不应该中毒，除非其他线程已经 panic");
```

---

### 12. 用嵌套 match 替代 `?` 运算符

```rust
// ❌ 嵌套 match 又深又难读
fn process(path: &str) -> Result<String, std::io::Error> {
    match std::fs::read_to_string(path) {
        Ok(content) => {
            match content.lines().next() {
                Some(line) => Ok(line.to_string()),
                None => Ok(String::new()),
            }
        }
        Err(e) => Err(e),
    }
}
```

```rust
// ✅ 用 ? 和组合子，代码扁平清晰
fn process(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let first_line = content.lines().next().unwrap_or("").to_string();
    Ok(first_line)
}

// ✅ Option 的组合子同理
fn find_first_even(v: &[i32]) -> Option<i32> {
    v.iter()
     .find(|&&x| x % 2 == 0)
     .copied() // 比 map(|&x| x) 更清晰
}
```

---

### 13. 在库代码中 panic

```rust
// ❌ 库函数直接 panic，调用方无法处理错误
pub fn parse_config(s: &str) -> Config {
    let v: serde_json::Value = serde_json::from_str(s).unwrap(); // 调用方拿到 panic，无能为力
    Config { name: v["name"].as_str().unwrap().to_string() }
}
```

```rust
// ✅ 库函数返回 Result，调用方自己决定如何处理
pub fn parse_config(s: &str) -> Result<Config, ConfigError> {
    let v: serde_json::Value = serde_json::from_str(s)
        .map_err(ConfigError::ParseFailed)?;
    let name = v["name"].as_str()
        .ok_or(ConfigError::MissingField("name"))?
        .to_string();
    Ok(Config { name })
}

// 规则：只有调用方"不可能"传入无效数据时，才用 panic
// 例如：Vec::get_unchecked（unsafe）、内部不变量断言（debug_assert!）
```

---

## Async

### 14. 在 async 函数里调用阻塞操作

```rust
// ❌ 阻塞整个 tokio 线程池，导致其他 async 任务饿死
async fn fetch_data() -> String {
    std::thread::sleep(std::time::Duration::from_secs(1)); // 阻塞！
    let content = std::fs::read_to_string("data.txt").unwrap(); // 阻塞！
    content
}
```

```rust
// ✅ 用 tokio 提供的异步版本
use tokio::time::sleep;
use tokio::fs;

async fn fetch_data() -> String {
    sleep(std::time::Duration::from_secs(1)).await; // 异步 sleep
    fs::read_to_string("data.txt").await.unwrap()   // 异步 IO
}

// ✅ 如果必须调用阻塞代码（如 CPU 密集型、旧库），用 spawn_blocking
async fn heavy_task() -> u64 {
    tokio::task::spawn_blocking(|| {
        // 在专用线程池里运行，不影响 async 运行时
        (1u64..1_000_000).sum()
    }).await.unwrap()
}
```

---

### 15. 忘记 `.await` 导致 Future 不执行

```rust
// ❌ 忘记 .await，Future 被创建但从未执行
async fn send_email(to: &str) -> Result<(), Error> { /* ... */ }

async fn notify_user() {
    send_email("alice@example.com"); // 没有 .await！这行什么都不做
    println!("通知已发送"); // 实际上根本没发
}
```

```rust
// ✅ 所有 async 函数调用都要 .await
async fn notify_user() -> Result<(), Error> {
    send_email("alice@example.com").await?; // 必须 .await
    println!("通知已发送");
    Ok(())
}

// clippy 会对未使用的 Future 发出警告：
// warning: unused implementer of `Future` that must be used
```

---

## 模式 / 结构

### 16. 用 if-else 链替代 `match`

```rust
// ❌ 枚举用 if-else，冗长且编译器不检查穷举
enum Status { Active, Inactive, Banned, Pending }

fn describe(s: &Status) -> &str {
    if let Status::Active = s { "活跃" }
    else if let Status::Inactive = s { "非活跃" }
    else if let Status::Banned = s { "封禁" }
    else { "其他" } // 万一新加枚举变体，这里不报错！
}
```

```rust
// ✅ match：穷举检查，新增变体时编译器报错
fn describe(s: &Status) -> &str {
    match s {
        Status::Active   => "活跃",
        Status::Inactive => "非活跃",
        Status::Banned   => "封禁",
        Status::Pending  => "待审核", // 忘写这个就编译不过
    }
}
```

---

### 17. 混淆 `Arc<Mutex<T>>` 和 `Rc<RefCell<T>>` 的使用场景

```rust
// ❌ 在多线程中用了 Rc<RefCell<T>>
use std::rc::Rc;
use std::cell::RefCell;

let data = Rc::new(RefCell::new(vec![1, 2, 3]));
std::thread::spawn(move || {
    data.borrow_mut().push(4); // 编译错误：Rc 不是 Send
});
```

```rust
// ✅ 多线程 → Arc<Mutex<T>>
use std::sync::{Arc, Mutex};

let data = Arc::new(Mutex::new(vec![1, 2, 3]));
let data_clone = Arc::clone(&data);
std::thread::spawn(move || {
    data_clone.lock().unwrap().push(4); // OK
});
```

**速记：**

| 场景 | 选择 |
|------|------|
| 单线程，多所有者 | `Rc<T>` |
| 单线程，多所有者 + 内部可变 | `Rc<RefCell<T>>` |
| 多线程，多所有者 | `Arc<T>` |
| 多线程，多所有者 + 内部可变 | `Arc<Mutex<T>>` |
| 多线程，读多写少 | `Arc<RwLock<T>>` |

---

### 18. 字符串拼接用 `+` 导致意外的所有权转移

```rust
// ❌ + 运算符会消耗左侧 String 的所有权
let s1 = String::from("Hello");
let s2 = String::from(", world");
let s3 = s1 + &s2; // s1 被 move 进去了！
println!("{}", s1); // 编译错误：s1 已被移走
```

```rust
// ✅ 多个字符串拼接用 format!，不转移任何所有权
let s1 = String::from("Hello");
let s2 = String::from(", world");
let s3 = format!("{}{}", s1, s2); // s1 和 s2 都还在
println!("{} and {}", s1, s2); // OK

// ✅ 或者明确用 push_str / push（知道只有一个字符串在增长时）
let mut result = String::new();
result.push_str(&s1);
result.push_str(&s2);
result.push('!');
```

---

### 19. 结构体实现方法时 `self` 选错

```rust
struct Counter {
    count: u32,
    name: String,
}

impl Counter {
    // ❌ 用了 self（消耗），调用后 counter 就没了
    fn increment(self) -> Counter {
        Counter { count: self.count + 1, ..self }
    }

    // ❌ 用了 &mut self 但实际只读，不必要的限制
    fn get_count(&mut self) -> u32 {
        self.count  // 只是读，不需要 mut
    }

    // ❌ 用了 &self 但需要修改
    fn reset(&self) {
        self.count = 0; // 编译错误：&self 不可变
    }
}
```

```rust
impl Counter {
    // ✅ 只读操作用 &self
    fn get_count(&self) -> u32 { self.count }
    fn get_name(&self) -> &str { &self.name }

    // ✅ 修改操作用 &mut self
    fn increment(&mut self) { self.count += 1; }
    fn reset(&mut self) { self.count = 0; }

    // ✅ 消耗自身并转换时才用 self（Builder 模式、into 转换）
    fn into_name(self) -> String { self.name }
}
```

**速记：** `&self` 读 → `&mut self` 改 → `self` 销毁/转换

---

### 20. 忽略 clippy 警告

```bash
# ❌ 从不运行 clippy，积累了大量"能跑但不地道"的代码
```

```bash
# ✅ 养成习惯
cargo clippy            # 运行检查
cargo clippy -- -D warnings  # 把警告当错误（CI 中常用）
```

Clippy 能发现的常见问题：

```rust
// clippy 会提示这些写法有更好的替代

// 1. 用 is_empty() 替代长度判断
if v.len() == 0 { ... }     // ❌
if v.is_empty() { ... }     // ✅

// 2. 用 ? 替代 match + return
match result {
    Ok(v) => v,
    Err(e) => return Err(e), // ❌
}
let v = result?;              // ✅

// 3. 用 map_or 替代 match
match opt {
    Some(v) => v * 2,
    None => 0,               // ❌
}
opt.map_or(0, |v| v * 2);   // ✅

// 4. 避免不必要的 collect 再迭代
v.iter().collect::<Vec<_>>().iter()... // ❌
v.iter()...                            // ✅

// 5. 用 ..= 替代 >= && <=
if x >= 1 && x <= 10 { ... }  // ❌
if (1..=10).contains(&x) { }  // ✅
```

---

## 总结：最高频的坑

| 排名 | 错误 | 一句话 |
|------|------|-------|
| 🥇 | `unwrap()` 满天飞 | 用 `?` 传播，用 `expect` 说清原因 |
| 🥈 | Move 后继续用值 | 参数尽量用 `&T`，不用 `T` |
| 🥉 | 到处 `clone()` | 先考虑借用，再考虑 clone |
| 4 | `&String` 参数 | 改成 `&str`，更通用 |
| 5 | async 里用阻塞 IO | 用 tokio::fs、tokio::time |
| 6 | 忘记 `.await` | Future 不 await 什么都不做 |
| 7 | `Arc` vs `Rc` 用混 | 多线程用 `Arc`，单线程用 `Rc` |
| 8 | `as` 类型转换截断 | 用 `try_into()` 处理可能失败的转换 |
