// 运行命令：cargo run -p learning_notes --example rts_enums
//
// ============================================================
// TypeScript 版本（注释掉）：
// ============================================================
// // 简单枚举
// enum Direction { North = "NORTH", South = "SOUTH", East = "EAST", West = "WEST" }
//
// // 判别联合（Discriminated Union）—— TS 里最接近 Rust enum 的东西
// type Shape =
//     | { kind: "circle";    radius: number }
//     | { kind: "rectangle"; width: number; height: number }
//     | { kind: "triangle";  base: number;  height: number };
//
// function area(shape: Shape): number {
//     switch (shape.kind) {
//         case "circle":    return Math.PI * shape.radius ** 2;
//         case "rectangle": return shape.width * shape.height;
//         case "triangle":  return 0.5 * shape.base * shape.height;
//     }
// }
//
// // 可选值（TS 用 T | null）
// function findUser(id: number): User | null { ... }
// const user = findUser(1);
// if (user !== null) { console.log(user.name); }
//
// // 错误处理（TS 用 union type 或 throw）
// type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
// ============================================================

// ============================================================
// 一、简单枚举（单元变体）
// TS 对应：enum Direction { North, South, East, West }
// ============================================================
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

// ============================================================
// 二、带数据的枚举（Rust 最强大的特性之一）
// TS 对应：Discriminated Union（判别联合）
// ============================================================
#[derive(Debug)]
enum Shape {
    Circle(f64),                           // 元组变体：一个 f64（半径）
    Rectangle { width: f64, height: f64 }, // 结构体变体：命名字段
    Triangle(f64, f64),                    // 元组变体：底和高
}

impl Shape {
    fn area(&self) -> f64 {
        // match 相当于 TS 的 switch(shape.kind)，但更强大、且编译器强制穷举
        match self {
            Shape::Circle(r) => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(base, h) => 0.5 * base * h,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle(r) => format!("圆形，半径 {r:.1}"),
            Shape::Rectangle { width, height } => format!("矩形 {width:.1}×{height:.1}"),
            Shape::Triangle(base, h) => format!("三角形，底 {base:.1}，高 {h:.1}"),
        }
    }

    fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle(_)) // 简洁写法，对应 TS: shape.kind === "circle"
    }
}

// ============================================================
// 三、复杂枚举（类似 TS 联合类型）
// TS: type Message = { type: "quit" } | { type: "move"; x: number; y: number } | ...
// ============================================================
#[derive(Debug)]
enum Message {
    Quit,                    // 无数据，对应 TS: { type: "quit" }
    Move { x: i32, y: i32 }, // 命名字段，对应 TS: { type: "move"; x: number; y: number }
    Write(String),           // 单值，对应 TS: { type: "write"; text: string }
    ChangeColor(u8, u8, u8), // 多值，对应 TS: { type: "color"; r: number; g: number; b: number }
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit => println!("退出"),
            Message::Move { x, y } => println!("移动到 ({x}, {y})"),
            Message::Write(text) => println!("写入: {text}"),
            Message::ChangeColor(r, g, b) => println!("颜色: rgb({r},{g},{b})"),
        }
    }
}

