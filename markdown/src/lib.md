# lib.rs — Rust 学习笔记库入口

## 编译器配置

```rust
#![allow(
    dead_code,
    unused_assignments,
    unused_must_use,
    unused_mut,
    unused_variables
)]
```

## 模块结构

共享库入口，包含以下模块：

| 模块 | 说明 |
|------|------|
| `advanced` | 高级主题 |
| `base_type` | 基础类型 |
| `basics` | 基础入门 |
| `config` | 配置 |
| `errors` | 错误处理 |
| `learning_additions` | 学习补充 |
| `ownership` | 所有权 |
| `practice_core` | 核心练习 |
| `rust_by_example` | Rust 示例 |
| `structs_enums` | 结构体与枚举 |
| `traits` | 特征 |
| `types` | 类型 |
| `utils` | 工具 |

## 颜色类型

```rust
pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(Debug, PartialEq)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}
```
