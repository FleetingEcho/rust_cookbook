# Package Module 模块

```rust
#[path = "crate.rs"]
pub mod crate_examples;
```

`package_module` 模块下包含一个子模块：

- `crate_examples` — Crate 与模块可见性的完整示例，使用 `#[path]` 属性指向 `crate.rs`。
