# Rust 模式匹配完全指南

## 1. 基本模式匹配

### 1.1 字面量与或模式

使用 `|` 组合多个匹配分支：

```rust
let x = 1;
match x {
    1 | 2 => println!("One or two"),
    3 => println!("Three"),
    _ => println!("Anything"),
}
```

### 1.2 范围模式

支持整数、字符范围匹配，`..=` 表示闭区间：

```rust
let x = 5;
match x {
    1..=5 => println!("One through five"),
    _ => println!("Something else"),
}

let x = 'c';
match x {
    'a'..='j' => println!("Early ASCII letter"),
    'k'..='z' => println!("Late ASCII letter"),
    _ => println!("Something else"),
}
```

## 2. 结构体解构

### 2.1 结构体字段绑定

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
let Point { x: a, y: b } = p;
println!("Destructured Point: a = {}, b = {}", a, b);
// Destructured Point: a = 0, b = 7
```

### 2.2 在 match 中解构结构体

```rust
match p {
    Point { x, y: 0 } => println!("On the x-axis at {}", x),
    Point { x: 0, y } => println!("On the y-axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}
```

## 3. 枚举解构

```rust
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

let msg = Message::ChangeColor(Color::Rgb(255, 160, 255));

match msg {
    Message::Quit => println!("The Quit variant has no data to destructure."),
    Message::Move { x, y } => println!(
        "Move in the x direction {} and in the y direction {}",
        x, y
    ),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(Color::Rgb(r, g, b)) => {
        println!("Change the color to RGB({}, {}, {})", r, g, b)
        // Change the color to RGB(255, 160, 255)
    }
    Message::ChangeColor(Color::Hsv(h, s, v)) => {
        println!("Change the color to HSV({}, {}, {})", h, s, v)
    }
}
```

## 4. 嵌套解构

### 4.1 元组与结构体嵌套

```rust
let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
println!(
    "Nested destructuring: feet = {}, inches = {}, x = {}, y = {}",
    feet, inches, x, y
);
// Nested destructuring: feet = 3, inches = 10, x = 3, y = -10
```

### 4.2 数组与切片解构

```rust
let arr: [u16; 2] = [114, 514];
let [x, y] = arr;
println!("Array destructuring: x = {}, y = {}", x, y);
// Array destructuring: x = 114, y = 514

let arr: &[u16] = &[114, 514];

if let [x, ..] = arr {
    println!("First element in the slice: {:?}", x);
    // First element in the slice: 114
}

if let &[.., y] = arr {
    println!("Last element in the slice: {:?}", y);
    // Last element in the slice: 514
}

let arr: &[u16] = &[];

assert!(matches!(arr, [..]));
assert!(!matches!(arr, [x, ..]));
```

## 5. 忽略值与通配符

### 5.1 忽略函数参数

使用 `_` 忽略不需要的参数：

```rust
fn foo(_: i32, y: i32) {
    println!("这个代码只使用了 y 参数: {}", y);
    // y 参数: 4
}
foo(3, 4);
```

### 5.2 在 match 中忽略

```rust
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
println!("当前设置值: {:?}", setting_value);
// 当前设置值: Some(5)
```

### 5.3 忽略元组特定元素

```rust
let numbers = (2, 4, 8, 16, 32);
match numbers {
    (first, _, third, _, fifth) => {
        println!("选取的数字: {}, {}, {}", first, third, fifth);
        // 选取的数字: 2, 8, 32
    }
}
```

### 5.4 `_` 与 `_var` 的区别

`_var` 仍然会绑定值但不使用，而 `_` 完全不绑定：

```rust
let _x = 5; // `_x` 绑定了值但未使用
let y = 10; // `y` 正常使用
println!("y = {}", y);
```

## 6. 省略剩余字段

使用 `..` 忽略结构体或元组的剩余部分：

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };
match origin {
    Point { x, .. } => println!("x 的值是 {}", x), // 0
}

let numbers = (2, 4, 8, 16, 32);
match numbers {
    (first, .., last) => {
        println!("选取的数字: {}, {}", first, last);
        // 选取的数字: 2, 32
    }
}
```

## 7. 匹配守卫

添加条件进行模式匹配：

```rust
let num = Some(4);
match num {
    Some(x) if x < 5 => println!("小于 5: {}", x),
    Some(x) => println!("{}", x),
    None => (),
}
```

### 7.1 组合或模式与守卫

```rust
let x = 4;
let y = false;
match x {
    4 | 5 | 6 if y => println!("是"),
    _ => println!("否"),
}
```

## 8. `@` 操作符

使用 `@` 在匹配范围的同时绑定变量：

```rust
enum Message {
    Hello { id: i32 },
}

let msg = Message::Hello { id: 5 };
match msg {
    Message::Hello { id: id_variable @ 3..=7 } => {
        println!("找到一个在范围内的 id: {}", id_variable);
        // 找到一个在范围内的 id: 5
    }
    Message::Hello { id: 10..=12 } => {
        println!("找到另一个范围内的 id");
    }
    Message::Hello { id } => {
        println!("找到其他 id: {}", id);
    }
}
```

### 8.1 绑定并解构结构体

```rust
let p @ Point { x: px, y: py, z: _ } = Point { x: 10, y: 23, z: 0 };
println!("x: {}, y: {}", px, py); // x: 10, y: 23
println!("{:?}", p);              // Point { x: 10, y: 23, z: 0 }

let point = Point { x: 10, y: 5, z: 0 };
if let p @ Point { x: 10, y, .. } = point {
    println!("x 是 10, y 是 {}，完整结构体: {:?}", y, p);
    // x 是 10, y 是 5，完整结构体: Point { x: 10, y: 5, z: 0 }
} else {
    println!("x 不是 10 :(");
}
```

### 8.2 `@` 与 `|` 的冲突

`@` 绑定不能用于或模式的右侧，因为无法确定绑定的是哪个分支：

```rust
// 会报错，因为只匹配了 1，2 没有绑定
// match 1 {
//     num @ 1 | 2 => {
//         println!("{}", num);
//     }
//     _ => {}
// }
```

---

## TypeScript 对比

Rust 的模式匹配覆盖了 TS 没有的很多能力：

| 模式类型 | Rust | TypeScript |
|---------|------|-----------|
| 字面量匹配 | `match x { 1 => ... }` | `switch(x) { case 1: ... }` |
| 变量绑定 | `match x { Some(n) => ... }` | 解构 |
| `@` 绑定 | `n @ 1..=5 =>` 匹配+绑定 | 不支持 |
| 或模式 | `1 \| 2 =>` | 多个 case 合并 |
| 范围模式 | `1..=10 =>` | 不支持 |
| 守卫 | `n if n > 0 =>` | `if` 守卫 |
| 通配 | `_ =>` | `default:` |
| 引用模式 | `ref x =>` / `ref mut x =>` | 不支持 |

详细对照 → [rust_vs_typescript.rs §5](../rust_vs_typescript.rs)
