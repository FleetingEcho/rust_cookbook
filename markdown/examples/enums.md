# Rust vs TypeScript: 枚举

**运行命令：** `cargo run -p learning_notes --example rts_enums`

## TypeScript 版本

```ts
enum Direction { North = "NORTH", South = "SOUTH", East = "EAST", West = "WEST" }

type Shape =
    | { kind: "circle";    radius: number }
    | { kind: "rectangle"; width: number; height: number }
    | { kind: "triangle";  base: number;  height: number };

function area(shape: Shape): number {
    switch (shape.kind) {
        case "circle":    return Math.PI * shape.radius ** 2;
        case "rectangle": return shape.width * shape.height;
        case "triangle":  return 0.5 * shape.base * shape.height;
    }
}

function findUser(id: number): User | null { ... }
type Result<T, E> = { ok: true; value: T } | { ok: false; error: E };
```

## 一、简单枚举

```rust
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

let dir = Direction::North;
let text = match dir {
    Direction::North => "向北走",
    Direction::South => "向南走",
    Direction::East  => "向东走",
    Direction::West  => "向西走",
};
println!("{text}");
println!("是北方吗: {}", Direction::North == Direction::North);
```

## 二、带数据的枚举

TS 对应：Discriminated Union（判别联合）。Rust 枚举最强大的特性之一。

```rust
#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(r)               => std::f64::consts::PI * r * r,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle(base, h)       => 0.5 * base * h,
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle(r)                   => format!("圆形，半径 {r:.1}"),
            Shape::Rectangle { width, height } => format!("矩形 {width:.1}×{height:.1}"),
            Shape::Triangle(base, h)           => format!("三角形，底 {base:.1}，高 {h:.1}"),
        }
    }

    fn is_circle(&self) -> bool {
        matches!(self, Shape::Circle(_))
    }
}

let shapes = vec![
    Shape::Circle(5.0),
    Shape::Rectangle { width: 4.0, height: 6.0 },
    Shape::Triangle(3.0, 8.0),
];

for shape in &shapes {
    println!("{} → 面积: {:.2}", shape.describe(), shape.area());
}
```

## 三、复杂枚举（Message 模式）

```rust
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

impl Message {
    fn process(&self) {
        match self {
            Message::Quit                   => println!("退出"),
            Message::Move { x, y }         => println!("移动到 ({x}, {y})"),
            Message::Write(text)            => println!("写入: {text}"),
            Message::ChangeColor(r, g, b)   => println!("颜色: rgb({r},{g},{b})"),
        }
    }
}
```

## 四、Option<T>

TS 对应：`T | null | undefined`。

```rust
fn find_user(id: u32) -> Option<&'static str> {
    match id {
        1 => Some("Alice"),
        2 => Some("Bob"),
        _ => None,
    }
}

match find_user(1) {
    Some(name) => println!("找到: {name}"),
    None       => println!("未找到"),
}

let name = find_user(99).unwrap_or("游客");
let upper = find_user(1).map(|n| n.to_uppercase());
```

## 五、Result<T, E>

TS 对应：`try/catch` 或 `T | Error`。

```rust
fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("'{s}' 不是有效整数"))
}

match parse_number("42") {
    Ok(n)    => println!("解析成功: {n}"),
    Err(msg) => println!("解析失败: {msg}"),
}

fn double_parse(s: &str) -> Result<i32, String> {
    let n = parse_number(s)?;
    Ok(n * 2)
}
```

## 六、枚举实现方法

```rust
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
}

impl Coin {
    fn value(&self) -> u32 {
        match self {
            Coin::Penny      => 1,
            Coin::Nickel     => 5,
            Coin::Dime       => 10,
            Coin::Quarter(_) => 25,
        }
    }
}
```

## 总结对照表

| TypeScript | Rust |
|------------|------|
| `enum Direction { N, S }` | `enum Direction { North, South }` |
| Discriminated Union + switch | `match` + 编译器强制穷举检查 |
| `T \| null` | `Option<T>`（Some/None） |
| `try/catch` | `Result<T, E>`（Ok/Err） |
| `switch(shape.kind)` | `match` + 解构（无冗余 kind） |
| 不能在枚举上实现方法 | `impl` 块给枚举添加方法 |
| `Object.values(Direction)` | 无内置（可派生 strum） |
| `case` 穿透 | `match` 不会穿透，无需 `break` |
| `default` 可选 | `_` 通配符必须（保证穷举） |

详细对照 → `rust_vs_typescript.rs §6 "枚举"`
