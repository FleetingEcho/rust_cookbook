# 常量

## static 与 const

```rust
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    n > THRESHOLD
}

fn test() {
    let n = 16;
    println!("这是 {}", LANGUAGE);
    println!("阈值为 {}", THRESHOLD);
    println!("{} 是 {}", n, if is_big(n) { "大的" } else { "小的" });
}
```

> 不能修改 `const` 或 `static` 常量。
