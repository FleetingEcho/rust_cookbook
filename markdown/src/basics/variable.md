# Rust 变量与常量

## 1. 可变与不可变变量

Rust 中 `let` 绑定的变量默认是**不可变**的。使用 `mut` 关键字可以声明可变变量，但变量的数据类型在声明后不可更改。

```rust
pub fn const_test(name: &str) {
    let mut x = name;
    println!("The value of x is: {}", x);
    x = "test";
    println!("The value of x is: {}", x);
}
```

## 2. 解构赋值

Rust 支持对元组、数组和结构体进行解构赋值。数组解构中使用 `..` 跳过中间元素，`_` 丢弃单个元素。

```rust
struct Struct {
    e: i32,
}

fn destructuring_assignment() {
    let (a, b) = (1, 2);
    let [c, .., d, _] = [1, 2, 3, 4, 5];
    let Struct { e, .. } = Struct { e: 5 };
    println!("a = {}, b = {}, c = {}, d = {}, e = {}", a, b, c, d, e);
    assert_eq!([1, 2, 1, 4, 5], [a, b, c, d, e]);
}
```

## 3. 常量

使用 `const` 关键字声明编译期常量，常量名通常使用大写字母和下划线。常量必须标注类型，且值在编译时必须确定。

```rust
const MAX_POINTS: u32 = 100_000;

fn my_const() {
    println!("const_value={}", MAX_POINTS);
}
```

## 4. 变量遮蔽

Rust 允许在同一作用域内使用 `let` 重新声明同名变量，新变量会遮蔽（shadow）旧变量。遮蔽后的两个变量同时存在，内层遮蔽不会影响外层。遮蔽的一个重要特性是可以改变变量的类型。

```rust
fn variable_shadowing() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
```

变量遮蔽的典型用法是在不同作用域中使用相同名称但不同含义的值。

```rust
pub fn const_test(name: &str) {
    let x = name;
    println!("The value of x is: {}", x);
    let x = 6;
    println!("The value of x is: {}", x);
}

fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }
    println!("The value of x is: {}", x);
}
```

## 5. 需要存储任意类型的替代方案

如果需要变量可以存储不同类型，可以使用 `std::any::Any`：

```rust
use std::any::Any;

fn main() {
    let mut x: Box<dyn Any> = Box::new(42);
    x = Box::new(3.14);
    x = Box::new("Hello Rust".to_string());
    if let Some(value) = x.downcast_ref::<String>() {
        println!("String value: {}", value);
    }
}
```

或者使用枚举：

```rust
enum AnyType {
    Int(i32),
    Float(f64),
    Str(String),
}

fn main() {
    let mut x = AnyType::Int(42);
    println!("{:?}", x);
    x = AnyType::Float(3.14);
    println!("{:?}", x);
    x = AnyType::Str("Hello Rust!".to_string());
    println!("{:?}", x);
}
```

---

## 📘 TypeScript 对比

Rust 的变量和 TypeScript 有本质区别。

**Rust：**

```rust
let x = 5;        // 不可变
let mut y = 5;    // 可变
```

**TypeScript：**

```ts
const x = 5;      // 不可变
let y = 5;        // 可变
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 默认可变性 | 不可变 `let` | 可变 `let` |
| 变量遮蔽 | ✅ 同一作用域可重复 `let` | ❌ 不能重复声明 |
| 常量 | `const MAX: u32 = 100` (编译期) | `const MAX = 100` (运行时) |
| 类型标注位置 | `let x: u32 = 5` | `let x: number = 5` |
| 解构赋值 | `let (a,b) = (1,2)` | `const [a,b] = [1,2]` |

> ⚠️ **Rust 的 `let` 默认不可变是关键设计。** 编译器会检查你没有意外修改变量，而 TS 的 `let` 默认可变。类型转换必须显式（`as`, `into`），TS 则有较多隐式转换。

详细对照 → [rust_vs_typescript.rs §1 "变量与常量"](../rust_vs_typescript.rs)
