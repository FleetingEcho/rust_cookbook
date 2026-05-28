# Rust 泛型

## 1. 泛型函数

泛型允许在定义函数时使用占位类型，使用时再具体化：

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

let number_list = vec![34, 50, 25, 100, 65];
let result = largest(&number_list);
println!("The largest number is {}", result); // 100

let char_list = vec!['y', 'm', 'a', 'q'];
let result = largest(&char_list);
println!("The largest char is {}", result); // y
```

`T: PartialOrd + Copy` 的约束解释：

- `PartialOrd` → 使 `T` 可比较（支持 `>` 操作）
- `Copy` → 确保 `T` 是小数据类型（如 `i32`、`char`），直接复制，不转移所有权

## 2. where 子句

当泛型约束较多时使用 `where` 子句，提高可读性：

```rust
use std::fmt::Display;

fn create_and_print<T>()
where
    T: From<i32> + Display,
{
    let a: T = 100.into();
    println!("a is: {}", a);
}

create_and_print::<i64>(); // a is: 100
```

泛型 `T` 必须满足两个约束：

- 实现 `From<i32>` → 确保可以从 `i32` 转换为 `T`
- 实现 `Display` → 确保 `T` 可以被 `println!` 格式化输出

`100.into()` 会调用 `T::from(100)` 将 `100` 转换成 `T` 类型。例如 `create_and_print::<i64>()` 时，`100.into()` 变成 `100i64`，然后打印 `a is: 100`。

这个模式常用于构造泛型值，并确保它可以被转换和显示。

## 3. 结构体中的泛型

### 3.1 单泛型参数

```rust
struct Point<T> {
    x: T,
    y: T,
}

let integer = Point { x: 5, y: 10 };
let float = Point { x: 1.0, y: 4.0 };
```

### 3.2 多泛型参数（不同类型）

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

let p = Point { x: 1, y: 1.1 };
```

### 3.3 枚举中的泛型

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## 4. 方法中使用泛型

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

let p = Point { x: 5, y: 10 };
println!("p.x = {}", p.x());
```

这里的 `Point<T>` 不再是泛型声明，而是一个完整的结构体类型，因为定义的结构体就是 `Point<T>` 而不再是 `Point`。

### 4.1 多泛型方法

```rust
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

// 也可以针对具体类型实现
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### 4.2 使用示例

```rust
let p1 = Point { x: 5, y: 10.4 };
let p2 = Point { x: "Hello", y: 'c' };
let p3 = p1.mixup(p2);
println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
```

## 5. 数组与泛型

`[i32; 3]` 和 `[i32; 2]` 是两种完全不同的类型，因此无法用同一个函数调用。

### 5.1 使用切片

让 `display_array` 能打印任意长度的 `i32` 数组：

```rust
fn display_array(arr: &[i32]) {
    println!("{:?}", arr);
}

let arr: [i32; 3] = [1, 2, 3];
display_array(&arr);
let arr: [i32; 2] = [1, 2];
display_array(&arr);
```

### 5.2 泛型数组

将所有类型的数组都支持：

```rust
fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr);
}
```

### 5.3 const 泛型

```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}

let arr: [i32; 3] = [1, 2, 3];
display_array(arr);
let arr: [i32; 2] = [1, 2];
display_array(arr);
```

`N` 就是 const 泛型，定义的语法是 `const N: usize`，表示 const 泛型 `N`，它基于的值类型是 `usize`。

## 6. const fn

const fn 即常量函数，允许在编译期对函数进行求值，实现更高效、更灵活的代码设计：

```rust
const fn add(a: usize, b: usize) -> usize {
    a + b
}

const RESULT: usize = add(5, 10);

fn main() {
    println!("The result is: {}", RESULT);
}
```

---

## TypeScript 对比

Rust 泛型约等于 TS 泛型，但约束方式不同。

**TypeScript：**

```ts
function largest<T extends { compareTo(a: T): number }>(list: T[]): T { ... }
```

**Rust：**

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { ... }
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 泛型约束 | `T: PartialOrd + Copy`（trait bound） | `T extends Comparable` |
| 组合约束 | `+` 连接 | `&` 连接 |
| where 子句 | `where T: Display` | 无等价语法 |
| 编译期 | 单态化——每种类型生成独立代码 | 不适用（JS 运行时无类型） |
| const 泛型 | `const N: usize` 编译期值参数 | 不支持 |

Rust 的 trait bound 写在泛型参数旁或 `where` 子句中，TS 用 `extends` 关键字。功能类似，但 Rust 更严格。

详细对照 → [rust_vs_typescript.rs §9](../rust_vs_typescript.rs) "泛型"
