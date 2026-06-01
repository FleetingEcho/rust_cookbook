# Rust 工程实战模式 30 讲
---

## 一、Builder 模式

Rust 里没有命名参数/默认参数，Builder 是替代方案。

### 典型场景：构造一个有很多可选参数的 struct

```rust
// 不用 builder 时：调用方要写一堆 None
let req = Request {
    url: "https://...".into(),
    method: Method::GET,
    headers: None,
    body: None,
    timeout: None,
    retries: None,
};

// 用 builder 时：只设置需要的
let req = RequestBuilder::new("https://...")
    .header("Authorization", "Bearer xxx")
    .timeout(30)
    .build();
```

### 最小实现模板

```rust
#[derive(Debug)]
struct Request {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout_secs: u64,
}

struct RequestBuilder {
    url: String,
    method: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
    timeout_secs: u64,
}

impl RequestBuilder {
    fn new(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method: "GET".into(),      // 默认值
            headers: vec![],
            body: None,
            timeout_secs: 30,          // 默认值
        }
    }

    // 每个 setter 返回 &mut self，支持链式调用
    fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((key.into(), value.into()));
        self
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    fn timeout(mut self, secs: u64) -> Self {
        self.timeout_secs = secs;
        self
    }

    fn build(self) -> Request {
        Request {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            timeout_secs: self.timeout_secs,
        }
    }
}

// 使用
let req = RequestBuilder::new("https://example.com/api")
    .header("Authorization", "Bearer token123")
    .timeout(60)
    .build();
```

### 工程中怎么用

真实项目里用 `derive_builder` 宏自动生成 Builder：

```rust
use derive_builder::Builder;

#[derive(Builder, Debug)]
struct Request {
    url: String,
    #[builder(default = "String::from(\"GET\")")]
    method: String,
    #[builder(default)]
    headers: Vec<(String, String)>,
    #[builder(default)]
    body: Option<String>,
    #[builder(default = "30")]
    timeout_secs: u64,
}
// 一行 #[derive(Builder)]，自动生成 RequestBuilder
```

---

## 二、Newtype 模式 + Deref

### 典型场景：给基础类型加上语义约束

```rust
// ❌ 裸 String：传错位置编译器也不报错
fn create_user(name: String, email: String) {}

create_user(email, name);  // 编译通过，逻辑错了！

// ✅ Newtype：编译器帮你检查
struct UserName(String);
struct Email(String);

fn create_user(name: UserName, email: Email) {}

create_user(Email("a@b.com".into()), UserName("Alice".into()));
// ❌ 编译错误：期望 UserName，传了 Email
```

### 配合 Deref 保留原方法

```rust
use std::ops::Deref;

struct Email(String);

impl Deref for Email {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

// 现在 Email 可以直接用 String/str 的所有方法
let email = Email("alice@example.com".into());

// 不用额外实现，直接能用：
email.contains('@');           // ✅ Deref 到 str
email.starts_with("alice");    // ✅
email.len();                   // ✅
```

### 工程价值

```rust
// serde 反序列化时自动验证
#[derive(Deserialize)]
struct CreateUserRequest {
    #[serde(with = "email_serde")]
    email: Email,
}

// 保证 Email 值永远是合法的
impl TryFrom<String> for Email {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.contains('@') {
            Ok(Email(value))
        } else {
            Err("invalid email".into())
        }
    }
}
// 从此系统中不可能出现非法的 Email——类型系统保证了
```

---

## 三、RAII 守卫（RAII Guard）

### 典型场景：进入作用域自动获取资源，离开自动释放

Rust 的 `Drop` 是最强的工程模式之一：

```rust
struct Timer {
    name: String,
    start: std::time::Instant,
}

impl Timer {
    fn new(name: &str) -> Self {
        Self { name: name.into(), start: std::time::Instant::now() }
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        println!("{} took {:?}", self.name, self.start.elapsed());
    }
}

// 使用：离开作用域自动打印耗时
fn compute() {
    let _timer = Timer::new("compute");
    // ... 复杂的业务逻辑 ...
    // `_timer` 在这里被 drop，自动输出耗时
}
```

### Mutex 的 Guard 也是 RAII

