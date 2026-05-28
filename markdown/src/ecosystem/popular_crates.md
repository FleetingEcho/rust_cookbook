# Rust 常用 Crates 速览

> Rust 生态中有大量高质量的第三方包（crates），这里列出日常开发中出现频率最高的那些，每个附带简短说明和核心用法示例。

---

## 目录

1. [序列化 — serde / serde_json](#1-序列化--serde--serde_json)
2. [HTTP 客户端 — reqwest](#2-http-客户端--reqwest)
3. [Web 框架 — axum](#3-web-框架--axum)
4. [命令行解析 — clap](#4-命令行解析--clap)
5. [异步运行时 — tokio](#5-异步运行时--tokio)
6. [错误处理 — anyhow / thiserror](#6-错误处理--anyhow--thiserror)
7. [日志 — tracing / log](#7-日志--tracing--log)
8. [数据并行 — rayon](#8-数据并行--rayon)
9. [日期时间 — chrono](#9-日期时间--chrono)
10. [随机数 — rand](#10-随机数--rand)
11. [正则表达式 — regex](#11-正则表达式--regex)
12. [数据库 — sqlx / diesel](#12-数据库--sqlx--diesel)
13. [UUID — uuid](#13-uuid--uuid)
14. [环境变量 — dotenvy](#14-环境变量--dotenvy)
15. [性能测试 — criterion](#15-性能测试--criterion)

---

## 1. 序列化 — `serde` / `serde_json`

**这是 Rust 生态中使用最广泛的 crate，几乎所有项目都会用到。**

`serde` 是序列化框架，`serde_json` / `serde_toml` / `serde_yaml` 等是具体格式的实现。

```toml
# Cargo.toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: Option<String>,
}

fn main() {
    let user = User {
        name: "Alice".to_string(),
        age: 30,
        email: Some("alice@example.com".to_string()),
    };

    // 序列化：struct → JSON 字符串
    let json = serde_json::to_string(&user).unwrap();
    println!("{}", json);
    // {"name":"Alice","age":30,"email":"alice@example.com"}

    // 反序列化：JSON 字符串 → struct
    let json_str = r#"{"name":"Bob","age":25,"email":null}"#;
    let parsed: User = serde_json::from_str(json_str).unwrap();
    println!("{:?}", parsed);

    // 解析为动态 JSON（类型不确定时）
    let v: serde_json::Value = serde_json::from_str(json_str).unwrap();
    println!("{}", v["name"]); // "Bob"
}
```

**常用属性：**

```rust
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename = "user_name")]   // JSON key 用不同名字
    username: String,

    #[serde(default)]                // 缺失时用 Default::default()
    retries: u32,

    #[serde(skip_serializing_if = "Option::is_none")]  // None 时不输出该字段
    token: Option<String>,
}
```

---

## 2. HTTP 客户端 — `reqwest`

Rust 中最流行的 HTTP 客户端，支持 async 和同步两种模式。

```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Post {
    id: u32,
    title: String,
    body: String,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // GET 请求
    let post: Post = reqwest::get("https://jsonplaceholder.typicode.com/posts/1")
        .await?
        .json()
        .await?;
    println!("{:?}", post);

    // POST 请求，发送 JSON body
    let client = reqwest::Client::new();
    let resp = client
        .post("https://httpbin.org/post")
        .header("Authorization", "Bearer my-token")
        .json(&serde_json::json!({ "key": "value" }))
        .send()
        .await?;

    println!("Status: {}", resp.status());
    Ok(())
}
```

---

## 3. Web 框架 — `axum`

基于 `tokio` + `tower` 的现代 Web 框架，类型安全，与 Rust 的 trait 系统深度结合。

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

```rust
use axum::{
    extract::{Path, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
}

#[derive(Deserialize)]
struct CreateUser {
    name: String,
}

// GET /users/:id
async fn get_user(Path(id): Path<u32>) -> Json<User> {
    Json(User { id, name: "Alice".to_string() })
}

// POST /users，从 body 读取 JSON
async fn create_user(Json(payload): Json<CreateUser>) -> Json<User> {
    Json(User { id: 1, name: payload.name })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/users/:id", get(get_user))
        .route("/users", post(create_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

> 类似框架还有 `actix-web`（性能极高，但 API 风格不同）。

---

## 4. 命令行解析 — `clap`

构建 CLI 工具的标准选择，支持子命令、参数验证、自动生成帮助文档。

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mytool", version = "1.0", about = "一个示例 CLI 工具")]
struct Cli {
    /// 输出详细信息
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 处理文件
    Process {
        /// 输入文件路径
        #[arg(short, long)]
        input: String,

        /// 输出文件路径（可选）
        #[arg(short, long, default_value = "output.txt")]
        output: String,
    },
    /// 显示状态
    Status,
}

fn main() {
    let cli = Cli::parse();

    if cli.verbose {
        println!("详细模式已开启");
    }

    match cli.command {
        Commands::Process { input, output } => {
            println!("处理文件: {} -> {}", input, output);
        }
        Commands::Status => {
            println!("系统状态正常");
        }
    }
}
```

运行效果：
```
$ mytool --help
$ mytool process --input data.csv --output result.csv
$ mytool -v status
```

---

## 5. 异步运行时 — `tokio`

Rust 没有内置异步运行时，`tokio` 是使用最广泛的选择。

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

```rust
use tokio::time::{sleep, Duration};
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    // 并发执行多个任务
    let task1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "task1 完成"
    });

    let task2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "task2 完成"
    });

    let (r1, r2) = tokio::join!(task1, task2);
    println!("{}", r1.unwrap());
    println!("{}", r2.unwrap());

    // channel：任务间通信
    let (tx, mut rx) = mpsc::channel::<String>(32);

    tokio::spawn(async move {
        tx.send("消息".to_string()).await.unwrap();
    });

    if let Some(msg) = rx.recv().await {
        println!("收到: {}", msg);
    }
}
```

**常用 features：**

| feature | 包含内容 |
|---------|---------|
| `full` | 所有功能（开发时常用） |
| `rt-multi-thread` | 多线程运行时 |
| `net` | TCP/UDP 网络 |
| `fs` | 异步文件 IO |
| `time` | 定时器、sleep |
| `sync` | channel、Mutex 等 |

---

## 6. 错误处理 — `anyhow` / `thiserror`

这两个 crate 经常配合使用，职责不同：

- **`thiserror`**：定义库的错误类型（给别人调用的代码）
- **`anyhow`**：应用层错误处理，快速传播任意错误（写应用程序时用）

```toml
[dependencies]
thiserror = "1"
anyhow = "1"
```

```rust
// --- 用 thiserror 定义错误类型（适合库） ---
use thiserror::Error;

#[derive(Error, Debug)]
enum AppError {
    #[error("文件未找到: {0}")]
    FileNotFound(String),

    #[error("解析失败: {0}")]
    ParseError(#[from] std::num::ParseIntError),

    #[error("IO 错误")]
    Io(#[from] std::io::Error),
}

// --- 用 anyhow 处理错误（适合应用程序） ---
use anyhow::{Context, Result};

fn read_config(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取配置文件: {}", path))?;
    Ok(content)
}

fn main() -> Result<()> {
    let config = read_config("config.toml")?;
    println!("{}", config);
    Ok(())
}
```

---

## 7. 日志 — `tracing` / `log`

### `tracing`（推荐，async 友好）

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing::{info, warn, error, debug, instrument};

#[instrument]  // 自动记录函数入参和调用链
async fn process_request(user_id: u32) {
    info!(user_id, "开始处理请求");

    if user_id == 0 {
        warn!("收到无效的 user_id");
        return;
    }

    debug!("详细处理逻辑...");
    info!("请求处理完成");
}

#[tokio::main]
async fn main() {
    // 初始化日志输出到终端
    tracing_subscriber::fmt::init();

    process_request(42).await;
    process_request(0).await;
}
```

### `log` + `env_logger`（传统方式，更简单）

```toml
[dependencies]
log = "0.4"
env_logger = "0.11"
```

```rust
use log::{info, warn, error};

fn main() {
    env_logger::init(); // 读取 RUST_LOG 环境变量

    info!("程序启动");
    warn!("注意：配置文件缺失，使用默认值");
    error!("发生错误！");
}
```

```bash
RUST_LOG=debug cargo run
RUST_LOG=warn cargo run  # 只显示 warn 及以上
```

---

## 8. 数据并行 — `rayon`

只需把 `.iter()` 换成 `.par_iter()`，即可让迭代自动并行化，充分利用多核 CPU。

```toml
[dependencies]
rayon = "1"
```

```rust
use rayon::prelude::*;

fn main() {
    let numbers: Vec<i64> = (1..=1_000_000).collect();

    // 串行
    let sum_serial: i64 = numbers.iter().sum();

    // 并行（自动分配到多个线程）
    let sum_parallel: i64 = numbers.par_iter().sum();

    assert_eq!(sum_serial, sum_parallel);

    // 并行 map + filter
    let result: Vec<i64> = numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();

    println!("并行计算完成，结果数量: {}", result.len());
}
```

> CPU 密集型任务效果显著；IO 密集型任务用 `tokio` 更合适。

---

## 9. 日期时间 — `chrono`

```toml
[dependencies]
chrono = { version = "0.4", features = ["serde"] }
```

```rust
use chrono::{DateTime, Local, Utc, NaiveDate, Duration};

fn main() {
    // 获取当前时间
    let now_utc: DateTime<Utc> = Utc::now();
    let now_local: DateTime<Local> = Local::now();

    println!("UTC: {}", now_utc);
    println!("本地: {}", now_local.format("%Y-%m-%d %H:%M:%S"));

    // 日期计算
    let tomorrow = now_utc + Duration::days(1);
    let last_week = now_utc - Duration::weeks(1);

    // 解析日期字符串
    let date = NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d").unwrap();
    println!("解析日期: {}", date);

    // 与 serde 配合序列化
    // 字段标注 #[serde(with = "chrono::serde::ts_seconds")] 可序列化为时间戳
}
```

---

## 10. 随机数 — `rand`

```toml
[dependencies]
rand = "0.8"
```

```rust
use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();

    // 生成随机数
    let n: u32 = rng.gen_range(1..=100);
    println!("随机数 1-100: {}", n);

    let f: f64 = rng.gen(); // 0.0 到 1.0
    println!("随机浮点: {:.4}", f);

    // 随机 bool
    let coin: bool = rng.gen_bool(0.5);
    println!("抛硬币: {}", coin);

    // 打乱数组
    let mut items = vec![1, 2, 3, 4, 5];
    items.shuffle(&mut rng);
    println!("打乱后: {:?}", items);

    // 从数组中随机选一个
    let choice = items.choose(&mut rng).unwrap();
    println!("随机选择: {}", choice);
}
```

---

## 11. 正则表达式 — `regex`

```toml
[dependencies]
regex = "1"
```

```rust
use regex::Regex;

fn main() {
    let re = Regex::new(r"\b\d{4}-\d{2}-\d{2}\b").unwrap();
    let text = "今天是 2024-01-15，明天是 2024-01-16。";

    // 检测是否匹配
    println!("是否包含日期: {}", re.is_match(text));

    // 找到所有匹配
    for date in re.find_iter(text) {
        println!("找到日期: {}", date.as_str());
    }

    // 捕获组
    let re_email = Regex::new(r"(\w+)@(\w+)\.(\w+)").unwrap();
    if let Some(caps) = re_email.captures("user@example.com") {
        println!("用户名: {}", &caps[1]);
        println!("域名: {}", &caps[2]);
    }

    // 替换
    let result = re.replace_all(text, "[日期]");
    println!("{}", result);
}
```

> **性能提示**：`Regex::new` 编译较慢，应在程序启动时创建一次，用 `lazy_static!` 或 `std::sync::OnceLock` 存储为全局变量。

---

## 12. 数据库 — `sqlx` / `diesel`

### `sqlx`（异步，SQL 在编译期验证）

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "postgres"] }
tokio = { version = "1", features = ["full"] }
```

```rust
use sqlx::sqlite::SqlitePool;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePool::connect("sqlite:mydb.sqlite").await?;

    // 建表
    sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)")
        .execute(&pool)
        .await?;

    // 插入
    sqlx::query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("Alice")
        .bind("alice@example.com")
        .execute(&pool)
        .await?;

    // 查询，映射到 struct
    let users: Vec<User> = sqlx::query_as("SELECT * FROM users")
        .fetch_all(&pool)
        .await?;

    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}
```

### `diesel`（同步 ORM，编译期 schema 校验）

适合偏好 ORM 风格的场景，支持 PostgreSQL、MySQL、SQLite。配置相对复杂，通过 `diesel_cli` 管理 migration。

---

## 13. UUID — `uuid`

```toml
[dependencies]
uuid = { version = "1", features = ["v4", "serde"] }
```

```rust
use uuid::Uuid;

fn main() {
    // 生成随机 UUID v4
    let id = Uuid::new_v4();
    println!("{}", id); // 例：550e8400-e29b-41d4-a716-446655440000

    // 解析 UUID 字符串
    let parsed = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    println!("解析: {}", parsed);

    // 不带连字符的格式
    println!("简单格式: {}", id.simple());

    // 与 serde 配合，在结构体中直接序列化/反序列化
}
```

---

## 14. 环境变量 — `dotenvy`

从 `.env` 文件加载环境变量，开发时管理配置的标准方式。

```toml
[dependencies]
dotenvy = "0.15"
```

```env
# .env 文件
DATABASE_URL=postgres://user:pass@localhost/mydb
SECRET_KEY=my-secret-key
PORT=8080
```

```rust
use std::env;

fn main() {
    dotenvy::dotenv().ok(); // 加载 .env，文件不存在时不报错

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL 未设置");
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse()
        .expect("PORT 必须是数字");

    println!("连接数据库: {}", db_url);
    println!("监听端口: {}", port);
}
```

---

## 15. 性能测试 — `criterion`

比 Rust 内置 `#[bench]` 更强大的基准测试框架，统计稳定可靠。

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

```rust
// benches/my_benchmark.rs
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("fibonacci 20", |b| {
        b.iter(|| fibonacci(black_box(20)))
    });
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);
```

```bash
cargo bench                    # 运行并生成 HTML 报告
cargo bench -- fibonacci       # 只运行包含 "fibonacci" 的测试
```

---

## 快速参考

| 场景 | 推荐 Crate |
|------|-----------|
| JSON 序列化 | `serde` + `serde_json` |
| HTTP 请求 | `reqwest` |
| Web 服务器 | `axum` |
| CLI 工具 | `clap` |
| 异步运行时 | `tokio` |
| 应用层错误 | `anyhow` |
| 库错误类型 | `thiserror` |
| 结构化日志 | `tracing` |
| 数据并行 | `rayon` |
| 日期时间 | `chrono` |
| 随机数 | `rand` |
| 正则表达式 | `regex` |
| 数据库（异步）| `sqlx` |
| UUID | `uuid` |
| 环境变量 | `dotenvy` |
| 性能测试 | `criterion` |
