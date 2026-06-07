# Serde 实战指南

> serde + serde_json 是 Rust 业务开发的必备组件。本文覆盖从基础到进阶的所有日常用法。

```toml
# Cargo.toml
[dependencies]
serde      = { version = "1", features = ["derive"] }
serde_json = "1"
```

---

## 一、基础 derive

```rust
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id:       u64,
    username: String,
    email:    String,
    active:   bool,
}

// 序列化：Rust → JSON 字符串
let user = User { id: 1, username: "alice".into(), email: "a@b.com".into(), active: true };
let json = serde_json::to_string(&user)?;
// {"id":1,"username":"alice","email":"a@b.com","active":true}

let pretty = serde_json::to_string_pretty(&user)?;
// 带缩进的格式

// 反序列化：JSON 字符串 → Rust
let json = r#"{"id":2,"username":"bob","email":"b@c.com","active":false}"#;
let user: User = serde_json::from_str(json)?;

// 从 Value（动态 JSON 对象）反序列化
let value: serde_json::Value = serde_json::from_str(json)?;
let user: User = serde_json::from_value(value)?;
```

---

## 二、字段控制 Attribute

### 2.1 rename：字段名映射

```rust
#[derive(Serialize, Deserialize)]
struct Response {
    // JSON 字段名与 Rust 字段名不同
    #[serde(rename = "userId")]
    user_id: u64,

    #[serde(rename = "createdAt")]
    created_at: String,
}

// 批量命名规则（不用每个字段单独写）
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // snake_case → camelCase
struct ApiResponse {
    user_id:    u64,     // → "userId"
    first_name: String,  // → "firstName"
    is_active:  bool,    // → "isActive"
}

// 其他可选值：
// "snake_case"        user_id      → user_id
// "camelCase"         user_id      → userId
// "PascalCase"        user_id      → UserId
// "SCREAMING_SNAKE_CASE" user_id   → USER_ID
// "kebab-case"        user_id      → user-id

// 序列化和反序列化可以用不同命名规则
#[serde(rename_all(serialize = "camelCase", deserialize = "snake_case"))]
```

### 2.2 skip：跳过字段

```rust
#[derive(Serialize, Deserialize)]
struct User {
    id:       u64,
    username: String,

    // 序列化时跳过（不输出到 JSON）
    #[serde(skip_serializing)]
    password_hash: String,

    // 反序列化时跳过（JSON 中有也忽略），必须有默认值
    #[serde(skip_deserializing)]
    #[serde(default)]
    computed_score: f64,

    // 两个方向都跳过
    #[serde(skip)]
    internal_cache: Option<String>,
}

// 条件跳过：值满足条件时不输出
#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,

    // None 时不输出（而不是输出 null）
    #[serde(skip_serializing_if = "Option::is_none")]
    bio: Option<String>,

    // 空 Vec 时不输出
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,

    // 自定义条件
    #[serde(skip_serializing_if = "is_zero")]
    score: u32,
}

fn is_zero(n: &u32) -> bool { *n == 0 }
```

### 2.3 default：缺失字段的默认值

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    host: String,

    // JSON 没有 port 字段时，用 u16::default()（即 0）
    #[serde(default)]
    port: u16,

    // 用指定函数的返回值作为默认
    #[serde(default = "default_timeout")]
    timeout_secs: u64,

    // 用 Default::default()（空字符串、false、0 等）
    #[serde(default)]
    debug: bool,
}

fn default_timeout() -> u64 { 30 }

// 整个结构体所有字段都用 default（JSON 可以是 {}）
#[derive(Serialize, Deserialize, Default)]
#[serde(default)]
struct Options {
    verbose: bool,       // 缺失 → false
    max_retry: u32,      // 缺失 → 0
    prefix: String,      // 缺失 → ""
}
```

### 2.4 flatten：内联嵌套结构体

```rust
#[derive(Serialize, Deserialize)]
struct Pagination {
    page:     u32,
    per_page: u32,
    total:    u64,
}

#[derive(Serialize, Deserialize)]
struct UserListResponse {
    users: Vec<User>,

    // 把 Pagination 的字段内联到当前层（不产生嵌套 key）
    #[serde(flatten)]
    pagination: Pagination,
}

// 序列化结果：
// {
//   "users": [...],
//   "page": 1,          ← 直接在顶层，不是 "pagination": { ... }
//   "per_page": 20,
//   "total": 100
// }
```

### 2.5 alias：多个 JSON key 映射同一字段

```rust
#[derive(Deserialize)]
struct Event {
    // 接受 "timestamp"、"ts"、"time" 三种写法
    #[serde(alias = "ts", alias = "time")]
    timestamp: u64,
}
```

---

## 三、枚举的 JSON 表示

枚举是业务中最常见的多态场景，Serde 提供 4 种序列化方式。

```rust
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]   // 见下面各种模式
enum Notification {
    Email { to: String, subject: String },
    Sms   { phone: String, message: String },
    Push  { device_id: String },
}
```

### 3.1 external（默认，不加任何 tag）

```rust
#[derive(Serialize, Deserialize)]
enum Shape {
    Circle { radius: f64 },
    Rect   { width: f64, height: f64 },
}