```rust
let data = Arc::new(Mutex::new(vec![1, 2, 3]));

{
    let mut guard = data.lock().unwrap();  // 自动加锁
    guard.push(4);                         // 修改数据
}  // guard 被 drop，自动解锁——永远不会有死锁忘记解锁的问题
```

### 工程中自建 RAII 守卫

```rust
// 数据库连接池的"借用计时"
struct ConnectionGuard<'a> {
    pool: &'a mut u32,  // 可用连接数
}

impl Drop for ConnectionGuard<'_> {
    fn drop(&mut self) {
        *self.pool += 1;  // 归还连接
        println!("connection returned");
    }
}

fn get_connection(pool: &mut u32) -> ConnectionGuard<'_> {
    *pool -= 1;
    ConnectionGuard { pool }
}
```

---

## 四、Typestate 模式（编译期状态机）

### 典型场景：编译期确保操作顺序

```rust
// 状态：用零大小类型表示
struct Unauthenticated;
struct Authenticated { token: String };

struct ApiClient<S> {
    base_url: String,
    state: S,
}

impl ApiClient<Unauthenticated> {
    fn new(url: &str) -> Self {
        Self { base_url: url.into(), state: Unauthenticated }
    }

    // login 只能在 Unauthenticated 状态调用
    fn login(self, username: &str, password: &str) -> ApiClient<Authenticated> {
        // ... 实际登录，拿到 token
        ApiClient {
            base_url: self.base_url,
            state: Authenticated { token: "xxx".into() },
        }
    }
}

impl ApiClient<Authenticated> {
    // get 只能在 Authenticated 状态调用
    fn get(&self, path: &str) -> String {
        format!("GET {} with token {}", path, self.state.token)
    }
}

// 编译期检查：
let client = ApiClient::new("https://api.example.com");
// client.get("/data");  // ❌ 编译错误！Unauthenticated 没有 get 方法
let client = client.login("admin", "password");
client.get("/data");  // ✅ 现在有 token 了
```

### 工程价值

把运行时才能发现的错误提前到编译期。例如 HTTP 响应的解析：

```rust
struct Response;
struct Parsed<T>(T);

fn parse_json<T: DeserializeOwned>(resp: Response) -> Result<Parsed<T>, Error>;
// 只有 Parsed 状态才能访问数据，未解析时不能碰
```

---

## 五、Cow：写时复制

### 典型场景：大部分只读、偶尔修改

```rust
use std::borrow::Cow;

fn normalize(name: &str) -> Cow<str> {
    if name.contains(' ') {
        // 只有真正需要修改时才 clone
        Cow::Owned(name.replace(' ', "_"))
    } else {
        // 不需要修改，直接返回引用
        Cow::Borrowed(name)
    }
}

// 用的时候：
let result = normalize("hello world");
// 不管返回 Owned 还是 Borrowed，都能当 &str 用
println!("{}", result.as_ref());

let result = normalize("hello");
// 没有空格，零分配
```

### 不用 Cow 的代价

```rust
// ❌ 不管有没有空格，都要分配
fn normalize(name: &str) -> String {
    name.replace(' ', "_")  // 没空格也创建新 String
}

// ✅ Cow：只在需要时才分配
fn normalize(name: &str) -> Cow<str> { ... }
```

### 工程中的常见用法

```rust
// serde 反序列化：字符串默认用 Cow 避免不必要分配
#[derive(Deserialize)]
struct Config<'a> {
    #[serde(borrow)]
    name: Cow<'a, str>,  // 从输入直接借用，除非需要转义
}
```

---

## 六、split borrow：同时借用一个 struct 的多个字段

### 典型场景：一个 struct 有多个字段，要同时修改两个

```rust
struct Canvas {
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl Canvas {
    fn resize(&mut self, new_w: u32, new_h: u32) {
        // ❌ 编译错误：不能同时借用 self.width 和 self.pixels
        // let old = self.width;
        // self.pixels.resize((new_w * new_h) as usize, 0);

        // ✅ 方案1：先把值取出来
        let old_w = self.width;
        let old_h = self.height;
        self.pixels.resize((new_w * new_h) as usize, 0);
        self.width = new_w;
        self.height = new_h;
    }
}
```

### 更复杂的场景：同时借两个字段

