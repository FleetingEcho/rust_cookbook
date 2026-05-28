# Rust 注释与文档

## 1. 注释类型

Rust 支持三种主要注释方式。行注释以 `//` 开头，块注释使用 `/* ... */` 包裹，文档注释以 `///` 开头用于公共 API。

```rust
// 行注释

/* 块注释
   可以跨越多行 */

/// 文档注释（用于公开 API）
```

## 2. 文档注释

文档注释使用 `///` 附加在公共项上，可以生成文档页面。被注释的对象需要使用 `pub` 对外可见。文档注释是给用户看的，内部实现细节不应该被暴露出去。

```rust
/// `add_one` 将指定值加1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

## 3. 模块级文档注释

对于模块或包的文档，可以使用内联文档注释 `//!`。这种方式适用于 `lib.rs` 或模块文件顶部。

```rust
//! 计算一些你口算算不出来的复杂算术题

pub mod compute;
```

也可以使用 `/*! ... */` 块形式：

```rust
/*! lib包是world_hello二进制包的依赖包，
  里面包含了compute等有用模块 */
```

## 4. 文档测试

文档注释中的代码块可以被编译和运行，称为 Doc Test。使用 `cargo test` 时可以验证文档示例是否正确。

```rust
/// `add_one` 将指定值加1
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = world_hello::compute::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

### 4.1 测试 Panic 行为

使用 `# Panics` 章节标记函数可能在什么条件下发生 panic，配合 `should_panic` 属性编写测试：

```rust
/// `div` 执行除法运算
///
/// # Panics
///
/// 当第二个参数为零时，函数会 panic。
///
/// ```rust
/// // panics on division by zero
/// world_hello::compute::div(10, 0);
/// ```
pub fn div(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("Divide-by-zero error");
    }
    a / b
}
```

## 5. 文档中的代码跳转

Rust 文档注释支持使用 `[`名称`]` 语法创建超链接，可以跳转到标准库项、自定义项或其他库的项。

### 5.1 跳转到标准库

```rust
/// `add_one` 返回一个[`Option`]类型
pub fn add_one(x: i32) -> Option<i32> {
    Some(x + 1)
}
```

### 5.2 跳转到自定义项

```rust
use std::sync::mpsc::Receiver;

/// [`Receiver<T>`] 与 [`std::future`] 结合使用。
///
/// [`std::future::Future`] 与 [`Self::recv()`] 配合。
pub struct AsyncReceiver<T> {
    sender: Receiver<T>,
}

impl<T> AsyncReceiver<T> {
    pub async fn recv() -> T {
        unimplemented!()
    }
}
```

可以通过指定完整路径跳转到自己代码或其他库的指定项。

---

## 📘 TypeScript 对比

**Rust 文档注释：**

```rust
/// 将值加一
/// # Examples
/// ```
/// assert_eq!(add_one(2), 3);
/// ```
pub fn add_one(x: i32) -> i32 { x + 1 }
```

**TypeScript JSDoc：**

```ts
/**
 * 将值加一
 * @example
 * assert(addOne(2) === 3);
 */
function addOne(x: number): number { return x + 1; }
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 文档语法 | `///` / `//!` | `/** */` JSDoc |
| 文档生成工具 | `cargo doc` | TypeDoc |
| 内联代码链接 | `[Option]` | `{@link Option}` |
| 文档测试 | ✅ `cargo test` 直接运行 | ❌ 需额外工具 |
| Panic 标注 | `# Panics` 章节 | `@throws` |
