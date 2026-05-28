# 模块分割

## 文件级模块

这个声明会查找名为 `my.rs` 的文件，并将其内容插入到当前作用域下名为 `my` 的模块中。

```rust
mod my;

fn function() {
    println!("调用了 `function()`");
}

pub fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
```
