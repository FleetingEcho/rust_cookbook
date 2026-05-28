# 基本 Cargo 用法

## 创建项目

```bash
cargo new foo          # 创建二进制项目
cargo new --lib bar    # 创建库项目
cargo test             # 运行测试
```

## Cargo.toml 依赖配置

```toml
[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1"                                              # 来自 crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自在线仓库
bar = { path = "../bar" }                                    # 来自本地路径
```

## 多个二进制文件

```
foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
```

使用 `--bin` 标志编译或运行特定二进制文件：

```bash
cargo run --bin my_other_bin
```
