# Sized 与非 Sized 类型

## 什么是 Sized？

`Sized` trait 表示类型的大小在编译期是已知的。大多数 Rust 类型都实现了 `Sized`。

```rust
fn main() {
    let x: i32 = 5; // i32 是 Sized
    let s: String = "hello".to_string(); // String 是 Sized
}
```

## 非 Sized 类型（Dynamically Sized Types）

某些类型的大小在编译期无法确定：

- `str` — 字符串切片（长度可变）
- `[T]` — 切片（长度可变）
- `dyn Trait` — trait 对象（大小取决于具体实现）

```rust
// str 和 [T] 必须通过引用使用
let s: &str = "hello";
let arr: &[i32] = &[1, 2, 3];

// dyn Trait 必须通过指针使用
let obj: &dyn std::fmt::Debug = &42;
```

## Sized 约束

泛型默认要求 `T: Sized`：

```rust
fn generic<T>(t: T) {
    println!("{}", std::mem::size_of_val(&t));
}

// 允许非 Sized 类型
fn generic_dyn<T: ?Sized>(t: &T) {
    println!("{:?}", t);
}
```

## 总结

- `Sized` 类型在编译期大小已知。
- 非 Sized 类型必须通过引用或指针使用。
- `?Sized` 语法允许泛型接受非 Sized 类型。
