/*
项目(Package)：可以用来构建、测试和分享包
工作空间(WorkSpace)：对于大型项目，可以进一步将多个包联合在一起，组织成工作空间
包(Crate)：一个由多个模块组成的树形结构，可以作为三方库进行分发，也可以生成可执行文件进行运行
模块(Module)：可以一个文件多个模块，也可以一个文件一个模块，模块可以被认为是真实项目中的代码组织单元


cargo new my-project   //Package

cargo new my-lib --lib

Package 是一个项目工程，而包只是一个编译单元


.
├── Cargo.toml             # Package 配置文件
├── Cargo.lock             # 依赖锁定文件
├── src
│   ├── main.rs            # 默认二进制 crate（如果是二进制包）
│   ├── lib.rs             # 库 crate（如果是库包）
│   └── bin/               # 额外的二进制 crate
│       ├── main1.rs       # 额外的二进制 crate
│       └── main2.rs
├── tests/                 # 集成测试
│   ├── some_integration_tests.rs
├── benches/               # 基准性能测试（Benchmark）
│   ├── simple_bench.rs
└── examples/              # 项目示例
    ├── simple_example.rs

*/

/*
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"  # 依赖的第三方库（库 crate）

[[bin]]
name = "custom_bin"
path = "src/bin/custom_bin.rs"  # 自定义二进制 crate 的入口文件

*/


/*
crate 是 Rust 的最小编译单元，可以是二进制或库。
package 是 Rust 项目，可以包含多个 crate。
workspace 可以管理多个 package，用于大型项目。
Cargo.toml 是 package 的配置文件。
src/main.rs 是默认的二进制 crate，src/lib.rs 是默认的库 crate。
src/bin/\*.rs 可以添加多个二进制 crate。
*/


/*
4. crate 之间的组织关系
在 Rust 中，一个 crate 内部可以组织多个 module，例如：

// src/lib.rs
pub mod utils {
    pub fn hello() {
        println!("Hello from utils!");
    }
}
然后在 main.rs 中使用：

use my_project::utils; // 使用库 crate 里面的 utils 模块

fn main() {
    utils::hello();
}
*/


/*
Rust 的路径主要分为 相对路径 和 绝对路径。

my_project/
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── utils.rs
│   ├── network/
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── server.rs
│   ├── models/
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   ├── order.rs

绝对路径 是从 crate 根目录（lib.rs 或 main.rs）开始的路径，使用 crate:: 作为起点。
绝对路径
// src/main.rs
mod utils; // 声明 utils.rs 作为一个模块
mod network; // 声明 network 目录作为一个模块
mod models; // 声明 models 目录作为一个模块

fn main() {
    // 绝对路径调用
    crate::utils::hello();
    crate::network::client::connect();
}

相对路径引用
相对路径 以 self::、super:: 或者直接使用本地模块名称开始：

self:: 访问当前模块内的子模块
super:: 访问当前模块的父模块（上一级）
直接写模块名访问当前模块的子模块


// src/network/mod.rs
pub mod client;
pub mod server;

pub fn init() {
    println!("Initializing network...");
}


// src/network/client.rs
pub fn connect() {
    println!("Client connected!");
}

pub fn reconnect() {
    // 使用相对路径调用 server 内部的方法
    super::server::restart();
}

// src/network/server.rs
pub fn restart() {
    println!("Server restarted!");
}


3. use 关键字优化路径
// src/main.rs
mod utils;
mod network;
mod models;

use crate::utils::hello;
use crate::network::client; // 引入 network::client 模块

fn main() {
    hello();
    client::connect(); // 直接调用，不需要写 crate::network::client::connect();
}

4. pub 关键字管理可见性
Rust 默认模块是私有的，如果想让外部模块可以访问，需要使用 pub 关键字。

// src/utils.rs
pub fn hello() {
    println!("Hello from utils!");
}


mod	声明一个新的模块
use	导入一个模块，使路径更简洁
crate	代表当前 crate 的根模块


self 其实就是引用自身模块中的项

*/



/*
引入模块还是函数
从使用简洁性来说，引入函数自然是更甚一筹，但是在某些时候，引入模块会更好：

需要引入同一个模块的多个函数
作用域中存在同名函数


同名的话
1. 按照模块划分
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}


2.使用as
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}




*/


/*
引入项再导出
当外部的模块项 A 被引入到当前模块中时，它的可见性自动被设置为私有的，如果你希望允许其它外部代码引用我们的模块项 A，那么可以对它进行再导出：


mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}

*/


