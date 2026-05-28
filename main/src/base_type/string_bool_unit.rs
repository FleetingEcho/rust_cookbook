fn main() {
    let c = 'z';
    let z = 'ℤ';
    let g = '国';
    let heart_eyed_cat = '😻';
}

// 4 字节

fn main2() {
    let t = true;
    let f: bool = false; // 使用类型标注,显式指定f的类型
    if f {
        println!("这是段毫无意义的代码");
    }
}

macro_rules! my_print {
    ($msg:expr) => {
        println!(">>> {}", $msg);
    };
}

fn main3() {
    my_print!("Hello Rust!"); // 输出 >>> Hello Rust!
}

/*
println! 是 宏（macro），不是函数，所以需要 !。
宏 在编译期展开，支持 可变参数 和 格式化解析，比普通函数更灵活。

📘 TypeScript 对比
====================
| 特性 | Rust | TypeScript |
|------|------|-----------|
| 布尔类型 | `bool`: `true` / `false` | `boolean`: `true` / `false` |
| 单元类型 | `()` — 不返回值的函数默认返回 `()` | `void` — 不返回值的函数返回 `undefined` |
| 格式化输出 | `println!(\"val = {}\", x)` 宏 | `` console.log(`val = ${x}`) `` |
| 字符串字面量 | `"hello"` 是 `&str` | `"hello"` 是 `string` |
| 字符字面量 | `'A'` 是 `char`（4 字节） | `'A'` 是 `string`（无 char 类型）|

⚠️ Rust 的 `char` 是 4 字节 Unicode 标量值（UTF-32 码点），
    TS 没有单独的字符类型，字符就是长度为 1 的 string。
*/
