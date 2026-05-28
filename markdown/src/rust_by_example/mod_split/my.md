# my 模块

同样，`mod inaccessible` 和 `mod nested` 会分别定位到 `inaccessible.rs` 和 `nested.rs` 文件。

```rust
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("调用了 `my::function()`");
}

fn private_function() {
    println!("调用了 `my::private_function()`");
}

pub fn indirect_access() {
    print!("调用了 `my::indirect_access()`，它\n> ");
    private_function();
}
```
