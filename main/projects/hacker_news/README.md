# Hacker News CLI

通过 Hacker News API 并发抓取热门文章的命令行工具。

## 练习的 Rust 概念

- **Async/await** — 使用 `tokio` 异步运行时
- **并发任务** — 两种模型对比：
  - `JoinSet`：spawn 所有任务，等全部完成后统一收集结果
  - `mpsc channel`：生产者-消费者模式，任务完成一个就发送一个
- **Serde** — `#[derive(Deserialize, Serialize)]`，字段重命名 `#[serde(rename = "type")]`
- **Trait 实现** — 为自定义 struct 实现 `fmt::Display`
- **Option 处理** — `unwrap_or`、`as_deref`
- **clap** — derive 宏风格的 CLI，使用枚举定义选项

## 使用方法

```bash
# 默认：JoinSet 模式抓取前 10 条
cargo run

# 抓取 20 条，保存到指定文件
cargo run -- --count 20 --output news.json

# 使用 mpsc channel 并发模型
cargo run -- --mode mpsc

# 查看帮助
cargo run -- --help
```

## 两种并发模型对比

| | JoinSet | mpsc channel |
|---|---|---|
| 模式 | Fork-join | 生产者-消费者 |
| 结果收集 | 所有任务完成后统一收集 | 任务完成一个就发送一个 |
| 适用场景 | 任务数量固定、需要全部结果 | 流水线处理、有背压需求的场景 |

两种方式在这里输出相同，重点是理解结构上的区别。

## 项目结构

```
src/
├── main.rs          # clap CLI、Args struct、模式分发
└── hacker_news.rs   # Story struct、fetch_stories_joinset、fetch_stories_mpsc
```

## 依赖

| Crate | 用途 |
|---|---|
| `tokio` | 异步运行时 |
| `reqwest` | HTTP 客户端 |
| `serde` / `serde_json` | JSON 序列化 |
| `anyhow` | 错误处理 |
| `clap` | CLI 参数解析 |
