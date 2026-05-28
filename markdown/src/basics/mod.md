# 基础入门模块结构

## 概述

`basics` 模块涵盖 Rust 基础入门的核心概念，包括变量、控制流、注释、格式化输出和方法。

## 模块结构

```rust
//! 基础入门：变量、控制流、注释、格式化输出、方法

pub mod comment;
pub mod formatted_output;
pub mod if_else;
pub mod method;
pub mod variable;
```

## 子模块说明

| 模块 | 文件 | 内容 |
|------|------|------|
| `variable` | [variable.md](variable.md) | 可变与不可变变量、解构赋值、常量、变量遮蔽 |
| `comment` | [comment.md](comment.md) | 行注释、块注释、文档注释、Doc Test |
| `formatted_output` | [formatted_output.md](formatted_output.md) | `println!` 格式化、位置/命名参数、Debug 格式化 |
| `if_else` | [if_else.md](if_else.md) | if 表达式、for 循环、while 循环、loop 循环 |
| `method` | [method.md](method.md) | 结构体方法、枚举方法、`&self` / `&mut self` / `self` |
