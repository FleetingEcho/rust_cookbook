/*
# 创建二进制项目
cargo new foo

# 创建库项目
cargo new --lib bar

$ cargo test


[package]
name = "foo"
version = "0.1.0"
authors = ["mark"]

[dependencies]
clap = "2.27.1" # 来自 crates.io
rand = { git = "https://github.com/rust-lang-nursery/rand" } # 来自在线仓库
bar = { path = "../bar" } # 来自本地文件系统的路径



那么，如果我们想在同一个项目中包含两个二进制文件，该怎么办呢？

cargo 实际上支持这种需求。如我们之前所见，默认的二进制文件名是 main，但你可以通过在 bin/ 目录中放置额外的文件来添加其他二进制文件：

foo
├── Cargo.toml
└── src
    ├── main.rs
    └── bin
        └── my_other_bin.rs
如果要指示 cargo 只编译或运行特定的二进制文件，只需传递 --bin my_other_bin 标志，其中 my_other_bin 是我们想要处理的二进制文件的名称。



*/