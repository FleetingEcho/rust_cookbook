# 生命周期 (Lifetime)

## 1. 生命周期简介

生命周期是 Rust 编译器的一个工具，用于确保引用始终有效。编译器通过**生命周期标注**追踪引用之间的关系，防止悬垂引用。

## 2. 函数中的引用

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

上述代码无法通过编译，因为编译器无法确定返回的引用生命周期与哪个参数绑定。

## 3. 生命周期标注

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

生命周期标注 `<'a>` 告诉编译器：返回的引用生命周期与参数 `x` 和 `y` 中较短的那个相同。

## 4. 结构体中的生命周期

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Are you ok?");

    {
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    } // i 在此处被丢弃
}
```

结构体中的引用需要生命周期标注，以确保引用不会在结构体之前失效。

## 5. 方法中的生命周期

```rust
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```

实现结构体的方法时，需要在 `impl` 上标注生命周期。

## 6. 静态生命周期

`'static` 表示引用在整个程序运行期间都有效：

```rust
let s: &'static str = "I have a static lifetime.";
```

字符串字面量具有 `'static` 生命周期，因为它们被存储在程序的二进制文件中。

## 7. 生命周期省略规则

编译器在某些情况下可以推断生命周期，无需显式标注：

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

- 如果只有一个输入引用，其生命周期自动赋给所有输出引用。
- 如果有多个输入引用但其中一个是 `&self`，`self` 的生命周期赋给所有输出引用。

---

## 📘 TypeScript 对比

Rust 生命周期在 TypeScript 中没有直接对应概念。

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 引用有效性 | 编译期生命周期检查 | 无检查，可能出现 null/undefined |
| 悬垂引用 | 编译期禁止 | 运行时可能出现 |
| 标注方式 | `<'a>` 生命周期参数 | 无 |
| 静态数据 | `'static` 标注 | 全局变量 |

> ⚠️ TypeScript 没有生命周期概念，引用的有效性完全在运行时处理。Rust 在编译期就排除了悬垂引用的可能。
