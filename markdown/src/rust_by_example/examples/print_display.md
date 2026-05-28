# Display 格式化

## {} 占位符

```rust
fn test() {
    println!("Hello, {}!", "world");
    println!("The number is {}", 42);
    println!("{:04}", 42); // 0042
}
```

- `{}` — 需要实现 `Display` 特征
- `{:?}` — 需要实现 `Debug` 特征
