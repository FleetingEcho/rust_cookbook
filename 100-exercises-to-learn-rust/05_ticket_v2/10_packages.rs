// 🔑 要点：Rust 项目可以同时包含库和二进制文件
// lib.rs = 库入口，main.rs = 二进制入口
// 二进制通过 use packages::hello_world 使用库

// 库部分：定义 hello_world 函数
pub fn hello_world() {
    println!("Hello, world!");
}

// 二进制入口
fn main() {
    hello_world();
}
