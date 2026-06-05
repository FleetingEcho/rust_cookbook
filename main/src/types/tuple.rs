fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
}
fn example_tuple_destructure() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}

fn example_tuple_access() {
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;
}

//使用元组返回多个数值

fn example_tuple_return() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}

// 📘 TypeScript 对比
// ====================
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 元组类型 | `(i32, &str, bool)` | `[number, string, boolean]` |
// | 访问元素 | `t.0`, `t.1` | `t[0]`, `t[1]` |
// | 解构 | `let (x, y) = t` | `const [x, y] = t` |
// | 语义 | 异构定长集合 | 定长数组类型 |
//
// ⚠️ TS 的 tuple 只是数组类型的一种特殊标注，
//    运行时仍是普通数组。Rust 的元组是真正的不同类型。
//
// 详细对照 → rust_vs_typescript.rs §4 "复合类型"
