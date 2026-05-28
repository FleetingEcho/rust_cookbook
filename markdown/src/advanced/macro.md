# Rust 宏编程（Macro Programming）详解

Rust 提供了强大的**宏（Macro）**机制，它可以生成代码，减少重复，提高性能，并且可以在编译期进行代码转换。相比 C 语言中的宏，Rust 的宏更加安全和灵活。

本章主要介绍：

- Rust 宏的分类
- 宏与函数的区别
- 声明式宏 `macro_rules!`
- 过程宏（Procedural Macros）
- 宏的实际应用

## 1. Rust 宏的分类

Rust 中的宏主要分为两大类：

**声明式宏（Declarative Macros）：**

- 通过 `macro_rules!` 定义
- 语法类似 `match`
- 用于创建灵活的代码模式匹配

**示例：**

```rust
macro_rules! say_hello {
    () => {
        println!("Hello, Rust!");
    };
}

fn main() {
    say_hello!(); // 调用宏
}
```

**过程宏（Procedural Macros）：**

- 处理输入 `TokenStream` 并返回 `TokenStream`
- 三种类型：
  - `#[derive]` 宏（派生宏）
  - 类属性宏（Attribute-like Macro）
  - 类函数宏（Function-like Macro）

## 2. 宏 vs 函数

| 特性 | 函数 | 宏 |
|------|------|-----|
| 代码生成 | 不能生成代码 | 可以生成代码（编译期） |
| 参数数量 | 固定 | 可变（可接收任意参数） |
| 代码展开 | 运行时执行 | 编译期展开 |
| 类型检查 | 有 | 部分情况无 |
| 复杂性 | 易读易写 | 较难维护 |

**示例：**

```rust
// 函数（固定参数）
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// 宏（可变参数）
macro_rules! sum {
    ($($x:expr),*) => {
        0 $(+ $x)*
    };
}

fn main() {
    println!("{}", add(1, 2)); // 输出 3
    println!("{}", sum!(1, 2, 3, 4)); // 输出 10
}
```

- 函数：参数个数固定
- 宏：参数个数可变，可以传递任意数量的参数

## 3. 声明式宏 macro_rules!

`macro_rules!` 是 Rust 传统的宏机制，类似 `match` 进行模式匹配，用于**代码复用**和**自动生成代码**。

### 3.1 macro_rules! 语法

```rust
macro_rules! my_macro {
    ($val:expr) => {
        println!("Value: {}", $val);
    };
}

fn main() {
    my_macro!(42); // 输出: Value: 42
}
```

**解析：**

- `$val:expr`：匹配一个表达式
- 模式匹配：`$val` 被替换为传入的值
- `println!` 语句执行

### 3.2 定义 vec! 宏

标准库的 `vec!` 宏用于创建 `Vec`：

```rust
#[macro_export] // 允许其他模块使用该宏
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    };
}

fn main() {
    let v = vec![1, 2, 3];
    println!("{:?}", v); // 输出 [1, 2, 3]
}
```

**解析：**

- `$( $x:expr ),*`：匹配零个或多个表达式
- 展开 `$x`：将所有 `$x` 逐一 push 到 `Vec` 中
- 等价代码：

```rust
let mut temp_vec = Vec::new();
temp_vec.push(1);
temp_vec.push(2);
temp_vec.push(3);
```

- `#[macro_export]`：让宏可以被其他模块调用

## 4. 过程宏（Procedural Macros）

过程宏使用 `TokenStream` 作为输入和输出，通常用于复杂代码生成。

### 4.1 自定义 #[derive] 宏

目标：创建 `#[derive(HelloMacro)]` 让结构体自动实现 `hello_macro` 方法。

**1. 创建 hello_macro 依赖**

```bash
cargo new hello_macro
cd hello_macro
cargo new hello_macro_derive --lib
```

**2. 在 hello_macro 定义特征**

```rust
pub trait HelloMacro {
    fn hello_macro();
}
```

**3. 在 hello_macro_derive 定义过程宏**

`Cargo.toml`:

```toml
[lib]
proc-macro = true

[dependencies]
syn = "1.0"
quote = "1.0"
```

`src/lib.rs`:

```rust
extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::DeriveInput;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };

    gen.into()
}
```

**解析：**

- `syn::parse(input)`：解析 Rust 代码为 AST（抽象语法树）
- `quote!`：将 AST 转换回 Rust 代码
- `stringify!(#name)`：获取类型名（Sunfei → "Sunfei"）

**4. 在 main.rs 使用**

```rust
use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

#[derive(HelloMacro)]
struct Sunfei;

fn main() {
    Sunfei::hello_macro(); // 输出: Hello, Macro! My name is Sunfei!
}
```

## 5. 其他过程宏

### 5.1 类属性宏（Attribute-like Macro）

示例：

```rust
#[route(GET, "/")]
fn index() {}

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 处理属性
}
```

用法：自定义 `#[route(...)]` 属性，可用于函数、结构体等。

### 5.2 类函数宏（Function-like Macro）

```rust
let sql = sql!(SELECT * FROM users WHERE id=1);

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // 解析 SQL 语句
}
```

与 `macro_rules!` 相似，但能解析更复杂的输入。

## 6. 总结

| 宏类型 | 描述 | 示例 |
|--------|------|------|
| 声明宏 `macro_rules!` | 代码模式匹配 | `vec![]` |
| `#[derive]` 派生宏 | 自动生成 trait 实现 | `#[derive(Debug)]` |
| 类属性宏 | 自定义 `#[route(...)]` 等 | `#[route(GET, "/")]` |
| 类函数宏 | 定义 `sql!(...)` 这样的宏 | `sql!(SELECT * FROM users)` |

> Rust 宏极其强大，但需要谨慎使用，过多的宏会影响可读性和可维护性。

## 📘 TypeScript 对比

Rust 宏 ≈ TS 中**没有对应概念**。

| Rust 宏 | 用途 | TS 对应 |
|---------|------|---------|
| `macro_rules!` | 声明式代码生成 | ❌ 无 |
| `#[derive]` | 自动实现 trait | 部分装饰器 |
| 过程宏 | 编译期代码转换 | ❌ 无 |
| `vec!`, `println!` | 便捷语法 | 语言内置语法 |

- TS 用函数/泛型复用代码，Rust 用宏在**编译期生成代码**。
- 宏可以解析任意语法结构，函数只能处理值。
- 举例：`println!("x = {}", x)` 在编译期检查格式化参数的类型完全匹配——这是函数做不到的。

> ⚠️ Rust 宏是在编译期展开的，生成零运行时开销的代码。TS 的装饰器在运行时执行，有性能开销。

详细对照 → `rust_vs_typescript.rs §21 "宏"`