```rust
// ✅ 方案2：拆开借用——编译器能识别 struct 的不同字段是独立的
struct Point { x: i32, y: i32 }

fn swap(p: &mut Point) {
    let x = &mut p.x;  // 只借用了 x
    let y = &mut p.y;  // 只借用了 y——允许，因为不同字段
    std::mem::swap(x, y);
}
```

### 函数调用时

```rust
struct Order {
    buyer: String,
    seller: String,
    amount: u64,
}

fn transfer(from: &mut String, to: &mut String, amount: u64) {
    // ...
}

// ❌ 编译错误：同时可变借用 self.buyer 和 self.seller
fn process(&mut self) {
    transfer(&mut self.buyer, &mut self.seller, self.amount);
}

// ✅ 方案：先拆开引用
fn process(&mut self) {
    let buyer = &mut self.buyer;
    let seller = &mut self.seller;
    transfer(buyer, seller, self.amount);
}
```

---

## 七、PhantomData：让类型系统记住你没有的字段

### 典型场景：泛型类型需要在编译期携带类型信息

```rust
use std::marker::PhantomData;

// 表示"这个 ID 关联的是 T 类型"
struct Id<T> {
    value: u64,
    _marker: PhantomData<T>,  // 零大小，只在编译期存在
}

// 不同实体类型用不同的 Id
struct User;
struct Product;

fn get_user(id: Id<User>) -> User { /* ... */ User }
fn get_product(id: Id<Product>) -> Product { /* ... */ Product }

// 编译器保证不会混淆：
let user_id = Id::<User> { value: 1, _marker: PhantomData };
let product_id = Id::<Product> { value: 1, _marker: PhantomData };

// get_user(product_id);  // ❌ 编译错误：期望 Id<User>，传了 Id<Product>
```

### Raw pointer 的所有权语义

```rust
struct MyBox<T> {
    ptr: *mut T,
    _marker: PhantomData<T>,  // 告诉编译器：MyBox"拥有"一个 T
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // 没有 PhantomData<T>，编译器不会调用 T 的 drop
        unsafe { drop(Box::from_raw(self.ptr)); }
    }
}
```

---

## 八、Into/From 转换链

### 典型场景：函数参数能接受多种类型

```rust
// ❌ 限制调用方只能传 String
fn greet(name: String) { println!("Hello, {name}"); }
greet("Alice".to_string());  // 必须 .to_string()

// ✅ 用 impl Into 放宽
fn greet(name: impl Into<String>) { println!("Hello, {}", name.into()); }
greet("Alice");               // &str → String，自动转换
greet(String::from("Bob"));   // String 也支持
```

### 多层转换链

```rust
struct Email(String);

// 可以从 &str 和 String 创建 Email
impl From<&str> for Email {
    fn from(s: &str) -> Self { Email(s.to_string()) }
}
impl From<String> for Email {
    fn from(s: String) -> Self { Email(s) }
}

fn send_email(to: impl Into<Email>) {
    let email = to.into();
    // ...
}

send_email("alice@example.com");  // ✅ &str
send_email(String::from("bob@example.com"));  // ✅ String
```

### 工程实战：Request 能接受多种 body 类型

```rust
fn post<T: Into<Body>>(url: &str, body: T) { /* ... */ }

// 可以直接传字符串、bytes、或者自定义类型
post("/api", "hello");
post("/api", b"binary data".to_vec());
post("/api", MyCustomPayload { ... });
```

---

## 九、条件编译 + feature gate

### 典型场景：同一份代码支持多种后端

```rust
// 编译时选择实现

// 方案1：feature gate
#[cfg(feature = "sqlite")]
fn connect() -> Database { Database::open("data.db") }

#[cfg(feature = "postgres")]
fn connect() -> Database {
    Database::connect("postgres://localhost/db")
}

// Cargo.toml:
// [features]
// default = ["sqlite"]
// sqlite = []
// postgres = []
```

### 编译时平台检测

```rust
#[cfg(target_os = "linux")]
fn open_url(url: &str) { std::process::Command::new("xdg-open").arg(url).spawn(); }

#[cfg(target_os = "macos")]
fn open_url(url: &str) { std::process::Command::new("open").arg(url).spawn(); }

#[cfg(target_os = "windows")]
fn open_url(url: &str) { std::process::Command::new("cmd").args(["/c", "start", url]).spawn(); }
```

### debug_only 宏