fn main() {
    // ============================================================
    // 一、简单枚举 + match
    // ============================================================
    let dir = Direction::North;
    let text = match dir {
        Direction::North => "向北走",
        Direction::South => "向南走",
        Direction::East => "向东走",
        Direction::West => "向西走",
    };
    println!("{text}");

    // PartialEq：可以用 == 比较
    println!("是北方吗: {}", Direction::North == Direction::North);

    // ============================================================
    // 二、带数据的枚举
    // ============================================================
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle {
            width: 4.0,
            height: 6.0,
        },
        Shape::Triangle(3.0, 8.0),
    ];

    for shape in &shapes {
        println!("{} → 面积: {:.2}", shape.describe(), shape.area());
    }

    // ============================================================
    // 三、模式匹配的各种写法
    // ============================================================
    let s = Shape::Circle(3.0);

    // if let：只关心一个变体（TS: if (shape.kind === "circle")）
    if let Shape::Circle(r) = &s {
        println!("是圆！半径: {r}");
    }

    // matches! 宏：快速判断变体
    println!("是圆吗: {}", s.is_circle());

    // 通配符 _ 匹配剩余所有情况（TS: default）
    match &s {
        Shape::Circle(r) if *r > 4.0 => println!("大圆，半径 {r}"),
        Shape::Circle(_) => println!("小圆"),
        _ => println!("不是圆"), // TS: default
    }

    // ============================================================
    // 四、while let（持续弹出直到空）
    // ============================================================
    let mut stack = vec![1_i32, 2, 3];
    while let Some(top) = stack.pop() {
        // TS: while (stack.length > 0)
        println!("弹出: {top}");
    }

    // ============================================================
    // 五、复杂 Message 枚举
    // ============================================================
    let messages = vec![
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("hello")),
        Message::ChangeColor(255, 128, 0),
        Message::Quit,
    ];

    for msg in &messages {
        msg.process();
    }

    // ============================================================
    // 六、枚举中的 Option<T>（内置在标准库）
    // TS 对应：T | null | undefined
    // ============================================================
    fn find_user(id: u32) -> Option<&'static str> {
        match id {
            1 => Some("Alice"),
            2 => Some("Bob"),
            _ => None, // TS: return null
        }
    }

    // match 处理 Option
    match find_user(1) {
        Some(name) => println!("找到: {name}"),
        None => println!("未找到"),
    }

    // unwrap_or 提供默认值
    // TS: findUser(99) ?? "游客"
    let name = find_user(99).unwrap_or("游客");
    println!("名字: {name}");

    // map：变换 Some 内的值，None 直接穿透
    // TS: user?.toUpperCase()
    let upper = find_user(1).map(|n| n.to_uppercase());
    println!("大写: {:?}", upper);

    // ============================================================
    // 七、Result<T, E>（另一个重要内置枚举）
    // TS 对应：try/catch 或 T | Error
    // ============================================================
    fn parse_number(s: &str) -> Result<i32, String> {
        s.parse::<i32>().map_err(|_| format!("'{s}' 不是有效整数"))
    }

    match parse_number("42") {
        Ok(n) => println!("解析成功: {n}"),
        Err(msg) => println!("解析失败: {msg}"),
    }

    match parse_number("abc") {
        Ok(n) => println!("解析成功: {n}"),
        Err(msg) => println!("解析失败: {msg}"),
    }

    // ? 运算符：自动向上传播错误（TS 里只能 try/catch）
    fn double_parse(s: &str) -> Result<i32, String> {
        let n = parse_number(s)?; // 如果 Err，立即 return Err(...)
        Ok(n * 2)
    }
    println!("double_parse: {:?}", double_parse("21"));
    println!("double_parse: {:?}", double_parse("bad"));

    // ============================================================
    // 八、枚举实现方法（不只是数据容器）
    // ============================================================
    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(String), // 携带州名
    }

    impl Coin {
        fn value(&self) -> u32 {
            match self {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(_) => 25,
            }
        }
    }

    let coins = vec![
        Coin::Penny,
        Coin::Quarter(String::from("Alaska")),
        Coin::Dime,
        Coin::Nickel,
    ];

    let total: u32 = coins.iter().map(|c| c.value()).sum();
    println!("硬币总值: {total} 分");

    // ============================================================
    // 总结对照表
    // ============================================================
    println!("\n=== Rust vs TS 枚举总结 ===");
    println!("┌──────────────────────────────┬─────────────────────────────────────┐");
    println!("│ TypeScript                   │ Rust                                │");
    println!("├──────────────────────────────┼─────────────────────────────────────┤");
    println!("│ enum Direction {{ N, S }}     │ enum Direction {{ North, South }}     │");
    println!("│ Discriminated Union + switch │ match + 编译器强制穷举检查          │");
    println!("│ T | null（可选值的替代）      │ Option<T>（Some/None，类型安全）    │");
    println!("│ try/catch（错误的替代）       │ Result<T, E>（Ok/Err，类型安全）    │");
    println!("│ switch(shape.kind) 判断类型  │ match + 解构（无冗余 kind 字段）    │");
    println!("│ 不能在枚举上实现方法         │ impl 块给枚举添加方法               │");
    println!("│ Object.values(Direction)     │ 无内置（可派生 strum）              │");
    println!("│ case 穿透 (fall-through)     │ match 不会穿透，无需 break          │");
    println!("│ default 可选                  │ _ 通配符必须（保证穷举）            │");
    println!("└──────────────────────────────┴─────────────────────────────────────┘");
}
