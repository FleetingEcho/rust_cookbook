# minigrep

一个类 grep 命令行工具，支持彩色输出和递归目录搜索。
基于 [Rust Book](https://doc.rust-lang.org/book/) 第 12 章项目扩展而来。

> Web 服务器部分已独立到 [`mini_web_server`](../mini_web_server/) 项目。

## 练习的 Rust 概念

- **生命周期** — `search<'a>` 将输出切片的生命周期绑定到输入字符串
- **Trait** — `Iterator`、`fmt::Display`
- **迭代器** — `lines().filter().collect()`，递归 `read_dir`
- **错误处理** — `Box<dyn Error>`、`?` 运算符
- **测试** — 单元测试，`#[cfg(test)]`
- **clap** — derive 宏风格的 CLI 参数解析
- **模块化** — `lib.rs` 存放库逻辑，`main.rs` 只负责 CLI 入口

## 使用方法

```bash
# 搜索单个文件
cargo run -- "fn" src/lib.rs

# 递归搜索目录（带彩色高亮）
cargo run -- "async" src/

# 忽略大小写
cargo run -- "rust" src/ --ignore-case
IGNORE_CASE=1 cargo run -- "rust" src/

# 禁用颜色（适合管道输出）
cargo run -- "fn" src/ --no-color

# 查看帮助
cargo run -- --help
```

## 测试

```bash
cargo test
```

## 项目结构

```
src/
├── main.rs    # clap CLI 入口，参数解析
└── lib.rs     # Config、run、search、highlight、递归搜索
```

## 依赖

| Crate | 用途 |
|---|---|
| `clap` | CLI 参数解析 |
| `colored` | 终端彩色输出 |