```rust
// 只在 debug 模式下生效
macro_rules! debug_only {
    ($($tt:tt)*) => {
        #[cfg(debug_assertions)]
        { $($tt)* }
    };
}

debug_only!({
    println!("debug 模式才有的日志");
});
```

### cfg! 宏：运行时也能检查

```rust
if cfg!(debug_assertions) {
    println!("debug 模式");
} else {
    println!("release 模式");
}

if cfg!(target_pointer_width = "64") {
    println!("64 位系统");
}
```

---

## 十、serde 工程技巧

### flatten：展开嵌套结构

```rust
#[derive(Serialize, Deserialize)]
struct Pagination {
    page: u32,
    page_size: u32,
    total: u64,
}

#[derive(Serialize, Deserialize)]
struct ListResponse {
    data: Vec<Item>,
    #[serde(flatten)]
    pagination: Pagination,
}

// JSON 输出：
// {
//   "data": [...],
//   "page": 1,
//   "page_size": 20,
//   "total": 100
// }
```

### untagged enum：不额外加标签

```rust
#[derive(Deserialize)]
#[serde(untagged)]
enum Value {
    Int(i32),
    Str(String),
    Arr(Vec<Value>),
}

// 可以反序列化任意 JSON 值，不加 "type" 字段
```

### rename：JSON 字段名映射

```rust
#[derive(Serialize, Deserialize)]
struct User {
    #[serde(rename = "firstName")]
    first_name: String,
    #[serde(rename = "lastName")]
    last_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    middle_name: Option<String>,
    #[serde(default)]
    age: u32,
}
```

---

## 十一、闭包作为 struct 字段

### 典型场景：策略模式 / 回调

```rust
struct Calculator {
    operation: Box<dyn Fn(i32, i32) -> i32>,
}

impl Calculator {
    fn new(op: impl Fn(i32, i32) -> i32 + 'static) -> Self {
        Self { operation: Box::new(op) }
    }

    fn calc(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}

let add = Calculator::new(|a, b| a + b);
let mul = Calculator::new(|a, b| a * b);

println!("{}", add.calc(3, 4));  // 7
println!("{}", mul.calc(3, 4));  // 12
```

### 用泛型避免 Box

```rust
// 不需要 trait object，编译期确定类型
struct Calculator<F: Fn(i32, i32) -> i32> {
    operation: F,
}

impl<F: Fn(i32, i32) -> i32> Calculator<F> {
    fn new(op: F) -> Self { Self { operation: op } }
    fn calc(&self, a: i32, b: i32) -> i32 { (self.operation)(a, b) }
}
```

### 什么时候用 Box 什么时候用泛型

| | 泛型 | `Box<dyn Fn>` |
|--|------|--------------|
| 性能 | 最优（可内联） | 有虚表调用开销 |
| 类型 | 编译期固定 | 运行时可变 |
| 存储在集合 | ❌ 每个泛型不同 | ✅ 可以放 Vec |
| 适用 | 每个实例用不同闭包 | 需要在运行时切换 |

---

## 十二、Default + 部分更新

### 典型场景：只修改部分字段

```rust
#[derive(Default, Debug)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
    retries: u32,
    tls: bool,
}

// 用 Default + 更新语法
let config = Config {
    host: "localhost".into(),
    port: 8080,
    ..Default::default()   // 其他字段用默认值
};
```

### 配合 builder 更灵活

```rust
#[derive(Default)]
struct Config {
    host: String,
    port: u16,
    timeout: u64,
    retries: u32,
    tls: bool,
}

// Partial 结构体：只包含要覆盖的字段
#[derive(Default)]
struct ConfigPatch {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
}

impl Config {
    fn apply(&mut self, patch: ConfigPatch) {
        if let Some(host) = patch.host { self.host = host; }
        if let Some(port) = patch.port { self.port = port; }
        if let Some(timeout) = patch.timeout { self.timeout = timeout; }
    }
}
```

---

## 十三、enum 变体携带不同类型的数据

### 典型场景：API 的统一响应类型

