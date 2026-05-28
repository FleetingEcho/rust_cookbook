# Rust 学习笔记索引

> 按**学习主题**分类，而不是按文件夹。同一主题的多个深度文件会列在一起。
> 如果你来自 TypeScript，直接去看 [→ TS 对比系列](#-typescript-对比系列)。

---

## 目录

1. [入门基础](#1-入门基础)
2. [核心特性：所有权与借用](#2-核心特性所有权与借用)
3. [类型系统](#3-类型系统)
4. [结构体与枚举](#4-结构体与枚举)
5. [Trait 与泛型](#5-trait-与泛型)
6. [错误处理](#6-错误处理)
7. [集合](#7-集合)
8. [迭代器与闭包](#8-迭代器与闭包)
9. [智能指针与内存管理](#9-智能指针与内存管理)
10. [并发](#10-并发)
11. [异步编程](#11-异步编程)
12. [宏](#12-宏)
13. [模块与包管理](#13-模块与包管理)
14. [进阶主题](#14-进阶主题)
15. [实践与工具](#15-实践与工具)
16. [TypeScript 对比系列](#-typescript-对比系列)
17. [生态系统（常用 Crates）](#-生态系统常用-crates)

---

## 1. 入门基础

最先读这些，对应其他语言的"hello world"阶段。

| 文件 | 内容 |
|------|------|
| [main.md](./main.md) | 程序入口，如何运行示例 |
| [basics/variable.md](./basics/variable.md) | 变量、`mut`、Shadowing、`const` |
| [basics/if_else.md](./basics/if_else.md) | 条件表达式与控制流 |
| [basics/method.md](./basics/method.md) | `impl` 块与方法定义 |
| [basics/comment.md](./basics/comment.md) | 行注释、块注释、文档注释 |
| [basics/formatted_output.md](./basics/formatted_output.md) | `println!` 格式化输出 |
| [config/constants.md](./config/constants.md) | 编译期常量 `const` |

---

## 2. 核心特性：所有权与借用

Rust 最重要也最独特的概念，必须掌握。

| 文件 | 描述 | 使用频率 | 掌握程度 |
|------|------|----------|----------|
| [ownership/ownership.md](./ownership/ownership.md) | 所有权三规则、Move 语义、Copy trait | `████████████████████` 每天用 | 必须掌握 |
| [ownership/lifetime.md](./ownership/lifetime.md) | 生命周期标注，防止悬垂引用 | `████████████████████` 每天用 | 必须掌握 |
| [lifetimes_from_ts_basics.md](./lifetimes_from_ts_basics.md) | 生命周期核心：为什么需要、三大场景、消除规则、编译错误解读、实战 | `████████████████████` 每天用 | **必读** |
| [learning_additions/ownership_borrowing.md](./learning_additions/ownership_borrowing.md) | 借用、可变借用规则、字符串切片 | `████████████░░░░░░░░` 经常用 | 熟练掌握 |
| [advanced/lifetime.md](./advanced/lifetime.md) | 生命周期深入：省略规则、`'static`、结构体中的引用 | `████░░░░░░░░░░░░░░░░` 偶尔查 | 理解概念 |
| [learning_additions/lifetimes.md](./learning_additions/lifetimes.md) | 生命周期补充练习 | `████░░░░░░░░░░░░░░░░` 偶尔翻 | 理解概念 |
| [lifetimes_advanced.md](./lifetimes_advanced.md) | 进阶：Variance / HRTB / GAT / Pin / async | `██░░░░░░░░░░░░░░░░░░` 极少用 | 遇到时回来翻 |

---

## 3. 类型系统

| 文件 | 内容 |
|------|------|
| [base_type/basic.md](./base_type/basic.md) | 整数、浮点、布尔、字符类型与范围 |
| [base_type/string_bool_unit.md](./base_type/string_bool_unit.md) | `char`、`bool`、单元类型 `()` |
| [base_type/string_str_difference.md](./base_type/string_str_difference.md) | `&str` vs `String` 深入对比 |
| [base_type/expression.md](./base_type/expression.md) | 表达式 vs 语句，代码块作为表达式 |
| [base_type/iteration.md](./base_type/iteration.md) | 数组切片与迭代模式 |
| [types/tuple.md](./types/tuple.md) | 元组定义与解构 |
| [types/array.md](./types/array.md) | 固定大小数组与 Copy trait |
| [types/generics.md](./types/generics.md) | 泛型函数与 trait bound |
| [types/compound.md](./types/compound.md) | 类型别名与复合类型 |
| [learning_additions/const_generics.md](./learning_additions/const_generics.md) | `const fn` 与 const 泛型参数 |
| [advanced/converse_type.md](./advanced/converse_type.md) | `as`、`TryInto`、`transmute` 类型转换全览 |
| [advanced/custom_type.md](./advanced/custom_type.md) | Newtype 模式与类型系统设计 |
| [advanced/sized.md](./advanced/sized.md) | Sized 与动态大小类型（DST）|

---

## 4. 结构体与枚举

| 文件 | 内容 |
|------|------|
| [structs_enums/structs.md](./structs_enums/structs.md) | 结构体定义、初始化、字段访问 |
| [structs_enums/enums.md](./structs_enums/enums.md) | 枚举与带数据的变体 |
| [structs_enums/match_basics.md](./structs_enums/match_basics.md) | `match` 表达式基础 |
| [structs_enums/all_pattern.md](./structs_enums/all_pattern.md) | 完整模式匹配：字面量、守卫、`@` 绑定 |
| [structs_enums/pattern_match.md](./structs_enums/pattern_match.md) | Vec 上的模式匹配与 `while let` |
| [learning_additions/pattern_matching.md](./learning_additions/pattern_matching.md) | `if let`、`let else`、解构补充 |
| [advanced/enum_int.md](./advanced/enum_int.md) | 枚举与整数互相转换 |

---

## 5. Trait 与泛型

| 文件 | 内容 |
|------|------|
| [traits/basics.md](./traits/basics.md) | Trait 定义、实现、默认方法 |
| [traits/advanced.md](./traits/advanced.md) | 关联类型、完全限定语法 |
| [traits/trait_objects.md](./traits/trait_objects.md) | `dyn Trait` 动态分发与虚表 |
| [learning_additions/traits_generics.md](./learning_additions/traits_generics.md) | Trait 约束与泛型参数综合练习 |
| [learning_additions/impl_trait.md](./learning_additions/impl_trait.md) | `impl Trait` vs `dyn Trait` 选择指南 |
| [learning_additions/derive_macros.md](./learning_additions/derive_macros.md) | 常用 `#[derive]` 宏：Debug、Clone、PartialEq 等 |
| [advanced/deref.md](./advanced/deref.md) | `Deref` trait 与智能指针解引用 |

---

## 6. 错误处理

| 文件 | 内容 |
|------|------|
| [errors/result_error.md](./errors/result_error.md) | `Result` 基础与 `?` 运算符 |
| [learning_additions/error_handling.md](./learning_additions/error_handling.md) | `Option` 与 `Result` 处理模式 |
| [learning_additions/option_result_combinators.md](./learning_additions/option_result_combinators.md) | `map`、`and_then`、`unwrap_or` 等组合子 |
| [advanced/errors.md](./advanced/errors.md) | 自定义错误类型、`thiserror`、`anyhow` |

---

## 7. 集合

| 文件 | 内容 |
|------|------|
| [collections/vector.md](./collections/vector.md) | `Vec<T>` 创建、修改、迭代 |
| [collections/hashmap.md](./collections/hashmap.md) | `HashMap` 基础操作与 entry API |
| [learning_additions/collections_extra.md](./learning_additions/collections_extra.md) | `HashSet`、`BTreeMap`、`BTreeSet`、`VecDeque` |
| [practice_core/core/array.md](./practice_core/core/array.md) | 数组操作练习 |
| [practice_core/core/hashmap.md](./practice_core/core/hashmap.md) | HashMap + serde 序列化练习 |

---

## 8. 迭代器与闭包

| 文件 | 内容 |
|------|------|
| [advanced/closure.md](./advanced/closure.md) | 闭包、捕获方式、`Fn`/`FnMut`/`FnOnce` |
| [advanced/iterator.md](./advanced/iterator.md) | Iterator trait 与惰性求值 |
| [learning_additions/iterators.md](./learning_additions/iterators.md) | `map`、`filter`、`fold`、`collect` 练习 |
| [base_type/iteration.md](./base_type/iteration.md) | `iter()`、`iter_mut()`、`into_iter()` 区别 |

---

## 9. 智能指针与内存管理

| 文件 | 内容 |
|------|------|
| [advanced/smart_pointer.md](./advanced/smart_pointer.md) | `Box`、`Rc`、`Ref`、`RefMut` |
| [advanced/rc_arc.md](./advanced/rc_arc.md) | 引用计数（`Rc`）与原子引用计数（`Arc`）|
| [advanced/weak.md](./advanced/weak.md) | `Weak` 引用，打破循环引用 |
| [advanced/cell_refcell.md](./advanced/cell_refcell.md) | 内部可变性：`Cell` 与 `RefCell` |
| [advanced/drop.md](./advanced/drop.md) | `Drop` trait，RAII 资源自动释放 |
| [advanced/pin_unpin.md](./advanced/pin_unpin.md) | `Pin`/`Unpin`，内存固定与自引用结构 |
| [advanced/self-referential.md](./advanced/self-referential.md) | 自引用结构体模式与解决方案 |

---

## 10. 并发

| 文件 | 内容 |
|------|------|
| [advanced/concurrency_with_threads.md](./advanced/concurrency_with_threads.md) | 多线程基础与数据竞争 |
| [advanced/concurrency_2.md](./advanced/concurrency_2.md) | `mpsc` channel 消息传递 |
| [advanced/concurrency_3.md](./advanced/concurrency_3.md) | `Mutex`、`RwLock`、条件变量 |
| [advanced/concurrency_4.md](./advanced/concurrency_4.md) | 原子操作与无锁同步原语 |
| [advanced/concurrency_5.md](./advanced/concurrency_5.md) | `Send` 与 `Sync` trait，编译期线程安全 |
| [advanced/global_variable.md](./advanced/global_variable.md) | 全局变量：`const`、`static`、懒初始化 |

---

## 11. 异步编程

| 文件 | 内容 |
|------|------|
| [learning_additions/async_basics.md](./learning_additions/async_basics.md) | `async`/`await` 入门 |
| [advanced/async.md](./advanced/async.md) | `Future` trait 与 async 执行机制 |
| [advanced/multi-futures-simultaneous.md](./advanced/multi-futures-simultaneous.md) | `join!` 与 `select!` 并发执行多个 Future |
| [advanced/stream.md](./advanced/stream.md) | `Stream` trait，异步迭代序列 |

---

## 12. 宏

| 文件 | 内容 |
|------|------|
| [advanced/macro.md](./advanced/macro.md) | 声明宏（`macro_rules!`）与过程宏 |
| [basics/formatted_output.md](./basics/formatted_output.md) | `println!`/`format!` 格式化语法详解 |

---

## 13. 模块与包管理

| 文件 | 内容 |
|------|------|
| [package_module/crate.md](./package_module/crate.md) | Crate 结构、`use` 导入、模块可见性 |
| [learning_additions/modules_and_testing.md](./learning_additions/modules_and_testing.md) | `pub` 可见性修饰与基础测试 |
| [learning_additions/cargo_features.md](./learning_additions/cargo_features.md) | `Cargo.toml` 中的可选 feature flags |
| [learning_additions/env_process.md](./learning_additions/env_process.md) | 环境变量读取与进程控制 |

---

## 14. 进阶主题

| 文件 | 内容 |
|------|------|
| [advanced/unsafe_superpowers.md](./advanced/unsafe_superpowers.md) | 五种 unsafe 操作：裸指针、unsafe fn、extern |
| [advanced/inline_assembly.md](./advanced/inline_assembly.md) | `asm!` 宏，内联汇编 |
| [learning_additions/serde_basics.md](./learning_additions/serde_basics.md) | `serde` 序列化/反序列化基础 |
| [learning_additions/testing_advanced.md](./learning_additions/testing_advanced.md) | 文档测试、`#[should_panic]`、测试配置 |
| [advanced/compiled_examples.md](./advanced/compiled_examples.md) | 可直接 cargo test 的稳定示例合集 |

---

## 15. 实践与工具

| 文件 | 内容 |
|------|------|
| [utils/math.md](./utils/math.md) | 数学工具函数 |
| [utils/helper.md](./utils/helper.md) | 通用 helper 函数 |
| [utils/string.md](./utils/string.md) | 字符串工具函数 |
| [rust_by_example/runner_notes.md](./rust_by_example/runner_notes.md) | Rust by Example 练习入口 |

---

## → TypeScript 对比系列

专为 TypeScript 开发者设计，每个主题都有 TS 代码对照。

**[进入 TS 对比系列总览 →](../examples/rust_vs_typescript/README.md)**

| 文件 | 主题 |
|------|------|
| [variables.md](../examples/rust_vs_typescript/variables.md) | 变量、`mut`、Shadowing |
| [primitives.md](../examples/rust_vs_typescript/primitives.md) | 数值类型与转换 |
| [strings.md](../examples/rust_vs_typescript/strings.md) | `&str` vs `String` |
| [functions.md](../examples/rust_vs_typescript/functions.md) | 函数、闭包、高阶函数 |
| [ownership_borrowing.md](../examples/rust_vs_typescript/ownership_borrowing.md) | 所有权与借用 |
| [structs.md](../examples/rust_vs_typescript/structs.md) | 结构体（代替 class） |
| [enums.md](../examples/rust_vs_typescript/enums.md) | 枚举（代替判别联合） |
| [pattern_matching.md](../examples/rust_vs_typescript/pattern_matching.md) | 模式匹配 |
| [traits.md](../examples/rust_vs_typescript/traits.md) | Trait（代替 interface） |
| [generics.md](../examples/rust_vs_typescript/generics.md) | 泛型 |
| [option_result.md](../examples/rust_vs_typescript/option_result.md) | Option / Result（代替 null / try-catch） |
| [error_handling_advanced.md](../examples/rust_vs_typescript/error_handling_advanced.md) | 高级错误处理、thiserror、anyhow |
| [closures_iter.md](../examples/rust_vs_typescript/closures_iter.md) | 闭包与迭代器链 |
| [smart_pointers.md](../examples/rust_vs_typescript/smart_pointers.md) | 智能指针 |
| [lifetimes.md](../examples/rust_vs_typescript/lifetimes.md) | 生命周期 |
| [async_await.md](../examples/rust_vs_typescript/async_await.md) | 异步编程、`select!` |
| [concurrency.md](../examples/rust_vs_typescript/concurrency.md) | 多线程与并发（vs Worker）|
| [cargo.md](../examples/rust_vs_typescript/cargo.md) | Cargo 包管理（vs npm）|
| [file_io.md](../examples/rust_vs_typescript/file_io.md) | 文件读写、路径操作、`tokio::fs` |
| [common_mistakes.md](../examples/rust_vs_typescript/common_mistakes.md) | 新手 / 中级常见错误 20 条 |

---

## → 生态系统（常用 Crates）

**[popular_crates.md](./ecosystem/popular_crates.md)** — 15 个最常用第三方包速览

| Crate | 用途 |
|-------|------|
| `serde` + `serde_json` | JSON 序列化（极高频）|
| `tokio` | 异步运行时 |
| `reqwest` | HTTP 客户端 |
| `axum` | Web 框架 |
| `clap` | CLI 参数解析 |
| `anyhow` / `thiserror` | 错误处理 |
| `tracing` | 结构化日志 |
| `rayon` | 数据并行 |
| `sqlx` | 异步数据库 |
| `chrono` / `rand` / `regex` / `uuid` | 日期、随机、正则、UUID |

---

## 快速入口

| 目标 | 直接跳到 |
|------|---------|
| 我是 TS 开发者，刚开始学 | [TS 对比系列 README](../examples/rust_vs_typescript/README.md) |
| 我想了解所有权 | [ownership.md](./ownership/ownership.md) |
| 我遇到借用检查器报错 | [common_mistakes.md](../examples/rust_vs_typescript/common_mistakes.md) |
| 我要做异步/网络请求 | [async_await.md](../examples/rust_vs_typescript/async_await.md) + [popular_crates.md](./ecosystem/popular_crates.md) |
| 我要写多线程代码 | [concurrency.md](../examples/rust_vs_typescript/concurrency.md) |
| 我需要处理错误 | [error_handling_advanced.md](../examples/rust_vs_typescript/error_handling_advanced.md) |
| 我想了解有哪些常用库 | [popular_crates.md](./ecosystem/popular_crates.md) |
