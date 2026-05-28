# 基本类型

## 概述

- **数值类型**：有符号整数 (`i8`, `i16`, `i32`, `i64`, `isize`)、无符号整数 (`u8`, `u16`, `u32`, `u64`, `usize`)、浮点数 (`f32`, `f64`)、以及有理数、复数
- **字符串**：字符串字面量和字符串切片 `&str`
- **布尔类型**：`true` 和 `false`
- **字符类型**：表示单个 Unicode 字符，存储为 4 个字节
- **单元类型**：即 `()`，其唯一的值也是 `()`

## 数值类型

### 整数类型

| 长度 | 有符号类型 | 无符号类型 |
|------|-----------|-----------|
| 8位 | `i8` | `u8` |
| 16位 | `i16` | `u16` |
| 32位 | `i32` | `u32` |
| 64位 | `i64` | `u64` |
| 128位 | `i128` | `u128` |
| 依赖架构 | `isize` | `usize` |

`isize` 和 `usize` 的大小依赖于 CPU 架构：
- 32 位系统：`isize` 和 `usize` 为 32 位
- 64 位系统：`isize` 和 `usize` 为 64 位

### 整数范围

**有符号整数**采用补码（two's complement）表示：

- `i8`: -128 ~ 127
- `i16`: -32,768 ~ 32,767
- `i32`: -2,147,483,648 ~ 2,147,483,647
- `i64`: -9,223,372,036,854,775,808 ~ 9,223,372,036,854,775,807
- `i128`: -170,141,183,460,469,231,731,687,303,715,884,105,728 ~ 170,141,183,460,469,231,731,687,303,715,884,105,727

**无符号整数**范围：

- `u8`: 0 ~ 255
- `u16`: 0 ~ 65,535
- `u32`: 0 ~ 4,294,967,295
- `u64`: 0 ~ 18,446,744,073,709,551,615
- `u128`: 0 ~ 340,282,366,920,938,463,463,374,607,431,768,211,455

### 整数字面量

```rust
fn main1() {
    let decimal = 98_222; // 十进制
    let hex = 0xff; // 十六进制
    let octal = 0o77; // 八进制
    let binary = 0b1111_0000; // 二进制
    let byte = b'A'; // 字节字面量，等同于 `65u8`
}
```

### 浮点类型

| 类型 | 大小 | 精度 |
|------|------|------|
| `f32` | 32 位 | 单精度 |
| `f64` | 64 位 | 双精度（默认） |

```rust
fn main2() {
    let x = 2.0; // 默认 f64
    let y: f32 = 3.14; // 指定为 f32
}
```

### 算术运算

```rust
fn main3() {
    let sum = 5 + 10; // 加法
    let difference = 95.5 - 4.3; // 减法
    let product = 4 * 30; // 乘法
    let quotient = 56.7 / 32.2; // 除法
    let remainder = 43 % 5; // 取余
}
```

## 类型转换

```rust
fn main4() {
    let x: i8 = 5;
    let y: i16 = 10;

    // 需要显式转换
    let sum = x as i16 + y;
    println!("{}", sum);
}
```

### as 可能导致溢出

```rust
fn main5() {
    let n: u16 = 256;
    let m: u8 = n as u8; // 256 变成 0（溢出）
    println!("{}", m);
}
```

## 溢出检测

- **Debug 模式**：整数溢出时 Rust 会 `panic`。
- **Release 模式**：整数溢出时 Rust 采用补码环绕（wrapping behavior）。

```rust
fn main() {
    let mut x: u8 = 255;
    x = x.wrapping_add(1); // 变成 0（环绕行为）
    println!("{}", x);
}
```

**可用方法：**

- `wrapping_add()` — 发生溢出时环绕
- `checked_add()` — 发生溢出时返回 `None`
- `overflowing_add()` — 发生溢出时返回 `(值, 是否溢出)`
- `saturating_add()` — 发生溢出时返回 `u8::MAX` 或 `u8::MIN`

> 防止溢出时，用 `checked_add()` 或 `saturating_add()`！

```rust
pub fn exceed_num() {
    let a: u8 = 250;

    // checked_add(): 溢出返回 None
    if let Some(result) = a.checked_add(10) {
        println!("Checked: {}", result);
    } else {
        println!("Checked: 溢出！");
    }

    // saturating_add(): 溢出返回最大值
    let result = a.saturating_add(10);
    println!("Saturating: {}", result); // 输出 255
}
```

## usize 和 isize

`usize` 常用于索引数组或指针运算：

```rust
fn main() {
    let arr = [10, 20, 30];
    let index: usize = 1;
    println!("{}", arr[index]); // 20
}
```

`isize` 主要用于指针计算，能存储指针宽度范围内的整数。

## 📘 TypeScript 对比

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 整数类型 | `i8`~`i128`, `u8`~`u128`, `isize`, `usize` | 只有 `number`（IEEE 754） |
| 浮点 | `f32`, `f64` | 只有 `number`（≈f64） |
| 布尔 | `bool` | `boolean` |
| 字符 | `char`（4 字节 Unicode） | `string`（UTF-16 码元） |
| 单位类型 | `()` 空元组 | `void` / `undefined` |
| 类型转换 | 必须显式 `as` / `From` / `into` | 有隐式转换 |

> ⚠️ Rust 的数字类型比 TS 精确得多：
>
> - TS 的 `number` 是 IEEE 754 双精度浮点，整数只能精确到 2^53
> - Rust 可以精确控制位宽（u8, i32, u64 等）

详细对照 → `rust_vs_typescript.rs §2 "基本类型系统"`