```rust
#[derive(Debug)]
enum ApiResponse<T> {
    Success { data: T, total: Option<u64> },
    Error { code: u32, message: String },
    Redirect { url: String },
}

impl<T> ApiResponse<T> {
    fn is_success(&self) -> bool {
        matches!(self, ApiResponse::Success { .. })
    }

    fn data(self) -> Option<T> {
        match self {
            ApiResponse::Success { data, .. } => Some(data),
            _ => None,
        }
    }

    fn error_message(&self) -> Option<&str> {
        match self {
            ApiResponse::Error { message, .. } => Some(message),
            _ => None,
        }
    }
}
```

### 错误传播：enum 自动转型

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(String),
    NotFound,
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

fn read_config() -> Result<String, AppError> {
    let content = std::fs::read_to_string("config.json")?;
    // io::Error → AppError::Io，自动转换（因为实现了 From）
    Ok(content)
}
```

---

## 十四、自定义迭代器

### 典型场景：遍历自身拥有的数据

```rust
struct Fibonacci {
    curr: u64,
    next: u64,
    max: u64,
}

impl Fibonacci {
    fn new(max: u64) -> Self { Self { curr: 0, next: 1, max } }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.curr > self.max { return None; }
        let current = self.curr;
        self.curr = self.next;
        self.next = current + self.next;
        Some(current)
    }
}

// 使用：可以配合所有迭代器适配器
for n in Fibonacci::new(100).filter(|&x| x % 2 == 0) {
    println!("even fib: {n}");
}
```

### 为你的 struct 实现 IntoIterator

```rust
struct TicketStore {
    tickets: Vec<Ticket>,
}

// 所有权的迭代器
impl IntoIterator for TicketStore {
    type Item = Ticket;
    type IntoIter = std::vec::IntoIter<Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.into_iter()
    }
}

// 引用的迭代器
impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = std::slice::Iter<'a, Ticket>;

    fn into_iter(self) -> Self::IntoIter {
        self.tickets.iter()
    }
}
```

---

## 十五、测试模式

### 测试模块的惯用组织

```rust
// 不放在单独的 tests/ 文件，而是放在每个模块里
#[cfg(test)]
mod tests {
    use super::*;  // 导入父模块的所有内容

    #[test]
    fn test_parse() {
        // ...
    }

    // 测试辅助函数：不在 #[test] 里，不会被当成测试
    fn setup() -> Config {
        Config::default()
    }
}
```

### should_panic + expected

```rust
#[test]
#[should_panic(expected = "divide by zero")]
fn test_divide_by_zero() {
    divide(1, 0);
}
```

### Result 类型的测试

```rust
#[test]
fn test_parse() -> Result<(), String> {
    let result = "42".parse::<i32>().map_err(|_| "parse failed")?;
    assert_eq!(result, 42);
    Ok(())   // 测试通过
    // 返回 Err(...) 测试失败
}
```

### 测试辅助模块

```rust
// 条件编译整个模块，只在测试时存在
#[cfg(test)]
pub(crate) mod test_helpers {
    pub fn make_test_ticket() -> Ticket {
        Ticket::new("title".into(), "desc".into(), "To-Do".into())
    }
}

// 产品代码里不能用 test_helpers
// #[cfg(test)] 保证了编译时不会包含
```

---

## 十六、map、and_then、or_else 组合

### 典型场景：Option/Result 链式处理

```rust
// Option 链
let name = Some("Alice");
let result = name
    .map(|s| s.to_lowercase())     // Some("alice")
    .filter(|s| s.len() > 3)       // Some("alice")
    .map(|s| format!("Hello, {s}")) // Some("Hello, alice")
    .unwrap_or("Hello, Guest".into());

// Result 链
let raw = "42".parse::<i32>()
    .map(|n| n * 2)                // Ok(84)
    .map_err(|e| format!("parse error: {e}"))  // Err→String
    .ok()                          // Option<i32>

// and_then：返回 Option/Result 的变换
let dice = Some(6);
let result = dice
    .and_then(|n| if n == 6 { Some("大成功".into()) } else { None })
    .or_else(|| Some("再扔一次".into()));
// and_then = flat_map：map 后展平 Option<Option<T>> → Option<T>
```

### 完整的工程写法

```rust
fn process_user_input(input: &str) -> Result<String, String> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return Err("输入为空".into());
    }

    // 链式处理
    let result = trimmed
        .parse::<i32>()
        .map_err(|_| "不是数字".to_string())?
        .checked_mul(2)
        .ok_or("结果溢出")?;

    Ok(format!("结果是: {result}"))
}
// checked_mul 返回 Option，用 ok_or 转成 Result
// ? 自动解包或返回错误
```

---

## 十七、collect 到不同类型

```rust
// 常见：collect 到 Vec
let v: Vec<i32> = (0..10).collect();