// {"Circle":{"radius":1.0}}
// {"Rect":{"width":2.0,"height":3.0}}
```

### 3.2 internal tag（最常用于 API 响应）

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]        // "type" 字段内联在对象里
enum Event {
    UserCreated { user_id: u64, email: String },
    OrderPlaced { order_id: u64, amount: f64 },
}

// {"type":"UserCreated","user_id":1,"email":"a@b.com"}
// {"type":"OrderPlaced","order_id":42,"amount":99.9}
```

### 3.3 adjacent tag（tag 和 content 并列）

```rust
#[derive(Serialize, Deserialize)]
#[serde(tag = "kind", content = "payload")]
enum Message {
    Text(String),
    Binary(Vec<u8>),
}

// {"kind":"Text","payload":"hello"}
// {"kind":"Binary","payload":[1,2,3]}
```

### 3.4 untagged（靠结构推断，反序列化容易出歧义，慎用）

```rust
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Value {
    Int(i64),
    Float(f64),
    Text(String),
}

// 42       → Int(42)
// 3.14     → Float(3.14)
// "hello"  → Text("hello")
```

---

## 四、Option 的三种情况

业务开发中经常需要区分"字段不存在"、"字段为 null"、"字段有值"三种情况：

```rust
use serde::{Serialize, Deserialize};

// 普通 Option：
//   字段缺失 → 反序列化失败（除非加 default）
//   null     → None
//   有值     → Some(v)

// 需要区分"缺失"和"null"时，用 Option<Option<T>>（或专门封装）
#[derive(Serialize, Deserialize, Debug)]
struct PatchUser {
    // None       = 不更新这个字段（客户端没传）
    // Some(None) = 把字段设为 null
    // Some(Some(v)) = 更新为新值
    #[serde(
        default,                              // 缺失 → None
        skip_serializing_if = "Option::is_none"
    )]
    email: Option<Option<String>>,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    bio: Option<Option<String>>,
}

// PATCH 请求场景：
// {}                     → email=None(不更新), bio=None(不更新)
// {"email":null}         → email=Some(None)(清空), bio=None(不更新)
// {"email":"a@b.com"}    → email=Some(Some("a@b.com"))(更新), bio=None(不更新)
```

---

## 五、自定义序列化/反序列化

### 5.1 with 模块（最常用：日期、特殊格式）

```rust
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{DateTime, Utc};

// 把 DateTime 序列化为 Unix 时间戳（整数），而不是 ISO 字符串
mod ts_seconds {
    use super::*;

    pub fn serialize<S: Serializer>(dt: &DateTime<Utc>, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_i64(dt.timestamp())
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<DateTime<Utc>, D::Error> {
        use serde::de::Error;
        let ts = i64::deserialize(d)?;
        DateTime::from_timestamp(ts, 0).ok_or_else(|| D::Error::custom("invalid timestamp"))
    }
}

#[derive(Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(with = "ts_seconds")]
    created_at: DateTime<Utc>,  // JSON: 1234567890（整数）
}

// chrono 自带的 serde 支持（需要 features = ["serde"]）
// #[serde(with = "chrono::serde::ts_seconds")]       // Unix 秒
// #[serde(with = "chrono::serde::ts_milliseconds")] // Unix 毫秒
```

### 5.2 serialize_with / deserialize_with（只改一个方向）

```rust
fn serialize_uppercase<S: serde::Serializer>(s: &str, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&s.to_uppercase())
}

#[derive(Serialize, Deserialize)]
struct User {
    #[serde(serialize_with = "serialize_uppercase")]
    country_code: String,  // 序列化时转大写，反序列化正常
}
```

### 5.3 敏感字段脱敏

```rust
fn mask_sensitive<S: serde::Serializer>(_: &str, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_str("***")
}

#[derive(Serialize, Deserialize)]
struct Credentials {
    username: String,
    #[serde(serialize_with = "mask_sensitive")]
    password: String,   // 序列化输出 "***"，不泄露明文
}
```

---

## 六、动态 JSON：serde_json::Value

当结构不固定时（如元数据、配置扩展字段），用 `Value` 处理动态 JSON：