/*
使用 {} 简化引入方式
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::collections::HashSet;

use std::cmp::Ordering;
use std::io;

use std::io;
use std::io::Write;


简化成
use std::collections::{HashMap,BTreeMap,HashSet};
use std::{cmp::Ordering, io};
use std::io::{self, Write};


self
上面使用到了模块章节提到的 self 关键字，用来替代模块自身，结合上一节中的 self，可以得出它在模块中的两个用途：

use self::xxx，表示加载当前模块中的 xxx。此时 self 可省略
use xxx::{self, yyy}，表示，加载当前路径下模块 xxx 本身，以及模块 xxx 下的 yyy

*/


/*
使用 * 引入模块下的所有项

use std::collections::*;
当使用 * 来引入的时候要格外小心，因为你很难知道到底哪些被引入到了当前作用域中，有哪些会和你自己程序中的名称相冲突：

*/



/*
pub mod a {
    pub const I: i32 = 3;

    // 仅 `a` 模块内部可见，因为不是pub
    fn private_function(x: i32) -> i32 {
        use self::b::c::J;
        x + J
    }

    // `a` 模块外部可见
    pub fn bar(z: i32) -> i32 {
        private_function(I) * z
    }

    pub fn foo(y: i32) -> i32 {
        private_function(I) + y
    }

    mod b {
        // `c` 仅 `a` 内部可见
        pub(in crate::a) mod c {
            pub(in crate::a) const J: i32 = 4;
        }
    }
}

fn main() {
    println!("bar: {}", a::bar(2)); // 3 + 4 = 7, 7 * 2 = 14
    println!("foo: {}", a::foo(2)); // 3 + 4 = 7, 7 + 2 = 9
}


pub	无限制，任何地方都可以访问
pub(crate)	限制在整个 crate（包）内可见
pub(self)	仅当前模块可见
pub(super)	仅父模块可见
pub(in path)	限制可见性到 path 指定的模块内

关键点
限制某个模块的可见性
pub(in crate::a) 使 b::c::J 只能在 a 模块内访问，避免了 main 或其他模块直接调用它。

封装内部实现细节
private_function 作为 a 内部的私有函数，确保 bar 和 foo 的内部逻辑对外部不可见，仅提供 bar 和 foo 作为接口。

父模块和祖先模块可见性控制

pub(super) 可以使子模块的函数对父模块可见，但不对更外层可见。
pub(in path) 允许更精准地控制可见性范围。

*/



mod my_mod {
    // 私有函数，仅 `my_mod` 内可见
    fn private_function() {
        println!("called `my_mod::private_function()`");
    }

    // `pub` 允许外部访问
    pub fn function() {
        println!("called `my_mod::function()`");
    }

    // 允许内部调用私有函数
    pub fn indirect_access() {
        print!("called `my_mod::indirect_access()`, that\n> ");
        private_function();
    }

    // 嵌套模块
    pub mod nested {
        pub fn function() {
            println!("called `my_mod::nested::function()`");
        }

        // 仅 `my_mod` 内可见
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called `my_mod::nested::public_function_in_my_mod()`");
        }

        // 仅 `nested` 内可见
        pub(self) fn public_function_in_nested() {
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        // 仅父模块 `my_mod` 内可见
        pub(super) fn public_function_in_super_mod() {
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_functions() {
        print!("called `my_mod::call_functions()`, that\n> ");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    // 限制在 crate 内 (pub(crate)) 仅在当前 crate 内可见。
    pub(crate) fn public_function_in_crate() {
        println!("called `my_mod::public_function_in_crate()`");
    }
}

fn main() {
    my_mod::function();
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_functions();
    my_mod::public_function_in_crate();
}

// 📘 TypeScript 对比
// ====================
// Rust 模块系统 ≈ TS 的 ES modules
//
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 导出 | `pub fn` / `pub struct` | `export function` / `export class` |
// | 导入 | `use crate::module::Item` | `import { Item } from './module'` |
// | 默认可见性 | 私有（`pub` 公开） | 导出才可见 |
// | 文件映射 | 模块树 = 文件树 | import 路径 |
// | 包管理器 | Cargo + crates.io | npm/yarn + registry |
// | 重导出 | `pub use` | `export * from` / `re-export` |
//
// ⚠️ 关键差异：
// - Rust 的模块默认**私有**，显式 `pub` 才公开
// - TS 的模块默认**不导出**，显式 `export` 才公开
// - Rust 模块树对应文件系统结构（`mod.rs` 或同名目录）
// - TS 用文件夹路径 + `index.ts` 组织
//
// 详细对照 → rust_vs_typescript.rs §23 "模块与包管理"

