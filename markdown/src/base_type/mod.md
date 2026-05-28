# base_type 模块

## 模块结构

```rust
pub mod basic;
pub mod expression;
pub mod iteration;
pub mod runner_notes;
pub mod string_bool_unit;
pub mod string_str_difference;
```

## 入口函数

```rust
pub fn base_type_main() {
    basic::exceed_num();
    expression::main_expression();
}
```
