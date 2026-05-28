# Rust 方法

## 1. 方法基础

方法是在 `impl` 块中定义的函数，与结构体或枚举关联。Rust 中 `new` 是社区常见的命名约定，类似于其他语言的构造函数。

### 1.1 结构体方法

```rust
mod my {
    pub struct Rectangle {
        width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        pub fn width(&self) -> u32 {
            self.width
        }

        pub fn height(&self) -> u32 {
            self.height
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }

        pub fn can_hold(&self, other: &Rectangle) -> bool {
            self.width > other.width && self.height > other.height
        }
    }
}

fn main() {
    let rect1 = my::Rectangle::new(30, 50);
    let rect2 = my::Rectangle::new(10, 40);
    let rect3 = my::Rectangle::new(60, 45);

    println!("Rectangle 1 width: {}", rect1.width());
    println!("Rectangle 1 height: {}", rect1.height());
    println!("Rectangle 1 area: {}", rect1.area());
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
```

### 1.2 枚举方法

枚举类型同样可以使用 `impl` 定义方法：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // 这里可以添加不同的逻辑处理
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

## 2. self 的三种接收方式

Rust 方法通过 `self` 参数访问结构体实例，有三种形式，分别代表不同的所有权语义。

### 2.1 不可变借用 — `&self`

`&self` 代表不可变借用，不会修改结构体字段。这是最常用的方式，类似于 `this` 参数但不获取所有权。

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}
```

### 2.2 可变借用 — `&mut self`

`&mut self` 代表可变借用，允许修改结构体字段。

```rust
impl Rectangle {
    pub fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }
}
```

### 2.3 获取所有权 — `self`

`self` 代表获取所有权，意味着结构体实例将被消费。适用于对象转换，转换后原对象无法再被使用。

```rust
struct Square {
    side: u32,
}

impl Rectangle {
    pub fn into_square(self) -> Square {
        Square {
            side: self.width.min(self.height),
        }
    }
}
```

## 3. 完整示例

结合 `new`、`&self`、`&mut self` 和 `self` 三种形式的完整用法：

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    pub fn double_size(&mut self) {
        self.width *= 2;
        self.height *= 2;
    }

    pub fn into_square(self) -> Square {
        Square {
            side: self.width.min(self.height),
        }
    }
}

struct Square {
    side: u32,
}

fn main() {
    let mut rect = Rectangle::new(30, 50);
    println!("矩形面积: {}", rect.area());
    rect.double_size();
    println!("放大后的矩形: {} x {}", rect.width, rect.height);
    let square = rect.into_square();
    println!("转换后的正方形边长: {}", square.side);
}
```

---

## 📘 TypeScript 对比

**Rust：**

```rust
impl Rectangle {
    fn area(&self) -> u32 { self.width * self.height }
}
```

**TypeScript：**

```ts
class Rectangle {
    constructor(public width: number, public height: number) {}
    area(): number { return this.width * this.height; }
}
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 方法定义 | `impl Struct { fn foo(&self) }` | `class Struct { foo() }` |
| 不可变借用 | `&self` | `readonly` 字段 |
| 可变借用 | `&mut self` | 直接修改 `this` |
| 消费自身 | `self`（所有权转移） | 无法销毁实例本身 |
| 构造函数 | `fn new() -> Self` 约定 | `constructor()` |
