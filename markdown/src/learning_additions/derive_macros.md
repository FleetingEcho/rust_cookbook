# #[derive(...)] 宏详解

## 简介

`#[derive(...)]` 让编译器自动生成 trait 实现。这里集中演示每个常用 derive 的作用和限制。

## Debug

生成 `{:?}` 和 `{:#?}` 的打印格式。几乎所有自定义类型都应该加。

```rust
#[derive(Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

pub fn show_debug() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("{p:?}");  // Point { x: 1.0, y: 2.5 }
    println!("{p:#?}"); // 带缩进的多行格式
}
```

## Clone 和 Copy

Clone 是显式深拷贝，调用 `.clone()`。

Copy 是隐式按位拷贝，赋值/传参时自动复制，不会发生移动。Copy 要求类型里的所有字段也都是 Copy（不能含 String、Vec 等）。

```rust
#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub fn show_copy() {
    let red = Color { r: 255, g: 0, b: 0 };
    let also_red = red; // Copy 类型：这里是复制，不是移动
    println!("{red:?} 和 {also_red:?} 都可以用");
}
```

String 含有堆数据，不能 Copy，只能 Clone。

```rust
#[derive(Debug, Clone)]
pub struct Name {
    pub first: String,
    pub last: String,
}

pub fn show_clone() {
    let n = Name { first: "Rust".into(), last: "Lang".into() };
    let n2 = n.clone(); // 显式深拷贝
    println!("{} {}", n.first, n2.first); // 两个都能用
}
```

## PartialEq 和 Eq

PartialEq 支持 `==` 和 `!=`。f32/f64 只实现 PartialEq（NaN != NaN）。

Eq 在 PartialEq 之上声明“等价关系是完全的”（所有值都能比较）。整数、字符串等实现了 Eq；f64 没有。

```rust
#[derive(Debug, PartialEq, Eq)]
pub struct UserId(u64);

pub fn show_partialeq() {
    let a = UserId(42);
    let b = UserId(42);
    let c = UserId(99);
    println!("a == b: {}", a == b); // true
    println!("a == c: {}", a == c); // false
}
```

## PartialOrd 和 Ord

PartialOrd 支持 `<`、`>`、`<=`、`>=`，可能返回 None（如 NaN 的比较）。

Ord 是全序，能用 `.sort()`、`.max()`、`.min()`。派生 Ord 要先派生 PartialOrd、Eq、PartialEq。派生时按字段声明顺序比较（先 major，再 minor，再 patch）。

```rust
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

pub fn show_ord() {
    let mut versions = vec![
        Version { major: 1, minor: 2, patch: 0 },
        Version { major: 0, minor: 9, patch: 5 },
        Version { major: 1, minor: 0, patch: 3 },
    ];
    versions.sort();
    for v in &versions {
        println!("{}.{}.{}", v.major, v.minor, v.patch);
    }
    // 输出：0.9.5 → 1.0.3 → 1.2.0
}
```

## Hash

让类型可以作为 `HashMap` / `HashSet` 的键。要求实现了 Hash 的类型必须同时实现 Eq。规则：`a == b` 必须推出 `hash(a) == hash(b)`，否则会有 bug。

```rust
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub fn show_hash() {
    let mut map: HashMap<Coord, &str> = HashMap::new();
    map.insert(Coord { x: 0, y: 0 }, "原点");
    map.insert(Coord { x: 1, y: 0 }, "右边");
    println!("{:?}", map[&Coord { x: 0, y: 0 }]); // "原点"
}
```

## Default

生成 `Type::default()`，每个字段使用其自身的 Default 值（数字→0，bool→false，String→""，Vec→[]，Option→None）。

```rust
#[derive(Debug, Default)]
pub struct Config {
    pub timeout_secs: u64,    // 默认 0
    pub retries: u32,          // 默认 0
    pub verbose: bool,         // 默认 false
    pub endpoint: String,      // 默认 ""
}

pub fn show_default() {
    let cfg = Config {
        timeout_secs: 30,
        ..Config::default() // 其余字段用默认值
    };
    println!("{cfg:?}");
}
```

## 不能自动 derive 的情况

如果某个字段的类型没有实现目标 trait，derive 会编译报错。例如：含 `*mut T` 的结构体无法 derive Clone（裸指针不是 Clone）。解决方法：手动实现 trait，或换用安全的包装类型。

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_does_not_move() {
        let c = Color { r: 1, g: 2, b: 3 };
        let _c2 = c;
        // c 仍然可用，因为 Color 是 Copy
        let _ = c.r;
    }

    #[test]
    fn ord_sorts_versions() {
        let v1 = Version { major: 1, minor: 0, patch: 0 };
        let v2 = Version { major: 2, minor: 0, patch: 0 };
        assert!(v1 < v2);
    }

    #[test]
    fn default_fills_zeros() {
        let cfg = Config::default();
        assert_eq!(cfg.timeout_secs, 0);
        assert_eq!(cfg.endpoint, "");
        assert!(!cfg.verbose);
    }
}
```
