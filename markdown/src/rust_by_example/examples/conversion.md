# 类型转换

## 基本转换

```rust
fn main() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("{} -> {} -> {}", decimal, integer, character);
}
```

## 溢出行为

```rust
fn main() {
    println!("1000 as u16: {}", 1000 as u16);
    println!("1000 as u8: {}", 1000 as u8);
    println!("-1 as u8: {}", (-1i8) as u8);
    println!("300.0 as u8: {}", 300.0_f32 as u8);
}
```
