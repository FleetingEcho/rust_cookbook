# Rust Trait 基础

Trait 也就是接口（interface）。

## 1. 定义和实现 Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Post {
    pub title: String,
    pub author: String,
    pub content: String,
}

impl Summary for Post {
    fn summarize(&self) -> String {
        format!("文章{}, 作者是{}", self.title, self.author)
    }
}

pub struct Weibo {
    pub username: String,
    pub content: String,
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

let post = Post {
    title: "Rust语言简介".to_string(),
    author: "Sunface".to_string(),
    content: "Rust棒极了!".to_string(),
};
let weibo = Weibo {
    username: "sunface".to_string(),
    content: "好像微博没Tweet好用".to_string(),
};

println!("{}", post.summarize());
println!("{}", weibo.summarize());
```

## 2. 默认方法实现

Trait 可以提供默认方法实现，类似函数重载：

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// 只需实现 summarize_author，summarize 自动可用
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
println!("1 new weibo: {}", weibo.summarize());
```

## 3. 特征约束 (Trait Bound)

### 3.1 impl Trait 语法

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### 3.2 泛型语法

```rust
pub fn notify<T: Summary>(item1: &T, item2: &T) {}
```

### 3.3 多重约束

```rust
pub fn notify(item: &(impl Summary + Display)) {}
pub fn notify<T: Summary + Display>(item: &T) {}
```

### 3.4 where 子句

当约束过多时使用 `where` 提高可读性：

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// 等价写法
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}
```

## 4. 泛型结构体与 Trait Bound

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

## 5. 返回 impl Trait

说明一个函数返回了一个类型，该类型实现了某个特征：

```rust
fn returns_summarizable() -> impl Summary {
    Weibo {
        username: String::from("sunface"),
        content: String::from("m1 max太厉害了，电脑再也不会卡"),
    }
}
```

## 6. derive 派生特征

形如 `#[derive(Debug)]` 的代码是一种特征派生语法，被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。例如 `Debug` 特征，标记后可以使用 `println!("{:?}", s)` 打印该结构体的对象。

## 7. TryInto 特征

```rust
use std::convert::TryInto;

let a: i32 = 10;
let b: u16 = 100;

let b_ = b.try_into().unwrap();
// b.try_into() 试图将 u16 转换为 i32，返回 Result<i32, _>。
// unwrap() 用于解包 Result，如果转换失败（如溢出），会 panic。
// 但 b: u16 = 100 在 i32 范围内，所以这里不会 panic。

if a < b_ {
    println!("Ten is less than one hundred.");
}
```

Rust 强调安全，不会自动进行可能导致数据丢失的类型转换，所以必须使用 `try_into()` 进行显式转换。

## 8. 运算符重载

为自定义类型实现 `+` 操作：

```rust
use std::ops::Add;

#[derive(Debug)]
struct Point<T: Add<T, Output = T>> {
    // 限制类型 T 必须实现了 Add 特征，否则无法进行 + 操作
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point {
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output = T>>(a: T, b: T) -> T {
    a + b
}

let p1 = Point { x: 1.1f32, y: 1.1f32 };
let p2 = Point { x: 2.1f32, y: 2.1f32 };
println!("{:?}", add(p1, p2));

let p3 = Point { x: 1i32, y: 1i32 };
let p4 = Point { x: 2i32, y: 2i32 };
println!("{:?}", add(p3, p4));
```

---

## TypeScript 对比

Rust `trait` 约等于 TS `interface` + 抽象类部分功能。

**Rust：**

```rust
trait Summary { fn summarize(&self) -> String; }
impl Summary for Post { ... }
```

**TypeScript：**

```ts
interface Summary { summarize(): string; }
class Post implements Summary { ... }
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 定义 | `trait` 关键字 | `interface` 关键字 |
| 默认方法 | 有默认实现 | interface 不能，需抽象类 |
| 区分 impl | `impl Trait for Type` | `class Type implements Interface` |
| 关联类型 | `type Item;` | 无 |
| 运算符重载 | 通过 `std::ops::Add` 等 | 不支持 |
| 派生宏 | `#[derive(Debug)]` 自动实现 | 需手动写 |

核心差异：Rust 的 trait 可以**为外部类型实现**，甚至可以给 `i32` 实现你自己定义的 trait。这是 TS interface 做不到的。

详细对照 → [rust_vs_typescript.rs §10](../rust_vs_typescript.rs) "Trait"
