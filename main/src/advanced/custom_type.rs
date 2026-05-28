/*

深入 Rust 类型
Rust 的类型系统是其学习难度较高的部分之一，这里主要介绍 newtype、类型别名 (type)、!（永不返回类型）等内容。

1. newtype 模式
newtype 作用：

提供更好的可读性，避免直接使用基础类型（如 Meters(u32)）。
解决孤儿规则，允许为外部类型实现外部特征。
隐藏内部类型，限制对内部数据的访问。
示例 1：包装类型

struct Meters(u32);
与其直接使用 u32 作为距离单位，不如用 Meters 让代码更清晰。

示例 2：为外部类型实现外部特征
由于 孤儿规则，无法直接为 Vec<String> 实现 Display，但可以用 newtype 解决：

*/

use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}

/*
关键点：

struct Wrapper(Vec<String>) 创建 newtype。
self.0 访问内部 Vec<String> 并拼接字符串。
示例 3：限制错误使用
如果使用 newtype，不同单位的距离不能相加：

*/

struct Meters(u32);
struct MilliMeters(u32);

fn calculate_distance(d1: Meters, d2: Meters) -> Meters {
    Meters(d1.0 + d2.0)
}

fn main() {
    let d = calculate_distance(Meters(10), Meters(20));
    println!("目标地点距离你{}米", d.0);
}
//如果 calculate_distance(Meters(10), MilliMeters(20))，编译会报错！


/*
2. 类型别名 (type)
作用：

让代码更易读（但不会创建新类型）。
简化复杂类型标注。
示例 1：基本别名

type Meters = u32;

fn main() {
    let x: u32 = 5;
    let y: Meters = 5;
    println!("x + y = {}", x + y); // 类型兼容
}

区别 newtype：

newtype 创建的是 新类型，不能与原类型混用。
type 只是别名，Meters 仍然是 u32，可以直接相加。


示例 2：简化复杂类型

type Thunk = Box<dyn Fn() + Send + 'static>;

let f: Thunk = Box::new(|| println!("hi"));

fn takes_long_type(f: Thunk) {}
fn returns_long_type() -> Thunk {
    Box::new(|| println!("hello"))
}
优势：

省去 Box<dyn Fn() + Send + 'static> 这种冗长的写法，提高代码可读性。



示例 3：简化 Result<T, E>

type Result<T> = std::result::Result<T, std::io::Error>;

fn read_file() -> Result<String> {
    // 省略实际逻辑
    Ok(String::from("file content"))
}

Result<T> 代替 Result<T, std::io::Error>，避免重复定义 std::io::Error。







3. !（永不返回类型）
作用：

用于函数或表达式表示 永不返回。
避免 match 分支类型不匹配。
示例 1：修正 match 语句
以下代码会报错：


fn main() {
    let i = 2;
    let v = match i {
       0..=3 => i,            // 返回整数
       _ => println!("不合规定的值:{}", i), // 返回 `()`（单元类型）
    };
}
报错原因：match 语句的分支类型不一致。

示例 2：使用 panic! 解决

fn main() {
    let i = 2;
    let v = match i {
       0..=3 => i,
       _ => panic!("不合规定的值:{}", i), // `panic!` 返回 `!`
    };
}
为什么可以通过？

panic! 永不返回，没有类型，所以不会导致类型冲突。


总结
特性	作用	适用场景

newtype	创建新类型，避免误用，增强可读性	需要定义自定义类型或为外部类型实现外部特征

type	仅是别名，不是新类型	简化类型标注，如 Result<T>

!	永不返回，不影响 match 分支类型	panic!、死循环 (loop {})

Rust 通过这些类型机制确保代码安全性，同时提升可读性。
*/