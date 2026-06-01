# Rust 必知必会的 crate 生态

> 语言本身只是起点，生态才是生产力。以下按"越常用越靠前"排列。

---

## 第一梯队：几乎每个项目都会用到

### 1. anyhow + thiserror — 错误处理双雄

```toml
[dependencies]
anyhow = "1"       # 应用层：不在乎具体错误类型
thiserror = "1"    # 库层：自定义错误枚举
```

**分工明确：**

```
┌─────────────┬────────────────────┬──────────────────────┐
│             │ thiserror          │ anyhow               │
├─────────────┼────────────────────┼──────────────────────┤
│ 谁用        │ 库的作者           │ 应用/二进制          │
│ 做什么      │ 定义错误类型       │ 快速传播错误         │
│ 返回值      │ Result<T, MyError> │ anyhow::Result<T>    │
│ ? 能把      │ 你的错误类型       │ 任何 Error 都行      │
│ 附加信息    │ #[error("...")]    │ .context("...")      │
└─────────────┴────────────────────┴──────────────────────┘
```

```rust
// 你的库：用 thiserror 定义清晰错误
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("配置文件 {0} 不存在")]
    NotFound(String),

    #[error("解析失败: {0}")]
    ParseFailed(#[from] toml::de::Error),
}

// 你的应用：用 anyhow 快速处理
use anyhow::Context;

fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("读取配置文件 {path} 失败"))?;
    let config: Config = toml::from_str(&content)?;  // anyhow 自动转
    Ok(config)
}
```

**一句话：** 写库用 thiserror，写应用用 anyhow。两个都学。

---

### 2. serde + serde_json — 序列化/反序列化

Rust 的 JSON 处理标准，没有替代品。

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    #[serde(default)]               // 缺省用默认值
    email: Option<String>,
    #[serde(rename = "created_at")]  // JSON 字段名映射
    created_at: String,
}

// 反序列化
let user: User = serde_json::from_str(json_str)?;

// 序列化
let json = serde_json::to_string_pretty(&user)?;

// 从文件
let user: User = serde_json::from_reader(std::fs::File::open("user.json")?)?;
```

**必学技巧：**

| 属性 | 作用 |
|------|------|
| `#[serde(flatten)]` | 展开嵌套结构，拍平到父级 |
| `#[serde(tag = "type")]` | 枚举的 JSON 标签 |
| `#[serde(untagged)]` | 枚举不额外加标签，靠字段区分 |
| `#[serde(skip_serializing_if = "Option::is_none")]` | None 字段不输出 |
| `#[serde(deny_unknown_fields)]` | 拒绝未知字段，严格校验 |

---

### 3. clap — 命令行参数解析

Rust 的 CLI 工具标配。

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "myapp", version, about = "我的 CLI 工具")]
struct Args {
    /// 输入文件路径
    #[arg(short, long)]
    input: String,

    /// 输出文件路径（可选）
    #[arg(short, long, default_value = "output.txt")]
    output: String,

    /// 详细模式
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}

fn main() {
    let args = Args::parse();
    println!("输入: {}", args.input);
}
```

**一句话：** 任何 CLI 工具都用 clap，别用 `std::env::args()` 手动解析。

---

### 4. chrono — 日期时间

标准库的 `std::time` 只有 Duration 和 Instant，没法处理"2024-01-01"这种日期。

```rust
use chrono::{NaiveDate, DateTime, Utc, Local, Duration, Datelike};

// 解析
let date = NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d")?;
let dt = DateTime::parse_from_rfc3339("2024-01-15T10:00:00Z")?;

// 计算
let tomorrow = Utc::now() + Duration::days(1);

// 格式化
println!("{}", Utc::now().format("%Y-%m-%d %H:%M:%S"));

// 获取年月日
let (y, m, d) = (date.year(), date.month(), date.day());
```

---

### 5. tracing / log — 日志

```rust
use tracing::{info, warn, error, debug};
use tracing_subscriber;

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter("myapp=info,myapp::critical_module=debug")
        .init();

    info!("服务启动");
    warn!("磁盘空间不足");
    error!("数据库连接失败: {}", err);
}
```

**tracing vs log：** tracing 是 log 的升级版，支持结构化日志、span（跟踪请求生命周期）。新项目直接用 tracing。

---

## 第二梯队：特定场景必备

### 6. reqwest — HTTP 客户端

```rust
let resp = reqwest::get("https://api.github.com/users/rust-lang")
    .await?
    .json::<serde_json::Value>()
    .await?;
```

特点：支持 async/await、HTTPS、重定向、cookie、JSON 自动解析。

### 7. axum / actix-web — Web 框架

| 框架 | 特点 |
|------|------|
| axum | tokio 官方出品的 async web 框架，类型安全、提取器模式 |
| actix-web | 性能极强，actor 模型，生态更成熟 |

**新手推荐 axum：** 和 tokio 生态无缝集成，提取器（Extractor）让参数解析极其简洁。

```rust
use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};

async fn get_user(
    Path(id): Path<u64>,
    Query(params): Query<HashMap<String, String>>,
) -> Json<User> {
    Json(User { id, name: "Alice".into() })
}

let app = Router::new()
    .route("/user/:id", get(get_user));
```

### 8. sqlx — 数据库

特点：编译期检查 SQL、不依赖 ORM、支持 PostgreSQL/MySQL/SQLite。

```rust
let row = sqlx::query!("SELECT id, name FROM users WHERE id = ?", user_id)
    .fetch_one(&pool)
    .await?;

