# Rust 枚举与类型转换

## 概述

Rust 的类型转换提供了多种机制，从安全的 `as` 到更可靠的 `TryInto`，再到不安全的 `transmute`。Rust 设计类型转换时优先保证安全性。

## 1. as 转换

`as` 关键字用于基本类型之间的强制转换：

```rust
fn main() {
    let decimal = 97.123_f32;

    // ❌ 直接转换失败：
    // let integer = decimal as u8; // 报错

    // ✅ 需要先转为整数类型
    let integer = decimal as i32;
    let c = integer as u8;
    println!("{}", c); // 输出: 97
}
```

> ⚠️ 注意：`as` 转换可能发生数据丢失（如精度截断、溢出），且不会报错。

## 2. TryInto — 安全的数值转换

`TryInto` 是 `TryFrom` 的自动衍生 trait，提供安全的数值转换，能够捕获溢出错误：

```rust
fn main() {
    let num: u32 = 256;
    let result = num.try_into::<u8>();
    println!("{:?}", result); // Err(256)

    let num: u32 = 255;
    let result = num.try_into::<u8>();
    println!("{:?}", result); // Ok(255)
}
```

> ✅ `TryInto` 是**推荐方式**，可以安全捕获溢出错误。

## 3. reinterpret — 结构体转换

使用 `bytemuck` 等 crate 实现结构体之间的安全转换：

```rust
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct MyType {
    a: u32,
    b: u32,
}

fn main() {
    let m = MyType { a: 1, b: 2 };
    let bytes = bytemuck::cast::<MyType, [u32; 2]>(m);
    println!("{:?}", bytes); // [1, 2]
}
```

## 4. 隐式类型转换

Rust 不支持隐式类型转换，但在方法调用等场景下，编译器会自动处理一些转换：

```rust
fn main() {
    let x: u32 = 5;
    let y = x.checked_add(10); // Option<u32>
}
```

## 5. transmute — 危险转换

`transmute` 允许在任意两个大小相同的类型之间转换，是最不安全的类型转换方式。

```rust
use std::mem;

fn main() {
    let x: i32 = 42;
    let y: f32 = unsafe { mem::transmute::<i32, f32>(x) };
    println!("{}", y); // 输出的浮点数是未定义的
}
```

> ⚠️ **极度危险**，可能导致未定义行为。

### 常见应用

**裸指针转函数指针：**

```rust
fn foo() -> i32 { 0 }

fn main() {
    let pointer = foo as *const ();
    let function: fn() -> i32 = unsafe { std::mem::transmute(pointer) };
    assert_eq!(function(), 0);
}
```

**延长或缩短生命周期：**

```rust
struct R<'a>(&'a i32);

// 延长生命周期
unsafe fn extend_lifetime<'b>(r: R<'b>) -> R<'static> {
    std::mem::transmute::<R<'b>, R<'static>>(r)
}

// 缩短生命周期
unsafe fn shorten_lifetime<'b, 'c>(r: &'b mut R<'static>) -> &'b mut R<'c> {
    std::mem::transmute::<&'b mut R<'static>, &'b mut R<'c>>(r)
}
```

## 总结

| 方式 | 适用范围 | 安全性 |
|------|---------|--------|
| `as` | 基本类型转换 | ❌ 溢出时不报错 |
| `TryInto` | 数值类型转换 | ✅ 捕获溢出错误 |
| `reinterpret` | 结构体转换 | ✅ 安全但啰嗦 |
| 隐式转换 | 方法调用等 | ✅ 自动处理 |
| `transmute` | 任意类型转换 | ⚠️ 极度危险 |

> Rust 设计类型转换时优先保证安全性，推荐尽量使用 `TryInto`，避免 `as` 和 `transmute` 造成的潜在问题。
