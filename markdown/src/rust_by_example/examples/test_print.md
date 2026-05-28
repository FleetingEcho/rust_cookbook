# 打印输出

```rust
fn main() {
    println!("Hello");
    println!("Hello, {}!", "world");
    println!("The number is {}", 1);
    println!("{:?}", (3, 4));
    println!("{value}", value=4);
    println!("{} {}", 1, 2);
    println!("{:04}", 42);
}
```

| 宏 | 说明 |
|-----|------|
| `print!` | 格式化输出，不换行 |
| `println!` | 格式化输出，末尾添加换行 |
| `format!` | 格式化到 `String` |
| `eprint!` / `eprintln!` | 打印到 stderr |
