# Rust vs TypeScript：泛型

> **运行命令**：`cargo run -p learning_notes --example rts_generics`

---

## TypeScript 参考版本

```ts
// 泛型函数
function identity<T>(x: T): T { return x; }
function first<T>(arr: T[]): T | undefined { return arr[0]; }

// 泛型接口
interface Pair<T, U> { first: T; second: U; }

// 泛型类
class Stack<T> {
    private items: T[] = [];
    push(item: T): void { this.items.push(item); }
    pop(): T | undefined { return this.items.pop(); }
}

// 泛型约束（extends）
function getLength<T extends { length: number }>(arg: T): number {
    return arg.length;
}

// 多约束
function process<T extends Serializable & Printable>(item: T): void { ... }

// 条件类型（TS 特有，Rust 没有）
type IsString<T> = T extends string ? "yes" : "no";

// 关联类型（TS 用 type 成员）
interface Iterator<T> { next(): { value: T; done: boolean; }; }
```

---

## 一、泛型函数

**TS**: `function identity<T>(x: T): T`

```rust
use std::fmt::Display;

fn identity<T>(x: T) -> T {
    x
}

fn first<T>(list: &[T]) -> Option<&T> {
    // TS: function first<T>(arr: T[]): T | undefined
    list.first()
}
```

---

## 二、trait 约束（TS: `T extends ...`）

```rust
// 单约束：T 必须实现 Display（可以被 println! 打印）
// TS: function print<T extends { toString(): string }>(x: T)
fn print_item<T: Display>(x: T) {
    println!("值: {x}");
}

// 多约束：T 必须同时实现 Display 和 PartialOrd
// TS: T extends Printable & Comparable
fn largest<T: PartialOrd + Display>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// where 子句：约束多时更清晰（TS 没有对应语法，直接 extends 叠加）
fn compare_and_print<T, U>(t: &T, u: &U)
where
    T: Display + PartialOrd,
    U: Display,
{
    println!("t={t}, u={u}");
}
```

---

## 三、泛型结构体

**TS**: `interface Pair<T, U> { first: T; second: U; }`

```rust
#[derive(Debug)]
struct Pair<T> {
    first: T,
    second: T,
}

// 为泛型结构体实现方法（无约束，所有 T 都有这些方法）
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

// 条件方法：只有当 T 实现了特定 trait 时才有这个方法
// TS: 无法直接表达，需要运行时类型检查
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("最大值: {}", self.first);
        } else {
            println!("最大值: {}", self.second);
        }
    }
}
```

---

## 四、泛型枚举（标准库的 Option 和 Result 就是这么实现的）

```rust
#[derive(Debug)]
enum MyOption<T> {
    Some(T),
    None,
}

#[derive(Debug)]
enum MyResult<T, E> {
    Ok(T),
    Err(E),
}
```

---

## 五、关联类型（Associated Types）

**TS**: `interface Iterator<T> { next(): IteratorResult<T>; }`

**Rust**: `Iterator` trait 使用关联类型 `type Item`。

```rust
// 自定义迭代器（关联类型比泛型参数更简洁）
struct Counter {
    count: u32,
    max:   u32,
}

impl Counter {
    fn new(max: u32) -> Self { Counter { count: 0, max } }
}

// Iterator trait 要求定义关联类型 Item
impl Iterator for Counter {
    type Item = u32;   // 关联类型：指定迭代产生的值类型
                       // TS: interface Iterator<T> 中的 T 是参数，不是关联类型

    fn next(&mut self) -> Option<u32> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

// 关联类型 vs 泛型参数的区别：
// - 泛型参数：trait Converter<T>，同一类型可以实现多个 Converter
// - 关联类型：trait Iterator { type Item }，每个类型只能有一个 Iterator 实现
```

---

## 六、const 泛型（泛型参数可以是常量值）

**TS** 没有对应（TS 有 template literal types 但不同）。

```rust
// 数组长度作为泛型参数
fn print_array<T: Display, const N: usize>(arr: [T; N]) {
    print!("[");
    for (i, item) in arr.iter().enumerate() {
        if i > 0 { print!(", "); }
        print!("{item}");
    }
    println!("] (长度: {N})");
}

// 固定大小的矩阵
#[derive(Debug)]
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}
```

---

## 七、泛型 trait 对象（动态分发 vs 静态分发）

**TS**: 接口类型参数就是动态分发。

```rust
trait Animal {
    fn name(&self) -> &str;
    fn sound(&self) -> &str;
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn name(&self) -> &str { "狗" }
    fn sound(&self) -> &str { "汪汪" }
}

impl Animal for Cat {
    fn name(&self) -> &str { "猫" }
    fn sound(&self) -> &str { "喵喵" }
}

// 静态分发（编译期单态化）：每种类型生成独立代码，零开销
// TS: function makeSound<T extends Animal>(animal: T)
fn make_sound_static<T: Animal>(animal: &T) {
    println!("{} 说: {}", animal.name(), animal.sound());
}

// 动态分发（运行时查表）：类型可以异构，有轻微性能开销
// TS: function makeSound(animal: Animal) — TS 默认就是这种
fn make_sound_dynamic(animal: &dyn Animal) {
    println!("{} 说: {}", animal.name(), animal.sound());
}

fn main() {
    // 泛型函数
    println!("{}", identity(42));
    println!("{}", identity("hello"));
    println!("{:?}", first(&[1, 2, 3]));

    // trait 约束
    print_item(42);
    print_item("hello");
    print_item(3.14);

    let numbers = vec![34, 50, 25, 100, 65];
    println!("最大值: {}", largest(&numbers));

    // 泛型结构体
    let pair = Pair::new(5_i32, 10);
    pair.cmp_display();

    // 泛型枚举
    let some: MyOption<i32> = MyOption::Some(42);
    println!("{:?}", some);

    // 关联类型 Iterator
    let sum: u32 = Counter::new(5).sum();
    println!("Counter sum: {sum}");

    // const 泛型
    print_array([1, 2, 3]);
    print_array(['a', 'b']);

    // 泛型 trait 对象
    let dog = Dog;
    let cat = Cat;
    make_sound_static(&dog);
    make_sound_dynamic(&cat);

    // 动态分发异构集合
    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];
    for animal in &animals {
        make_sound_dynamic(animal.as_ref());
    }

    // TS 和 Rust 泛型的关键区别总结：
    println!("\n1. Rust 用 trait bound（:），TS 用 extends");
    println!("2. Rust 有关联类型，TS 用类型参数");
    println!("3. Rust 区分静态分发（泛型）vs 动态分发（dyn）");
    println!("4. Rust 有 const 泛型，TS 没有");
    println!("5. Rust 泛型单态化（零开销），TS 泛型在运行时擦除");
}
```
