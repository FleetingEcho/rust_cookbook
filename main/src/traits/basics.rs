//也就是接口 interface

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
    pub content: String
}

impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}

fn main() {
    let post = Post{title: "Rust语言简介".to_string(),author: "Sunface".to_string(), content: "Rust棒极了!".to_string()};
    let weibo = Weibo{username: "sunface".to_string(),content: "好像微博没Tweet好用".to_string()};

    println!("{}",post.summarize());
    println!("{}",weibo.summarize());
}

// ====================================================

// 函数重载
/*
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

//可以不用实现已经提供的默认函数
impl Summary for Weibo {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}
println!("1 new weibo: {}", weibo.summarize());
*/


// 使用特征作为函数参数 特征约束(trait bound)

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//pub fn notify(item1: &impl Summary, item2: &impl Summary) {}

//pub fn notify<T: Summary>(item1: &T, item2: &T) {}

//多重约束
//pub fn notify(item: &(impl Summary + Display)) {}
//pub fn notify<T: Summary + Display>(item: &T) {}


//where 约束
/*
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{}
*/


use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
  //只有 T 同时实现了 Display + PartialOrd 的 Pair<T> 才可以拥有此方法。
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}



fn returns_summarizable() -> impl Summary {//说明一个函数返回了一个类型，该类型实现了某个特征：
    Weibo {
        username: String::from("sunface"),
        content: String::from(
            "m1 max太厉害了，电脑再也不会卡",
        )
    }
}

/*
通过 derive 派生特征
在本书中，形如 #[derive(Debug)] 的代码已经出现了很多次，这种是一种特征派生语法，被 derive 标记的对象会自动实现对应的默认特征代码，继承相应的功能。

例如 Debug 特征，它有一套自动实现的默认代码，当你给一个结构体标记后，就可以使用 println!("{:?}", s) 的形式打印该结构体的对象。

*/

use std::convert::TryInto;

fn main() {
  let a: i32 = 10;
  let b: u16 = 100;
//b.try_into() 试图将 u16 转换为 i32，返回 Result<i32, _>。
// unwrap() 用于解包 Result，如果转换失败（如溢出），会 panic。但 b: u16 = 100 在 i32 范围内，所以这里不会 panic。
  let b_ = b.try_into()
            .unwrap();
//因为 Rust 强调 安全，不会自动进行可能导致数据丢失的类型转换，所以必须使用 try_into() 进行显式转换。
  if a < b_ {
    println!("Ten is less than one hundred.");
  }
}

//为自定义类型实现 + 操作
use std::ops::Add;

// 为Point结构体派生Debug特征，用于格式化输出
#[derive(Debug)]
struct Point<T: Add<T, Output = T>> { //限制类型T必须实现了Add特征，否则无法进行+操作。
    x: T,
    y: T,
}

impl<T: Add<T, Output = T>> Add for Point<T> {
    type Output = Point<T>;

    fn add(self, p: Point<T>) -> Point<T> {
        Point{
            x: self.x + p.x,
            y: self.y + p.y,
        }
    }
}

fn add<T: Add<T, Output=T>>(a:T, b:T) -> T {
    a + b
}

fn main() {
    let p1 = Point{x: 1.1f32, y: 1.1f32};
    let p2 = Point{x: 2.1f32, y: 2.1f32};
    println!("{:?}", add(p1, p2));

    let p3 = Point{x: 1i32, y: 1i32};
    let p4 = Point{x: 2i32, y: 2i32};
    println!("{:?}", add(p3, p4));
}

// 📘 TypeScript 对比
// ====================
// Rust `trait` ≈ TS `interface` + 抽象类部分功能
//
// ```rust
// trait Summary { fn summarize(&self) -> String; }
// impl Summary for Post { ... }
// ```
// ```ts
// interface Summary { summarize(): string; }
// class Post implements Summary { ... }
// ```
//
// | 特性 | Rust | TypeScript |
// |------|------|-----------|
// | 定义 | `trait` 关键字 | `interface` 关键字 |
// | 默认方法 | ✅ 有默认实现 | ❌ interface 不能，需抽象类 |
// | 区分 impl | `impl Trait for Type` | `class Type implements Interface` |
// | 关联类型 | ✅ `type Item;` | ❌ 无 |
// | 运算符重载 | ✅ 通过 `std::ops::Add` 等 | ❌ 不支持 |
// | 派生宏 | `#[derive(Debug)]` 自动实现 | 需手动写 |
//
// ⚠️ 核心差异：Rust 的 trait 可以**为外部类型实现**，
//    甚至可以给 `i32` 实现你自己定义的 trait。这是 TS interface 做不到的。
//
// 详细对照 → rust_vs_typescript.rs §10 "Trait"