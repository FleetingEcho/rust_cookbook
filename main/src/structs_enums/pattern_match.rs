use std::str::FromStr;

fn main() {
    // Vec 是 Rust 的动态数组
    let mut stack = Vec::new();

    // 向数组尾部插入元素
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // 从数组尾部弹出元素，直到为空
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // 直接使用 vec! 宏初始化数组
    let v = vec!['a', 'b', 'c'];

    // 使用 iter().enumerate() 获取索引和值
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 测试 get_count_item 函数
    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    // 示例: if let
    let some_option_value: Option<i32> = Some(42);
    if let Some(x) = some_option_value {
        println!("{}", x);
    }

    // 示例: let-else
    let Some(y) = some_option_value else {
        return;
    };
    println!("{}", y);
}

// 解析字符串 "3 chairs" 这种格式，返回 (数量, 物品)
fn get_count_item(s: &str) -> (u64, &str) {
    let mut it = s.split(' ');

    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("无法解析计数项对: '{s}'");
    };

    let Ok(count) = u64::from_str(count_str) else {
        panic!("无法解析整数: '{count_str}'");
    };

    (count, item)
}

// 使用标准库的 Result<T, E>（std::result::Result 已通过 prelude 自动引入）

fn example_divide() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("结果: {}", result),
        Err(error) => println!("错误: {}", error),
    }

    match divide(10.0, 0.0) {
        Ok(result) => println!("结果: {}", result),
        Err(error) => println!("错误: {}", error),
    }

    let result = divide(10.0, 2.0);
    println!("直接获取结果: {}", result.unwrap()); // 5.0
                                                   //⚠ 注意：如果 result 是 Err，unwrap() 会触发 panic!，导致程序崩溃。
    if let Ok(value) = divide(10.0, 2.0) {
        println!("运算成功，值为 {}", value);
    }
}

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if y == 0.0 {
        Err(String::from("The divisor cannot be zero"))
    } else {
        Ok(x / y)
    }
}

fn divide_and_print(x: f64, y: f64) -> Result<(), String> {
    let result = divide(x, y)?; // If it fails, return Err directly
    println!("Calculation result: {}", result);
    Ok(())
}

fn example_divide_and_print() {
    let _ = divide_and_print(10.0, 2.0);
    let _ = divide_and_print(10.0, 0.0);
}

// OR use map
fn example_divide_map() {
    let result = divide(10.0, 2.0);
    result.map(|val| println!("成功的值: {}", val));
}
//如果 result 是 Ok(val)，则 map() 执行闭包，否则什么都不做。

// 📘 TypeScript 对比
// ====================
// 模式匹配 = 解构 + switch + 条件守卫 三合一
//
// | 模式 | Rust 示例 | TS 对应 |
// |------|-----------|---------|
// | 解构枚举 | `Message::Move{x,y} =>` | `switch(msg.kind) + 解构` |
// | 解构结构体 | `User { name, age } =>` | 对象解构 `const { name, age } = user` |
// | 解构元组 | `(a, b, ..) =>` | 数组解构 `const [a, b] = arr` |
// | 范围匹配 | `1..=5 =>` | `if (x >= 1 && x <= 5)` |
// | 匹配守卫 | `n if n > 0 =>` | `if (n > 0)` |
//
// 详细对照 → rust_vs_typescript.rs §5 "枚举与模式匹配"
