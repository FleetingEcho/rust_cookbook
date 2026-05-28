use std::fmt::Debug;

// Define the Point struct
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

// Define enums for demonstration
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

// Main function to demonstrate various pattern matching techniques
fn demonstrate_pattern_matching() {
    // Simple match expressions
    let x = 1;
    match x {
        1 | 2 => println!("One or two"), //✅
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    let x = 5;
    match x {
        1..=5 => println!("One through five"), //✅
        _ => println!("Something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("Early ASCII letter"),//✅
        'k'..='z' => println!("Late ASCII letter"),
        _ => println!("Something else"),
    }

    // Struct destructuring
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    println!("Destructured Point: a = {}, b = {}", a, b);//✅Destructured Point: a = 0, b = 7

    match p {
        Point { x, y: 0 } => println!("On the x-axis at {}", x),
        Point { x: 0, y } => println!("On the y-axis at {}", y),//✅
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    // Enum destructuring
    let msg = Message::ChangeColor(Color::Rgb(255, 160, 255));

    match msg {
        Message::Quit => println!("The Quit variant has no data to destructure."),
        Message::Move { x, y } => println!(
            "Move in the x direction {} and in the y direction {}",
            x, y
        ),
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change the color to RGB({}, {}, {})", r, g, b)//Change the color to RGB(255, 160, 255)
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change the color to HSV({}, {}, {})", h, s, v)
        }
    }

    // Nested destructuring of structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
    println!(
        "Nested destructuring: feet = {}, inches = {}, x = {}, y = {}",//Nested destructuring: feet = 3, inches = 10, x = 3, y = -10
        feet, inches, x, y
    );

    // Array destructuring
    let arr: [u16; 2] = [114, 514];
    let [x, y] = arr;
    println!("Array destructuring: x = {}, y = {}", x, y);//Array destructuring: x = 114, y = 514

    let arr: &[u16] = &[114, 514];

    if let [x, ..] = arr {
        println!("First element in the slice: {:?}", x);//First element in the slice: 114
    }

    if let &[.., y] = arr {
        println!("Last element in the slice: {:?}", y);//Last element in the slice: 514
    }

    let arr: &[u16] = &[];

    assert!(matches!(arr, [..]));
    assert!(!matches!(arr, [x, ..]));
}

pub fn test() {
    demonstrate_pattern_matching();
    demonstrate_pattern_matching2();
}


fn demonstrate_pattern_matching2() {
  #[derive(Debug)]
  struct Point {
      x: i32,
      y: i32,
      z: i32,
  }

  enum Message {
      Hello { id: i32 },
  }

    // 1. 忽略函数参数中的某些值
    fn foo(_: i32, y: i32) {
        println!("这个代码只使用了 y 参数: {}", y);// y 参数: 4
    }
    foo(3, 4);

    // 2. 在 match 表达式中忽略某些值
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("无法覆盖已有的自定义值");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("当前设置值: {:?}", setting_value);//当前设置值: Some(5)

    // 3. 忽略元组中的特定元素
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("选取的数字: {}, {}, {}", first, third, fifth);//选取的数字: 2, 8, 32
        }
    }

    // 4. `_` 和 `_var` 的区别 (`_var` 仍然会绑定值但不使用)
    let _x = 5; // `_x` 绑定了值但未使用
    let y = 10; // `y` 正常使用
    println!("y = {}", y);

    // 5. 使用 `..` 忽略结构体的剩余字段
    let origin = Point { x: 0, y: 0, z: 0 };
    match origin {
        Point { x, .. } => println!("x 的值是 {}", x),//0
    }

    // 6. 使用 `..` 忽略元组中间的元素
    match numbers {
        (first, .., last) => {
            println!("选取的数字: {}, {}", first, last);//选取的数字: 2, 32
        }
    }

    // 7. 使用 match guard (添加条件进行匹配)
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("小于 5: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    // 8. 使用 `|` 组合多个模式，并添加 match guard
    let x = 4;
    let y = false;
    match x {
        4 | 5 | 6 if y => println!("是"),
        _ => println!("否"), //✅
    }

    // 9. 使用 `@` 绑定变量，同时匹配范围
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: id_variable @ 3..=7 } => {
            println!("找到一个在范围内的 id: {}", id_variable);//找到一个在范围内的 id: 5
        }
        Message::Hello { id: 10..=12 } => {
            println!("找到另一个范围内的 id");
        }
        Message::Hello { id } => {
            println!("找到其他 id: {}", id);
        }
    }

    // 10. 绑定并解构结构体
    let p @ Point { x: px, y: py, z: _ } = Point { x: 10, y: 23, z: 0 };
    println!("x: {}, y: {}", px, py);//x: 10, y: 23
    println!("{:?}", p);//Point { x: 10, y: 23, z: 0 }

    let point = Point { x: 10, y: 5, z: 0 };
    if let p @ Point { x: 10, y, .. } = point {
        println!("x 是 10, y 是 {}，完整结构体: {:?}", y, p);
        //x 是 10，y 是 5，完整结构体: Point { x: 10, y: 5, z: 0 }
    } else {
        println!("x 不是 10 :(");
    }

    // 11. 使用 `@` 绑定 match 结果 会报错，因为只匹配了 1， 2 没有
    // match 1 {
    //     num @ 1 | 2 => {
    //         println!("{}", num);
    //     }
    //     _ => {}
    // }
}

// 📘 TypeScript 对比
// ====================
// Rust 的模式匹配覆盖了 TS 没有的很多能力：
// 
// | 模式类型 | Rust | TypeScript |
// |---------|------|-----------|
// | 字面量匹配 | `match x { 1 => ... }` | `switch(x) { case 1: ... }` |
// | 变量绑定 | `match x { Some(n) => ... }` | 解构 |
// | `@` 绑定 | `n @ 1..=5 =>` 匹配+绑定 | ❌ |
// | 或模式 | `1 | 2 =>` | 多个 case 合并 |
// | 范围模式 | `1..=10 =>` | ❌ |
// | 守卫 | `n if n > 0 =>` | `if` 守卫 |
// | 通配 | `_ =>` | `default:` |
// | 引用模式 | `ref x =>` / `ref mut x =>` | ❌ |
// 
// 详细对照 → rust_vs_typescript.rs §5

