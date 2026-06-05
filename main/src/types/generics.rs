use std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

pub fn test() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result); // 100

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result); //y

    create_and_print::<i64>();
}

/*
T 需要实现 PartialOrd

> 运算符需要 PartialOrd，否则编译器无法知道 T 是否可比较。
T 需要实现 Copy

list[0] 和 item 可能是非 Copy 类型（比如 String）。
Copy 确保 largest = item; 时不会发生所有权转移（否则需要 Clone）。

泛型约束的解释
T: PartialOrd → 使 T 可比较（支持 > 操作）。
T: Copy → 确保 T 是小数据类型（如 i32、char），直接复制，不转移所有权。

*/

/*
T: From<i32> → T 必须能够从 i32 类型转换（即 T 必须实现 From<i32> trait）。
T: Display → T 必须实现 Display trait，这样才能在 println! 中格式化输出。

100.into()：into() 是 From<T> trait 的方法，它会调用 T::from(100) 将 100 转换成 T 类型。
*/
fn create_and_print<T>()
where
    T: From<i32> + Display,
{
    let a: T = 100.into(); // 创建了类型为 T 的变量 a，它的初始值由 100 转换而来
    println!("a is: {}", a);
}

/*
✅ 泛型 T 必须满足两个约束：

实现 From<i32> → 确保可以从 i32 转换为 T。
实现 Display → 确保 T 可以被 println! 格式化输出。
✅ 运行时行为

create_and_print::<i64>() → 100.into() 变成 100i64，然后打印 a is: 100。
🚀 这个模式常用于构造泛型值，并确保它可以被转换和显示！
*/

//结构体中使用泛型

// struct Point<T> {
//     x: T,
//     y: T,
// }

// fn main() {
//     let integer = Point { x: 5, y: 10 };
//     let float = Point { x: 1.0, y: 4.0 };
// }

// 不同类型
struct Point<T, U> {
    x: T,
    y: U,
}
fn main() {
    let p = Point { x: 1, y: 1.1 };
}

/*
枚举中的泛型
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}


方法中使用泛型

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
这里的 Point<T> 不再是泛型声明，而是一个完整的结构体类型，因为我们定义的结构体就是 Point<T> 而不再是 Point

*/

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// impl Point<f32> {
//     fn distance_from_origin(&self) -> f32 {
//         (self.x.powi(2) + self.y.powi(2)).sqrt()
//     }
// }

fn generic_test() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

// [i32; 3] 和 [i32; 2] 确实是两个完全不同的类型，因此无法用同一个函数调用。

// 让 display_array 能打印任意长度的 i32 数组：
// fn display_array(arr: &[i32]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(&arr);
// }

//将 i32 改成所有类型的数组：
// fn display_array<T: std::fmt::Debug>(arr: &[T]) {
//     println!("{:?}", arr);
// }
// fn main() {
//     let arr: [i32; 3] = [1, 2, 3];
//     display_array(&arr);

//     let arr: [i32; 2] = [1, 2];
//     display_array(&arr);
// }

/*
const 泛型
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
fn main() {
    let arr: [i32; 3] = [1, 2, 3];
    display_array(arr);

    let arr: [i32; 2] = [1, 2];
    display_array(arr);
}
    N 就是 const 泛型，定义的语法是 const N: usize，表示 const 泛型 N ，它基于的值类型是 usize。
*/

// const fn，即常量函数。const fn 允许我们在编译期对函数进行求值，从而实现更高效、更灵活的代码设计。
// const fn add(a: usize, b: usize) -> usize {
//     a + b
// }

// const RESULT: usize = add(5, 10);

// fn main() {
//     println!("The result is: {}", RESULT);
// }

// 📘 TypeScript 对比
// ====================
// Rust 泛型 ≈ TS 泛型，但约束方式不同：
//
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 泛型约束 | `T: PartialOrd + Copy`（trait bound） | `T extends Comparable` |
// | 组合约束 | `+` 连接 | `&` 连接 |
// | where 子句 | `where T: Display` | 无等价语法 |
// | 编译期 | 单态化——每种类型生成独立代码 | 不适用（JS 运行时无类型）|
// | const 泛型 | ✅ `const N: usize` 编译期值参数 | ❌ 不支持 |
//
// ```ts
// // TS 泛型写法
// function largest<T extends { compareTo(a: T): number }>(list: T[]): T { ... }
// ```
// ```rust
// // Rust 泛型写法
// fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { ... }
// ```
//
// ⚠️ Rust 的 trait bound 写在泛型参数旁或 where 子句中，
//    TS 用 `extends` 关键字。功能类似，但 Rust 更严格。
//
// 详细对照 → rust_vs_typescript.rs §9 "泛型"
