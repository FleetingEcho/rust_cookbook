// 这个属性用于隐藏未使用代码的警告。
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 单元结构体
struct Unit;

// 元组结构体
struct Pair(i32, f32);

// 带有两个字段的结构体
struct Point {
    x: f32,
    y: f32,
}

// 结构体可以作为另一个结构体的字段
struct Rectangle {
    // 可以通过指定左上角和右下角的位置
    // 来定义一个矩形。
    top_left: Point,
    bottom_right: Point,
}

pub fn test() {
    // 使用字段初始化简写语法创建结构体
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 打印结构体的调试信息
    println!("{:?}", peter);

    // 实例化一个 `Point`
    let point: Point = Point { x: 5.2, y: 0.4 };
    let another_point: Point = Point { x: 10.3, y: 0.2 };

    // 访问点的字段
    println!("点的坐标：({}, {})", point.x, point.y);

    // 使用结构体更新语法创建新的点,
    // 复用之前创建的点的字段
    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    // `bottom_right.y` 与 `another_point.y` 相同,
    // 因为我们使用了 `another_point` 的该字段
    println!("第二个点：({}, {})", bottom_right.x, bottom_right.y);

    // 使用 `let` 绑定解构点
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // 结构体实例化也是一个表达式
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right: bottom_right,
    };

    // 实例化一个单元结构体
    let _unit = Unit;

    // 实例化一个元组结构体
    let pair = Pair(1, 0.1);

    // 访问元组结构体的字段
    println!("pair 包含 {:?} 和 {:?}", pair.0, pair.1);

    // 解构一个元组结构体
    let Pair(integer, decimal) = pair;

    println!("pair 包含 {:?} 和 {:?}", integer, decimal);

    enum_test();

    use_test();
}

// 创建一个 `enum` 来分类网页事件。注意名称和类型信息如何共同指定变体：
// `PageLoad != PageUnload` 且 `KeyPress(char) != Paste(String)`。
// 每个变体都是不同且独立的。
enum WebEvent {
    // `enum` 变体可以类似单元结构体（`unit-like`）,
    PageLoad,
    PageUnload,
    // 类似元组结构体,
    KeyPress(char),
    Paste(String),
    // 或类似 C 语言的结构体。
    Click { x: i64, y: i64 },
}

// 一个接受 `WebEvent` 枚举作为参数且不返回任何值的函数。
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("页面已加载"),
        WebEvent::PageUnload => println!("页面已卸载"),
        // 从 `enum` 变体内部解构 `c`。
        WebEvent::KeyPress(c) => println!("按下了'{}'键。", c),
        WebEvent::Paste(s) => println!("粘贴了\"{}\"。", s),
        // 将 `Click` 解构为 `x` 和 `y`。
        WebEvent::Click { x, y } => {
            println!("点击坐标：x={}, y={}。", x, y);
        }
    }
}

fn enum_test() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` 从字符串切片创建一个拥有所有权的 `String`。
    let pasted = WebEvent::Paste("我的文本".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}

enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

// 带隐式判别值的枚举（从 0 开始）
enum Number {
    Zero,
    One,
    Two,
}

// 带显式判别值的枚举
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn use_test() {
    // 显式 `use` 每个名称,使它们可以不需要
    // 手动作用域限定就能使用。
    use crate::rust_by_example::examples::structs::Stage::{Advanced, Beginner};
    // 自动 `use` `Role` 内的每个名称。
    use crate::rust_by_example::examples::structs::Role::*;

    // 等同于 `Stage::Beginner`。
    let stage = Beginner;
    // 等同于 `Role::Student`。
    let role = Student;

    match stage {
        // 注意由于上面的显式 `use`,这里不需要作用域限定。
        Beginner => println!("初学者正在开始他们的学习之旅！"),
        Advanced => println!("高级学习者正在掌握他们的科目..."),
    }

    match role {
        // 再次注意这里不需要作用域限定。
        Student => println!("学生正在获取知识！"),
        Teacher => println!("教师正在传播知识！"),
    }

    // C语言风格

    // `enum` 可以转换为整数。
    println!("zero 的值是 {}", Number::Zero as i32);
    println!("one 的值是 {}", Number::One as i32);

    println!("玫瑰的颜色是 #{:06x}", Color::Red as i32);
    println!("紫罗兰的颜色是 #{:06x}", Color::Blue as i32);
}
