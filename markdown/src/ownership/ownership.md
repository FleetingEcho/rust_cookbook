# 所有权 (Ownership)

## 1. 所有权规则

Rust 的所有权系统遵循三条核心规则：

- 每个值都有一个称为**所有者 (owner)** 的变量。
- 同一时刻只能有一个所有者。
- 当所有者超出作用域时，值将被丢弃。

## 2. 内存分配

### 栈上分配

固定大小的数据存储在栈上，例如 `i32`、`bool` 等。

### 堆上分配

大小未知或可变的数据存储在堆上，例如 `String`、`Vec<T>`。变量本身在栈上，但数据在堆上。

## 3. 移动语义

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // 所有权从 s1 移动到 s2

    println!("{}", s1); // ❌ 编译错误：s1 已失效
    println!("{}", s2); // ✅ 合法
}
```

当 `s1` 赋值给 `s2` 时，`s1` 的所有权被移动，`s1` 不再有效。

## 4. 克隆

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝

    println!("s1 = {}, s2 = {}", s1, s2); // ✅ 两者都有效
}
```

使用 `.clone()` 可以创建堆数据的完整副本，原值和新值都有效。

## 5. 浅拷贝 (Copy)

对于完全在栈上的类型，使用 `Copy` 语义：

```rust
fn main() {
    let x = 5;
    let y = x; // i32 实现 Copy，发生浅拷贝

    println!("x = {}, y = {}", x, y); // ✅ 两者都有效
}
```

`i32`、`bool`、`f64` 等基本类型实现 `Copy`，赋值时不会移动所有权。

## 6. 函数与所有权

```rust
fn takes_ownership(s: String) {
    println!("{}", s);
} // s 在此处被丢弃

fn main() {
    let s = String::from("hello");
    takes_ownership(s); // s 的所有权移动到函数

    println!("{}", s); // ❌ 编译错误：s 已移动
}
```

将值传递给函数会移动所有权。函数返回时，参数会被丢弃。

## 7. 返回值与所有权转移

```rust
fn gives_ownership() -> String {
    let s = String::from("hello");
    s // 返回 s，所有权转移给调用者
}

fn main() {
    let s = gives_ownership(); // s 获得所有权
    println!("{}", s); // ✅ 合法
}
```

函数返回值会将所有权转移给调用者。

---

## 📘 TypeScript 对比

Rust 所有权 ≈ TS 引用语义，但有本质区别。

**Rust：**

```rust
let s2 = s1; // 所有权移动，s1 失效
```

**TypeScript：**

```ts
let s2 = s1; // 两个变量都指向同一引用
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 赋值 | 移动所有权 | 复制引用 |
| 释放 | 所有者出作用域自动 drop | GC 垃圾回收 |
| 克隆 | 需显式 `.clone()` | 浅拷贝自动进行 |
| 基本类型 | Copy 语义（栈） | 值传递 |

> ⚠️ Rust 的所有权系统在编译期保证内存安全，而 TypeScript 依赖 GC。

详细对照 → [rust_vs_typescript.rs §3 "所有权与内存管理"](../rust_vs_typescript.rs)
