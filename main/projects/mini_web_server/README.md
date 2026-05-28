# mini_web_server

一个用 `async-std` 实现的迷你异步 HTTP 服务器，支持并发连接处理。
基于 [Rust Book](https://doc.rust-lang.org/book/) 第 20 章项目扩展而来。

## 练习的 Rust 概念

- **Async/await** — 使用 `async-std` 异步运行时
- **Trait 对象** — `impl Read + Write + Unpin` 作为泛型约束
- **`Pin` 与 `Unpin`** — 理解异步代码中对象为何不能随意移动
- **手动实现异步 Trait** — `poll_read`、`poll_write`、`poll_flush`
- **并发连接** — `for_each_concurrent` 同时处理多个请求
- **测试异步代码** — `MockTcpStream` 模拟 TCP 流，无需绑定真实端口

## 使用方法

```bash
cargo run
# 服务器启动在 http://127.0.0.1:8888
```

| 路由 | 行为 |
|---|---|
| `GET /` | 返回 `hello.html`，状态 200 |
| `GET /sleep` | 等待 5 秒后返回 `hello.html`（演示异步不阻塞） |
| 其他路由 | 返回 `404.html`，状态 404 |

## 测试

```bash
cargo test
```

测试用 `MockTcpStream` 直接调用 `handle_connection`，验证 HTTP 响应格式正确（状态行、`Content-Type`、`Content-Length`、响应体）。

## 项目结构

```
src/
├── main.rs      # 入口，启动服务器
├── server.rs    # 服务器逻辑 + MockTcpStream 测试
├── hello.html   # GET / 返回的页面
└── 404.html     # 未知路由返回的页面
```

## 依赖

| Crate | 用途 |
|---|---|
| `async-std` | 异步运行时 + TCP 监听 |
| `futures` | `StreamExt`，支持并发连接迭代 |