```rust
use serde_json::{Value, json, Map};

// json! 宏：直接构造 JSON 值
let v = json!({
    "name": "Alice",
    "age": 30,
    "tags": ["rust", "dev"],
    "address": null
});

// 访问
v["name"].as_str();           // Some("Alice")
v["age"].as_u64();            // Some(30)
v["missing"].is_null();       // true（不存在的 key 返回 null）
v.get("name");                // Option<&Value>

// 遍历对象
if let Value::Object(map) = &v {
    for (key, val) in map { println!("{key}: {val}"); }
}

// 遍历数组
if let Value::Array(arr) = &v["tags"] {
    for item in arr { println!("{item}"); }
}

// 动态构建 JSON 对象
let mut map = Map::new();
map.insert("key".to_string(), json!("value"));
map.insert("num".to_string(), json!(42));
let obj = Value::Object(map);

// 结构体 ↔ Value 转换
let user = User { id: 1, username: "bob".into() };
let value: Value = serde_json::to_value(&user)?;      // struct → Value
let user: User   = serde_json::from_value(value)?;    // Value  → struct

// 合并两个 JSON 对象
fn merge(base: &mut Value, patch: Value) {
    if let (Value::Object(base), Value::Object(patch)) = (base, patch) {
        for (k, v) in patch { base.insert(k, v); }
    }
}
```

### 结构体带任意额外字段

```rust
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
struct Flexible {
    id:   u64,
    name: String,
    // 所有未知字段都收进这个 map
    #[serde(flatten)]
    extra: HashMap<String, Value>,
}

// JSON: {"id":1,"name":"alice","custom_field":"xyz","another":123}
// extra = {"custom_field": "xyz", "another": 123}
```

---

## 七、与配置文件配合

```toml
# Cargo.toml
[dependencies]
config  = "0.14"
dotenvy = "0.15"
```

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AppConfig {
    pub database_url: String,
    pub port:         u16,
    pub jwt_secret:   String,
    #[serde(default = "default_log_level")]
    pub log_level:    String,
}

fn default_log_level() -> String { "info".to_string() }

fn load_config() -> Result<AppConfig, config::ConfigError> {
    // 加载 .env 文件（开发用）
    dotenvy::dotenv().ok();   // 文件不存在时不报错

    config::Config::builder()
        // 先读默认配置文件
        .add_source(config::File::with_name("config/default").required(false))
        // 再读环境特定配置（config/production.toml 等）
        .add_source(config::File::with_name(
            &format!("config/{}", std::env::var("APP_ENV").unwrap_or_else(|_| "dev".into()))
        ).required(false))
        // 环境变量优先级最高（APP_PORT → port）
        .add_source(config::Environment::with_prefix("APP").separator("_"))
        .build()?
        .try_deserialize()
}
```

---

## 八、常见坑与解决

```rust
// ─ 坑1：数字类型在 JSON 中可能是字符串 ─
// API 有时返回 "123" 而不是 123
#[derive(Deserialize)]
struct Order {
    #[serde(deserialize_with = "deserialize_string_or_number")]
    amount: u64,
}

fn deserialize_string_or_number<'de, D: serde::Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
    use serde::de::{self, Visitor};
    struct V;
    impl<'de> Visitor<'de> for V {
        type Value = u64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "number or string")
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<u64, E> { Ok(v) }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<u64, E> {
            v.parse().map_err(de::Error::custom)
        }
    }
    d.deserialize_any(V)
}

// ─ 坑2：Vec 中有坏数据，不想整体失败 ─
#[derive(Deserialize)]
struct List {
    // 反序列化失败的元素跳过，不整体 Err
    #[serde(deserialize_with = "deserialize_lossy_vec")]
    items: Vec<Item>,
}

fn deserialize_lossy_vec<'de, D, T>(d: D) -> Result<Vec<T>, D::Error>
where
    D: serde::Deserializer<'de>,
    T: Deserialize<'de>,
{
    let raw: Vec<serde_json::Value> = Vec::deserialize(d)?;
    Ok(raw.into_iter().filter_map(|v| serde_json::from_value(v).ok()).collect())
}

// ─ 坑3：枚举变体名大小写不一致 ─
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]   // "active" / "inactive"，不是 "Active"
enum Status { Active, Inactive }
```

---

## 速查表

```
#[serde(rename = "name")]                  字段重命名
#[serde(rename_all = "camelCase")]         批量命名规则
#[serde(skip)]                             序列化+反序列化都跳过
#[serde(skip_serializing)]                 只在序列化时跳过
#[serde(skip_serializing_if = "fn")]       条件跳过
#[serde(default)]                          缺失时用 Default::default()
#[serde(default = "fn")]                   缺失时用指定函数返回值
#[serde(flatten)]                          内联嵌套结构体字段
#[serde(alias = "other_name")]             接受多个 JSON key
#[serde(with = "module")]                  自定义序列化模块
#[serde(serialize_with = "fn")]            只自定义序列化
#[serde(deserialize_with = "fn")]          只自定义反序列化
#[serde(tag = "type")]                     枚举 internal tag
#[serde(tag = "t", content = "c")]         枚举 adjacent tag
#[serde(untagged)]                         枚举 untagged
```
