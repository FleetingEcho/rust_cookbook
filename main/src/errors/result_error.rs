//Result<T, E> 用于可恢复错误，panic! 用于不可恢复错误


/*
Panic

当出现 panic! 时，程序提供了两种方式来处理终止流程：栈展开和直接终止。

其中，默认的方式就是 栈展开，这意味着 Rust 会回溯栈上数据和函数调用，因此也意味着更多的善后工作，好处是可以给出充分的报错信息和栈调用信息，便于事后的问题复盘。直接终止，顾名思义，不清理数据就直接退出程序，善后工作交与操作系统来负责。

例如下面的配置修改 Cargo.toml 文件，实现在 release 模式下遇到 panic 直接终止：


[profile.release]
panic = 'abort'

尽量不要在 main 线程中做太多任务，将这些任务交由子线程去做，就算子线程 panic 也不会导致整个程序的结束。

*/


/*
因为 panic 的触发方式比错误处理要简单，因此可以让代码更清晰，可读性也更加好，当我们的代码注定是正确时，你可以用 unwrap 等方法直接进行处理，反正也不可能 panic ：

use std::net::IpAddr;
let home: IpAddr = "127.0.0.1".parse().unwrap();
例如上面的例子，"127.0.0.1" 就是 ip 地址，因此我们知道 parse 方法一定会成功，那么就可以直接用 unwrap 方法进行处理。

当然，如果该字符串是来自于用户输入，那在实际项目中，就必须用错误处理的方式，而不是 unwrap，否则你的程序一天要崩溃几十万次吧！

可能导致全局有害状态时
有害状态大概分为几类：

非预期的错误
后续代码的运行会受到显著影响
内存安全的问题

*/



//可恢复的错误 Result

/*
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}


对返回的错误进行处理

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}

失败就 panic: unwrap 和 expect

use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

*/



//错误传播
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    // 打开文件，f是`Result<文件句柄,io::Error>`
    let f = File::open("hello.txt");

    let mut f = match f {
        // 打开文件成功，将file句柄赋值给f
        Ok(file) => file,
        // 打开文件失败，将错误返回(向上传播)
        Err(e) => return Err(e),
    };
    // 创建动态字符串s
    let mut s = String::new();
    // 从f文件句柄读取数据并写入s中
    match f.read_to_string(&mut s) {
        // 读取成功，返回Ok封装的字符串
        Ok(_) => Ok(s),
        // 将错误向上传播
        Err(e) => Err(e),
    }
}


//其实不如这种方便
fn open_file() -> Result<File, Box<dyn std::error::Error>> {
    let mut f = File::open("hello.txt")?;
    Ok(f)
}

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    // read_to_string是定义在std::io中的方法，因此需要在上面进行引用
    fs::read_to_string("hello.txt")
}

// ? 不仅仅可以用于 Result 的传播，还能用于 Option 的传播
// Result 通过 ? 返回错误，那么 Option 就通过 ? 返回 None：


fn first(arr: &[i32]) -> Option<&i32> {
   arr.get(0)
}

/*
会编译错误
fn first(arr: &[i32]) -> Option<&i32> {
   arr.get(0)?
}
这段代码无法通过编译，切记：? 操作符需要一个变量来承载正确的值，这个函数只会返回 Some(&i32) 或者 None，只有错误值能直接返回，正确的值不行，所以如果数组中存在 0 号元素，那么函数第二行使用 ? 后的返回类型为 &i32 而不是 Some(&i32)。因此 ? 只能用于以下形式：

let v = xxx()?;
xxx()?.yyy()?;

// 📘 TypeScript 对比
// ====================
// Rust 错误处理 ≈ TS 的 try/catch + 规范返回值
//
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 空值 | `Option<T>`（Some/None） | `null` / `undefined` |
// | 错误 | `Result<T, E>`（Ok/Err） | `throw Error` |
// | 错误传播 | `?` 运算符 | `try/catch` |
// | 强制处理 | ✅ `#[must_use]` 警告忽略 Result | ❌ 可忽略返回值 |
// | 链式操作 | `.map()`, `.and_then()`, `?` | `.then()`, try/catch |
//
// ⚠️ TS 开发者最需适应的：
// - Rust 不抛异常，错误是普通返回值
// - `?` 运算符 ≈ 自动 return Err（更接近 `throw` 但可见）
// - 处理 Option/Result 是常态，编译器强制你考虑"None 情况"
//
// 详细对照 → rust_vs_typescript.rs §12 "错误处理"