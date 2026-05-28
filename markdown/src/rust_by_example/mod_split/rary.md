# 库 crate 示例

## 库配置

```rust
#![crate_type = "lib"]
#![crate_name = "rary"]
```

## 公开与私有函数

```rust
pub fn public_function() {
    println!("调用了 rary 的 `public_function()`");
}

fn private_function() {
    println!("调用了 rary 的 `private_function()`");
}

pub fn indirect_access() {
    print!("调用了 rary 的 `indirect_access()`，它\n> ");
    private_function();
}
```

## 编译命令

```bash
rustc --crate-type=lib ./src/mod_split/rary.rs
# 或使用 crate_type 属性时无需传递 --crate-type 标志
rustc ./src/mod_split/rary.rs
```
