# 字符串练习

## 基础操作

```rust
pub fn string_practice() {
    // 创建字符串
    let s1 = String::from("Hello");
    let s2 = String::from("Rust");
    let s3 = format!("{} {}", s1, s2);
    println!("{}", s3);

    let s = String::from("Hello, Rust!");
    println!("长度: {}", s.len());

    // 遍历字符串
    for c in "Hello Rust".chars() {
        println!("{}", c);
    }

    // 字符访问
    let s = "Hello Rust";
    if let Some(ch) = s.chars().nth(2) {
        println!("{}", ch);
    }

    // 字符串切片（基于字节索引，需保证合法 UTF-8 边界）
    let mut s = String::from("hello world");
    {
        let slice = &s[0..5];
        println!("{}", slice);
    }
    s.push_str("!!!");
    println!("{}", s);

    // 直接修改
    let mut s = String::from("hello world");
    s = s.replace("world", "Rust");
    println!("{}", s);
}
```