println!("{}: {}", row.id, row.name);
```

编译期检查：SQL 语句里写 `?` 参数，表名和列名编译器会去连数据库验证。

### 9. tokio — 异步运行时

你已经知道它了。只说一句：**不要自己手写 async runtime，tokio 就是标准。**

相关的：

```toml
tokio = { version = "1", features = ["full"] }
tokio-util        # 异步工具：CancellationToken、DelayQueue
tokio-stream      # 把异步类型转成 Stream
futures           # FuturesUnordered、StreamExt、join! 等
```

---

### 10. itertools — 迭代器扩展

标准库的迭代器够用，但 itertools 有更多好用的组合子：

```rust
use itertools::Itertools;

// 按连续相同元素分组
let data = vec![1, 1, 2, 2, 2, 3];
let groups: Vec<Vec<_>> = data.into_iter().chunk_by(|&x| x).into_iter()
    .map(|(_, g)| g.collect())
    .collect();  // [[1,1], [2,2,2], [3]]

// 排列组合
for perm in (0..3).permutations(2) {
    println!("{:?}", perm);  // [0,1], [0,2], [1,0], [1,2], [2,0], [2,1]
}

// 在相邻元素上滑动
for (a, b) in vec![1,2,3,4].into_iter().tuple_windows::<(_,_)>() {
    println!("{a} {b}");  // (1,2) (2,3) (3,4)
}

// 选出前 n 个
let top3 = vec![3,1,4,1,5,9,2,6].into_iter()
    .sorted()
    .rev()
    .take(3)
    .collect::<Vec<_>>();  // [9, 5, 6]
```

---

## 第三梯队：进阶但非常有用

### 11. rayon — 并行迭代器

把 `.iter()` 改成 `.par_iter()` 就自动并行：

```rust
use rayon::prelude::*;

fn sum_of_squares(v: &[i32]) -> i32 {
    v.par_iter()                // 自动并行（多线程）
     .map(|x| x * x)
     .sum()
}
```

**不需要手动管理线程，不需要 channel，一行换并行。**

### 12. parking_lot — 更快更安全的锁

标准库 Mutex/RwLock 的替代品：

```rust
use parking_lot::Mutex;  // 用法和 std 一样，但更快

let data = Mutex::new(0);
{
    let mut guard = data.lock();  // 不像 std 需要 unwrap()
    *guard = 42;
}
```

优势：不会污染（poison）、没有 unwrap 负担、速度更快。

### 13. tempfile — 临时文件/目录

测试时非常有用：

```rust
use tempfile::{tempdir, NamedTempFile};

let dir = tempdir()?;
let file_path = dir.path().join("test.txt");
std::fs::write(&file_path, "hello")?;

// dir 离开作用域时，整个临时目录自动删除
```

### 14. regex — 正则表达式

```rust
use regex::Regex;

let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$")?;
assert!(re.is_match("2024-01-15"));
```

编译一次，复用多次。

### 15. uuid — 生成唯一 ID

```rust
let id = Uuid::new_v4();          // 随机 UUID
let id = Uuid::new_v7(Timestamp::now(UnixTimestamp));  // 时间排序 UUID
```

### 16. once_cell / lazy_static — 全局惰性初始化

```rust
use std::sync::OnceLock;

static CONFIG: OnceLock<Config> = OnceLock::new();

fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        Config::load("config.toml").expect("加载配置失败")
    })
}
```

Rust 1.80+ 的 `LazyLock` 和 `OnceLock` 已是标准库，可以不依赖外部 crate。

### 17. dashmap — 并发 HashMap

```rust
use dashmap::DashMap;

let map = DashMap::new();
map.insert("key", 42);

// 多个线程同时读写，不需要 Arc<Mutex<HashMap>>
```

适用于高并发读写的场景，替代 `Arc<Mutex<HashMap>>`。

### 18. indicatif — 进度条

```rust
let pb = indicatif::ProgressBar::new(100);
for i in 0..100 {
    do_work();
    pb.inc(1);
}
pb.finish_with_message("完成");
```

### 19. criterion — 基准测试

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n { 0 => 0, 1 => 1, _ => fibonacci(n-1) + fibonacci(n-2) }
}

fn bench(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, bench);
criterion_main!(benches);
```

---

## 速查表：什么时候用什么

| 需求 | crate |
|------|-------|
| 定义错误类型 | thiserror |
| 快速传播错误 | anyhow |
| JSON / 序列化 | serde + serde_json |
| 命令行参数 | clap |
| 日期时间 | chrono |
| 日志 | tracing |
| HTTP 客户端 | reqwest |
| Web 框架 | axum（新）/ actix-web（成熟） |
| 数据库 | sqlx |
| 异步运行时 | tokio |
| 迭代器增强 | itertools |
| 并行计算 | rayon |
| 更快的锁 | parking_lot |
| 临时文件 | tempfile |
| 正则 | regex |
| UUID | uuid |
| 全局惰性初始化 | 标准库 OnceLock/LazyLock（1.80+） |
| 并发 HashMap | dashmap |
| 进度条 | indicatif |
| 基准测试 | criterion |
| 配置文件 | toml / config / dotenvy |
| 静态文件/资源 | rust-embed / include_dir! |
| 测试数据生成 | fake / fakeit |
| 模糊测试/属性测试 | proptest / quickcheck |
| MCP / AI 工具 | rmcp |

---

## 学习路线

```
第一阶段：每个项目必备
  thiserror + anyhow → serde + serde_json → clap → chrono → tracing

第二阶段：按项目需求学
  Web → axum + reqwest + sqlx
  CLI → clap + anyhow + serde
  数据处理 → rayon + itertools + serde

第三阶段：遇到瓶颈时学
  性能 → criterion + parking_lot + dashmap
  可靠性 → proptest + tempfile
  并发 → tokio + rayon + dashmap
```
