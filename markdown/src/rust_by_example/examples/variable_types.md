# 变量类型

## 类型转换

```rust
fn main() {
    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;
    println!("{} -> {} -> {}", decimal, integer, character);

    println!("1000 as u16: {}", 1000 as u16);
    println!("1000 as u8: {}", 1000 as u8);
    println!("  -1 as u8: {}", (-1i8) as u8);
    println!(" 300.0 as u8: {}", 300.0_f32 as u8);
}
```

## 类型后缀

```rust
fn main() {
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;
    let i = 1;
    let f = 1.0;

    println!("x: {} bytes", std::mem::size_of_val(&x));
    println!("y: {} bytes", std::mem::size_of_val(&y));
}
```

## 类型别名

```rust
type NanoSecond = u64;
type Inch = u64;

fn main() {
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as u64;
    println!("{} 纳秒 + {} 英寸 = {} 单位？", nanoseconds, inches, nanoseconds + inches);
}
```
