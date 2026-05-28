# 结构体访问

## self 参数

```rust
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, w: u32) {
        self.width = w;
    }

    fn consume(self) {
        println!("Consumed {}x{}", self.width, self.height);
    }
}
```

| 参数 | 含义 |
|------|------|
| `&self` | 不可变借用 |
| `&mut self` | 可变借用 |
| `self` | 获取所有权 |
