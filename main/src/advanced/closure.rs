/*
闭包 Closure
迭代器 Iterator
模式匹配
枚举
*/


/*
Rust 闭包 (Closure) 总结
1. 什么是闭包？
闭包是匿名函数，可以赋值给变量或作为参数传递给函数。与普通函数不同，它可以捕获调用时作用域中的变量。

fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;



fn main() {
    let x = 1;
    let sum = |y| x + y;//闭包
    assert_eq!(3, sum(2)); // sum(2) = 1 + 2
}

2. 使用闭包简化代码
传统函数实现
fn muuuuu(intensity: u32) -> u32 {
    println!("muuuu.....");
    intensity
}

fn workout(intensity: u32) {
    println!("做 {} 个俯卧撑!", muuuuu(intensity));
    println!("再来 {} 组卧推!", muuuuu(intensity));
}
如果 muuuuu 需要修改，所有调用点都要改。

使用闭包
fn workout(intensity: u32) {
    let action = || {
        println!("muuuu.....");
        intensity
    };

    println!("做 {} 个俯卧撑!", action());
    println!("再来 {} 组卧推!", action());
}
只需要修改 action 的实现，其他代码无需变动。

3. 闭包的语法

let sum = |x, y| x + y; // 简化版
let sum = |x: i32, y: i32| -> i32 { x + y }; // 带类型标注
4. 闭包的类型推导
Rust 会自动推导闭包参数和返回值的类型：


let sum = |x, y| x + y; // 如果 sum(1, 2)，则推导 x, y 为 i32
如果没有调用，编译器会要求显式指定类型。

*/

// 5. 结构体中存储闭包

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    query: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,//这里 Fn(u32) -> u32 表示 query 必须是一个闭包或函数，接收 u32 返回 u32。
{
    fn new(query: T) -> Cacher<T> {
        Cacher { query, value: None }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.query)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}


/*


6. 闭包的 Fn 特征
Rust 闭包有三种不同的 Fn 特征：

FnOnce: 只能调用一次，获取变量所有权。
FnMut: 可变借用变量，多次调用。
Fn: 只借用变量，不修改，可多次调用。
示例
*/



fn main() {
    let s = String::from("hello");

    let print_s = || println!("{}", s);
    exec(print_s);  // 只借用，不修改
}

fn exec<F: Fn()>(f: F) {
    f();
}

/*

上面闭包 print_s 只读取 s，所以实现 Fn。

如果闭包修改变量：

*/


fn main() {
    let mut s = String::new();

    let mut update_string = |str| s.push_str(str);
    update_string("hello");

    println!("{}", s); // "hello"
}
/*
这里 update_string 修改了 s，所以是 FnMut。

如果闭包获取变量所有权：

*/

fn main() {
    let s = String::from("hello");

    let consume_s = move || println!("{}", s);
    consume_s();
    // println!("{}", s); // ❌ s 已被移动，无法再使用
}
// 使用 move 关键字让闭包获取 s 的所有权，因此它只能调用一次 (FnOnce)。


/*
7. 闭包作为函数返回值
闭包的类型是未知的，无法直接返回，但可以用 impl Fn 或 Box<dyn Fn>：
*/

fn factory(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
 }
//如果返回不同类型的闭包：


fn factory(x: i32) -> Box<dyn Fn(i32) -> i32> {
    if x > 1 {
        Box::new(move |y| x + y)
    } else {
        Box::new(move |y| x - y)
    }
}
//这里 Box<dyn Fn> 让不同类型的闭包可以共存。

/*

总结
闭包可以捕获作用域中的变量，而普通函数不能。
闭包支持自动类型推导，但只能使用一种类型。

闭包有三种 Fn 特征：
1. FnOnce: 获取所有权，调用一次。
2. FnMut: 可变借用，可多次调用。
3. Fn: 只借用，不修改，可多次调用。


闭包可存入结构体，但需要 Fn 特征约束。
闭包作为函数返回值 需要 impl Fn 或 Box<dyn Fn>。

Rust 的闭包提供了灵活性，同时结合所有权和借用机制，使其更加安全高效。

📘 TypeScript 对比
====================
| 特性 | Rust | TypeScript |
|------|------|-----------|
| 语法 | `\|x\| x + 1` | `(x) => x + 1` |
| 捕获方式 | Fn/FnMut/FnOnce（所有权级别） | 按引用捕获 |
| FnOnce | 获取所有权，只能调用一次 | 无对应 |
| FnMut | 可变借用 | 箭头函数可修改外部变量 |
| Fn | 只借用 | 默认行为 |
| move 关键字 | 强制获取所有权 | 无对应（箭头函数已捕获）|

⚠️ Rust 闭包最独特的地方：
- 编译器根据闭包体**自动推断**是 Fn/FnMut/FnOnce
- 如果你在闭包内修改了外部变量，自动变成 FnMut
- 如果你 move 了外部变量，自动变成 FnOnce
- TS 箭头函数没有这些分类——总是按引用捕获

详细对照 → rust_vs_typescript.rs §15 "闭包"