// collect 到 String
let s: String = ['a', 'b', 'c'].iter().collect();  // "abc"

// collect 到 HashSet（去重）
use std::collections::HashSet;
let set: HashSet<i32> = vec![1, 1, 2, 2, 3].into_iter().collect();

// collect 到 HashMap
use std::collections::HashMap;
let map: HashMap<&str, i32> = vec![("a",1), ("b",2)].into_iter().collect();

// collect 到 Result<Vec<T>, E> — 碰到 Err 就停止
let results = vec![Ok(1), Ok(2), Err("boom"), Ok(3)];
let collected: Result<Vec<i32>, &str> = results.into_iter().collect();
assert_eq!(collected, Err("boom"));  // 第一个 Err 就停

// collect 到 Option<Vec<T>>
let opts = vec![Some(1), Some(2), None, Some(3)];
let collected: Option<Vec<i32>> = opts.into_iter().collect();
assert_eq!(collected, None);  // 第一个 None 就停
```

---

## 十八、切片模式匹配

```rust
// 匹配固定长度
let arr = [1, 2, 3, 4, 5];
match arr {
    [first, second, rest @ ..] => {
        println!("first={first}, second={second}, rest={rest:?}");
    }
}

// 匹配开头和结尾
match &arr[..] {
    [first, .., last] => println!("first={first}, last={last}"),
    [] => println!("empty"),
}

// 安全取前三个
let first_three = match arr {
    [a, b, c, ..] => Some((a, b, c)),
    _ => None,
};
```

---

## 十九、Option 的 transpose

### 典型场景：Option<Result<T, E>> → Result<Option<T>, E>

```rust
// 场景：解析一批输入，有些可能解析失败
fn try_parse(s: &str) -> Option<i32> {
    s.parse().ok()
}

// 你可能想 map 后拿到 Option<Result<...>>
let inputs = vec!["42", "hello", "100"];
let results: Vec<Option<i32>> = inputs.iter().map(|s| try_parse(s)).collect();

// 但如果你有一个 Option<Result<T,E>>：
let opt_result: Option<Result<i32, &str>> = Some(Ok(42));

// transpose 互换内外层：
let result_opt: Result<Option<i32>, &str> = opt_result.transpose();
// Some(Ok(42)) → Ok(Some(42))
// Some(Err(e))  → Err(e)
// None          → Ok(None)
```

### 实战：parse + 默认值

```rust
fn parse_field(input: &str) -> Result<i32, String> {
    input.parse().map_err(|_| "not a number".to_string())
}

fn process(config: &[&str]) -> Result<Vec<i32>, String> {
    let first = config.first();  // Option<&&str>

    // 想把 Option 和 Result 一起处理：
    match first.map(|s| parse_field(s)).transpose()? {
        Some(val) => Ok(vec![val]),
        None => Ok(vec![]),
    }
}
```

---

## 二十、Deref 的隐式转换

```rust
// String → &str（最常见）
fn takes_str(s: &str) { println!("{s}"); }

let s = String::from("hello");
takes_str(&s);  // &String 自动 deref 成 &str

// Vec<T> → &[T]
fn takes_slice(s: &[i32]) { println!("len={}", s.len()); }

let v = vec![1, 2, 3];
takes_slice(&v);  // &Vec<i32> 自动 deref 成 &[i32]

// Box<T> → &T
fn takes_ref(r: &i32) { println!("{r}"); }

let b = Box::new(42);
takes_ref(&b);  // &Box<i32> 自动 deref 成 &i32

// &T 到 &T 的多层 deref
fn takes_str(s: &str) {}

