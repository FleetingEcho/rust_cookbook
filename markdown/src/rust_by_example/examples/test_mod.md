# 模块

## 模块可见性

```rust
mod my {
    pub struct Rectangle {
        width: u32,
        pub height: u32,
    }

    impl Rectangle {
        pub fn new(width: u32, height: u32) -> Self {
            Rectangle { width, height }
        }

        pub fn area(&self) -> u32 {
            self.width * self.height
        }
    }
}

fn main() {
    let rect = my::Rectangle::new(30, 50);
    println!("Rectangle area: {}", rect.area());
}
```
