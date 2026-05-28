# 结构体

## 结构体类型

```rust
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

struct Point {
    x: f32,
    y: f32,
}

struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}
```

## 创建与使用

```rust
pub fn test() {
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };
    println!("{:?}", peter);

    let point = Point { x: 5.2, y: 0.4 };
    let another_point = Point { x: 10.3, y: 0.2 };

    let bottom_right = Point {
        x: 10.3,
        ..another_point
    };

    let Point { x: left_edge, y: top_edge } = point;

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    println!("pair 包含 {:?} 和 {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
}
```

## 枚举示例

```rust
enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("页面已加载"),
        WebEvent::KeyPress(c) => println!("按下了'{}'键。", c),
        WebEvent::Click { x, y } => println!("点击坐标：x={}, y={}。", x, y),
        _ => {}
    }
}
```

## 枚举作为整数

```rust
enum Number { Zero, One, Two }

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}
```