let s = "hello".to_string();
let r = &s;         // &String
takes_str(r);       // &String → &str，自动 deref
```

### 自动 deref 的规则

```
&String     → &str       (String: Deref<Target=str>)
&Vec<T>     → &[T]       (Vec<T>: Deref<Target=[T]>)
&Box<T>     → &T         (Box<T>: Deref<Target=T>)
&Rc<T>      → &T         (Rc<T>: Deref<Target=T>)
&Arc<T>     → &T         (Arc<T>: Deref<Target=T>)
```

**所以函数参数用 `&str` / `&[T]` / `&T` 是最通用的，调用方传什么都能自动转。**

---

## 二十一、? 运算符 + From 自动转错误

### 原理

```rust
fn read_file() -> Result<String, AppError> {
    // io::Error → AppError::Io 自动转（因为实现了 From<io::Error>）
    let content = std::fs::read_to_string("file.txt")?;

    // parse 的 ParseIntError → AppError::Parse 也自动转
    let n: i32 = content.trim().parse()?;

    Ok(format!("result: {n}"))
}
```

### 自己的错误类型

```rust
#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(String),
    Validation { field: String, msg: String },
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self { AppError::Io(e) }
}

impl From<String> for AppError {
    fn from(s: String) -> Self { AppError::Parse(s) }
}
```

### 用 thiserror 省掉手写

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),   // 自动生成 From

    #[error("parse error: {0}")]
    Parse(String),

    #[error("{field}: {msg}")]
    Validation { field: String, msg: String },
}
// 没有 #[from] 的变体，需要手动 .map_err() 转换
```

---

## 二十二、matches! 宏

### 典型场景：只关心是否匹配，不关心值

```rust
// ❌ 写 match 只是为了判断 true/false
fn is_admin(role: &Role) -> bool {
    match role {
        Role::Admin => true,
        _ => false,
    }
}

// ✅ matches! 一行搞定
fn is_admin(role: &Role) -> bool {
    matches!(role, Role::Admin)
}
```

### 带条件的匹配

```rust
let x = Some(42);

matches!(x, Some(v) if v > 10);  // true
matches!(x, None);                // false
matches!(x, Some(42));            // true
```

### 多个可能性

```rust
enum HttpStatus { Ok, NotFound, ServerError, BadRequest }

fn is_error(status: &HttpStatus) -> bool {
    matches!(status, HttpStatus::NotFound | HttpStatus::ServerError)
}
```

---

## 二十三、std::mem 技巧

### take：替换成默认值，取出旧值

```rust
use std::mem;

let mut value: Option<String> = Some("hello".into());

// ❌ 手动取
// let old = value;  // 这样会把 value 变成 None
// value = Some("world".into());

// ✅ take：把 value 替换成 None，返回旧值
let old = mem::take(&mut value);  // old = Some("hello")，value = None
value = Some("world".into());
```

### replace：替换成指定值

```rust
let mut old = String::from("old");
let prev = mem::replace(&mut old, "new".into());
assert_eq!(prev, "old");
assert_eq!(old, "new");
```

### swap：交换两个可变引用

```rust
let mut a = 1;
let mut b = 2;
mem::swap(&mut a, &mut b);
assert_eq!(a, 2);
assert_eq!(b, 1);
```

### drop：提前释放

```rust
let big_data = load_huge_file();
let stats = compute_stats(&big_data);
drop(big_data);  // 提前释放，不用等到函数结束
save_stats(&stats);
```

---

## 二十四、as 转换 vs From/Into

```rust
// as：编译器转换，不安全（可能截断）
let x: u32 = 300;
let y: u8 = x as u8;     // 44，截断了！

// From/Into：安全转换，编译期检查或实现者保证不丢失
let x: u32 = 300;
let y: u64 = u64::from(x);  // ✅ 安全，不会丢失数据
// let y: u8 = u8::from(x);  // ❌ 编译错误，u8::from(u32) 没有实现
```

### 什么时候用 as，什么时候用 From

```rust
// as：类型大小不同，明确要截断
let ptr = &v as *const i32 as usize;  // 指针转地址

// From：同大小类型间转换（不会丢失数据）
let x: i32 = 42;
let y: f64 = x.into();  // i32 → f64，不会丢失

// as：字面量类型
let x = 255u8 as i8;    // 明确要截断成 -1

// try_into：可能失败的转换
let x: i64 = 500;
let y: u8 = x.try_into().unwrap_or(0);  // 超出 u8 范围，给默认值
```

---

## 更多资源

- 读完本文后，推荐看 Rust 标准库的 `std::collections` 文档——每个集合的"常见用法"部分
- https://doc.rust-lang.org/std/collections/index.html
- 实际项目中多看开源代码：tokio、serde、clap、axum 的源码

