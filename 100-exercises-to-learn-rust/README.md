# 100 Exercises to Learn Rust — 独立单文件版本

从 [mainmatter/100-exercises-to-learn-rust](https://github.com/mainmatter/100-exercises-to-learn-rust) 克隆并整理为独立单文件，
每个文件已**完成答案**并附带**中文要点注释**（🔑）。

```bash
cargo test                           # 运行所有 std-only 练习的测试
cargo test --test ch03_02_validation  # 运行单个练习
cargo test --features ch08 --test ch08_01_async_fn  # 运行第8章（需 tokio）
```

## 第一章：入门 (01_intro)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_welcome.rs`](01_intro/00_welcome.rs) | Rust 入门欢迎，了解 `&str` 字符串字面量 | Rust 注释、`assert_eq!` 测试宏、`&'static str` |
| [`01_syntax.rs`](01_intro/01_syntax.rs) | 函数签名语法——参数必须标注类型 | 强类型、函数签名、返回值类型推理 |

## 第二章：基础计算器 (02_basic_calculator)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](02_basic_calculator/00_intro.rs) | 字符串字面量练习 | `&'static str`、字符串字面量 |
| [`01_integers.rs`](02_basic_calculator/01_integers.rs) | 整数类型匹配——u8 与 u32 不能直接运算 | 整数类型、类型一致性、编译器错误 |
| [`02_variables.rs`](02_basic_calculator/02_variables.rs) | 变量声明 `let`，类型自动推理 | `let` 绑定、类型推理、整数除法截断 |
| [`03_if_else.rs`](02_basic_calculator/03_if_else.rs) | 条件判断 `if/else if/else` | if 是表达式、条件优先级、`%` 取模 |
| [`04_panics.rs`](02_basic_calculator/04_panics.rs) | 用 `panic!` 处理不可恢复错误 | `panic!` 宏、`#[should_panic]`、错误消息匹配 |
| [`05_factorial.rs`](02_basic_calculator/05_factorial.rs) | **递归**实现阶乘 | 递归函数、base case、栈溢出风险 |
| [`06_while.rs`](02_basic_calculator/06_while.rs) | **while 循环**实现阶乘 | `while` 循环、`mut` 可变绑定、递增 `+=` |
| [`07_for.rs`](02_basic_calculator/07_for.rs) | **for 循环**实现阶乘（最地道） | `for` 循环、`1..=n` 范围表达式、迭代器 |
| [`08_overflow.rs`](02_basic_calculator/08_overflow.rs) | 整数溢出——使用 `wrapping_mul` | `wrapping_mul`、debug 模式溢出检查、回绕语义 |
| [`09_saturating.rs`](02_basic_calculator/09_saturating.rs) | 饱和运算——使用 `saturating_mul` | `saturating_mul`、饱和 vs 回绕、`u32::MAX` |
| [`10_as_casting.rs`](02_basic_calculator/10_as_casting.rs) | 类型转换 `as` 关键字 | `as` 转换、`u8→i8` 截断、`bool→u8`、`#[allow(overflowing_literals)]` |

## 第三章：票据V1 (03_ticket_v1)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](03_ticket_v1/00_intro.rs) | 章节介绍 | |
| [`01_struct.rs`](03_ticket_v1/01_struct.rs) | 定义 `Order` 结构体和方法 | `struct` 定义、`impl` 块、`&self` 方法 |
| [`02_validation.rs`](03_ticket_v1/02_validation.rs) | `Ticket::new` 构造函数验证 | 构造函数模式、`String::is_empty()`/`len()`、panic 验证 |
| [`03_modules.rs`](03_ticket_v1/03_modules.rs) | 模块 `mod` 和 `super::` 路径 | 子模块、`super::` 访问父模块、路径解析 |
| [`04_visibility.rs`](03_ticket_v1/04_visibility.rs) | 可见性 `pub` 控制 | `pub` 关键字、默认私有、封装性 |
| [`05_encapsulation.rs`](03_ticket_v1/05_encapsulation.rs) | Getter 方法实现封装 | getter 模式、`&self` 返回引用、`pub` 方法 |
| [`06_ownership.rs`](03_ticket_v1/06_ownership.rs) | `&self` vs `self`——所有权区别 | 所有权、借用 `&self`、方法参数 `self` |
| [`07_setters.rs`](03_ticket_v1/07_setters.rs) | `&mut self` setter 方法 | 可变借用、私有辅助方法、setter 模式 |
| [`08_stack.rs`](03_ticket_v1/08_stack.rs) | 基础类型在栈上的大小 | `std::mem::size_of`、`u16=2B`、`i32=4B`、`bool=1B` |
| [`09_heap.rs`](03_ticket_v1/09_heap.rs) | `String` 和 `Ticket` 在栈上的大小 | `String=24B`（ptr+len+capacity）、堆 vs 栈 |
| [`10_references_in_memory.rs`](03_ticket_v1/10_references_in_memory.rs) | 引用在 64 位系统上固定 8 字节 | 引用大小、指针大小、64 位 vs 32 位 |
| [`11_destructor.rs`](03_ticket_v1/11_destructor.rs) | `Drop` 析构概念介绍 | `Drop` trait、资源释放、RAII |
| [`12_outro.rs`](03_ticket_v1/12_outro.rs) | 综合练习：实现完整 `Order` 类型 | `pub` API 设计、getter/setter、验证、总价计算 |

## 第四章：Trait (04_traits)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](04_traits/00_intro.rs) | 章节介绍 | |
| [`01_trait.rs`](04_traits/01_trait.rs) | 定义和实现 `IsEven` trait | `trait` 定义、为 `u32`/`i32` 实现 trait、方法调用 |
| [`02_orphan_rule.rs`](04_traits/02_orphan_rule.rs) | 孤儿规则（Orphan Rule） | 孤儿规则、一致性、不能为外部类型实现外部 trait |
| [`03_operator_overloading.rs`](04_traits/03_operator_overloading.rs) | 实现 `PartialEq` 运算符重载 | `PartialEq` trait、`==`/`!=` 运算符、手动 `eq` 实现 |
| [`04_derive.rs`](04_traits/04_derive.rs) | `#[derive(Debug, PartialEq)]` | `derive` 宏、`Debug` 格式化、自动实现 |
| [`05_trait_bounds.rs`](04_traits/05_trait_bounds.rs) | 泛型的 trait bound 约束 | `PartialOrd`、`T: PartialOrd`、`where` 子句 |
| [`06_str_slice.rs`](04_traits/06_str_slice.rs) | 返回 `&str` 而非 `&String` | Deref 强制转换、`&str` vs `&String`、`type_id()` |
| [`07_deref.rs`](04_traits/07_deref.rs) | `str::trim()` 去除首尾空白 | Deref 强制转换、`trim()` 方法、`str` vs `String` |
| [`08_sized.rs`](04_traits/08_sized.rs) | 动态大小类型（DST）`str` | `Sized` trait、DST、`?Sized` 边界 |
| [`09_from.rs`](04_traits/09_from.rs) | `From`/`Into` 类型转换 | `From` trait、`Into` trait、`42.into()` |
| [`10_assoc_vs_generic.rs`](04_traits/10_assoc_vs_generic.rs) | 泛型参数 vs 关联类型 | 泛型 `Power<Exponent>`、多次实现、`self.pow()` |
| [`11_clone.rs`](04_traits/11_clone.rs) | `#[derive(Clone)]` 显式复制 | `Clone` trait、`.clone()`、所有权与复制 |
| [`12_copy.rs`](04_traits/12_copy.rs) | `#[derive(Copy)]` 自动复制 | `Copy` trait（子 trait of `Clone`）、自动复制 vs 移动 |
| [`13_drop.rs`](04_traits/13_drop.rs) | Drop Bomb 模式 | `Drop` trait、析构函数、panic unless defused |
| [`14_outro.rs`](04_traits/14_outro.rs) | 综合练习：实现 `SaturatingU16` | 多个 `From`/`Add` 实现、`saturating_add`、`PartialEq<u16>` |

## 第五章：票据V2 (05_ticket_v2)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](05_ticket_v2/00_intro.rs) | 章节介绍 | |
| [`01_enum.rs`](05_ticket_v2/01_enum.rs) | 用 `Status` 枚举替代字符串 | `enum` 定义、`derive` 递归要求、类型安全 |
| [`02_match.rs`](05_ticket_v2/02_match.rs) | `match` 穷举匹配 | match 穷举性、`assert_eq!`、模式匹配 |
| [`03_variants_with_data.rs`](05_ticket_v2/03_variants_with_data.rs) | 带数据的枚举变体 | `InProgress{assigned_to}`、匹配带数据变体、panic vs 安全 |
| [`04_if_let.rs`](05_ticket_v2/04_if_let.rs) | `if let` 简洁模式匹配 | `if let` 语法、只匹配一个模式、`let else` |
| [`05_nullability.rs`](05_ticket_v2/05_nullability.rs) | `Option<T>` 替代 panic | `Option`、`Some`/`None`、`is_none()`、类型安全 |
| [`06_fallibility.rs`](05_ticket_v2/06_fallibility.rs) | `Result<T, E>` 替代 panic | `Result`、`Ok`/`Err`、`unwrap_err()`、错误传播 |
| [`07_unwrap.rs`](05_ticket_v2/07_unwrap.rs) | 标题 panic / 描述默认值 | `unwrap`/`expect`、选择性 panic、默认值策略 |
| [`08_error_enums.rs`](05_ticket_v2/08_error_enums.rs) | 自定义错误枚举 | 错误枚举类型、`TitleError`/`DescriptionError` 区分 |
| [`09_error_trait.rs`](05_ticket_v2/09_error_trait.rs) | 实现 `Display` + `Error` trait | `std::error::Error`、`Display` 格式化、`#[derive(Debug)]` |
| [`10_packages.rs`](05_ticket_v2/10_packages.rs) | 库+二进制项目结构 | `lib.rs` vs `main.rs`、`pub fn`、二进制入口 |
| [`11_dependencies.rs`](05_ticket_v2/11_dependencies.rs) | 外部依赖概念 | Cargo.toml `[dependencies]`、`use anyhow::Error` |
| [`12_thiserror.rs`](05_ticket_v2/12_thiserror.rs) | 手动实现 `Error` 替代 thiserror | 手动 `Display`+`Debug`+`Error`、`thiserror` 原理 |
| [`13_try_from.rs`](05_ticket_v2/13_try_from.rs) | `TryFrom<String>` 解析状态 | `TryFrom` trait、不区分大小写、`to_lowercase()` |
| [`14_source.rs`](05_ticket_v2/14_source.rs) | 错误链——`source()` 方法 | `Error::source()`、`Box<dyn Error>`、模块分离 |
| [`15_outro.rs`](05_ticket_v2/15_outro.rs) | 综合练习：模块化 Ticket 类型 | re-export 模式、内联子模块、类型级验证 |

## 第六章：票据管理 (06_ticket_management)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](06_ticket_management/00_intro.rs) | 章节介绍 | |
| [`01_arrays.rs`](06_ticket_management/01_arrays.rs) | 数组实现周温度存储 | 固定大小数组 `[Option<i32>; 7]`、枚举→索引映射 |
| [`02_vec.rs`](06_ticket_management/02_vec.rs) | Vec 记忆化 Fibonacci | `Vec` 动态数组、`push` 追加、记忆化 |
| [`03_resizing.rs`](06_ticket_management/03_resizing.rs) | Vec 扩容策略 | `Vec::with_capacity`、自动扩容翻倍、`capacity()` |
| [`04_iterators.rs`](06_ticket_management/04_iterators.rs) | 实现 `IntoIterator` for TicketStore | `IntoIterator` trait、`into_iter()`、委托给 `Vec` |
| [`05_iter.rs`](06_ticket_management/05_iter.rs) | 添加 `iter()` 方法 | `iter()` → `std::slice::Iter`、借用迭代 |
| [`06_lifetimes.rs`](06_ticket_management/06_lifetimes.rs) | `IntoIterator for &TicketStore` | 生命周期标注 `'a`、引用迭代器、`for &store` |
| [`07_combinators.rs`](06_ticket_management/07_combinators.rs) | `filter` 迭代器适配器 | 迭代器适配器、`filter`、`collect`、链式调用 |
| [`08_impl_trait.rs`](06_ticket_management/08_impl_trait.rs) | `impl Trait` 返回迭代器 | `impl Iterator`、隐藏返回类型、零成本抽象 |
| [`09_impl_trait_2.rs`](06_ticket_management/09_impl_trait_2.rs) | 泛型参数 `T: Into<Ticket>` | 泛型参数 vs `impl Trait`、`From`/`Into` 转换 |
| [`10_slices.rs`](06_ticket_management/10_slices.rs) | 切片 `&[u32]` 求和 | 切片类型、`&[T]` 可接受 `Vec<T>` 和 `[T; N]` |
| [`11_mutable_slices.rs`](06_ticket_management/11_mutable_slices.rs) | 可变切片 `&mut [i32]` 平方 | `&mut [T]`、`iter_mut()`、原地修改 |
| [`12_two_states.rs`](06_ticket_management/12_two_states.rs) | `TicketDraft` → `TicketId` | TicketDraft/Ticket 分离、唯一 ID、`Option` 查找 |
| [`13_index.rs`](06_ticket_management/13_index.rs) | 实现 `Index` trait | `Index<T>` trait、`store[id]` 语法、`Output` 关联类型 |
| [`14_index_mut.rs`](06_ticket_management/14_index_mut.rs) | 实现 `IndexMut` trait | `IndexMut` trait、`&mut store[id]`、可变索引 |
| [`15_hashmap.rs`](06_ticket_management/15_hashmap.rs) | HashMap 存储票据 | `HashMap<K,V>`、`Hash + Eq` bound、`insert`/`get` |
| [`16_btreemap.rs`](06_ticket_management/16_btreemap.rs) | BTreeMap 有序遍历 | `BTreeMap` 按键排序、`IntoIterator` 按序遍历、`Ord` trait |

## 第七章：线程 (07_threads)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](07_threads/00_intro.rs) | 章节介绍 | |
| [`01_threads.rs`](07_threads/01_threads.rs) | `thread::spawn` 多线程求和 | 线程创建、`join()`、move 闭包、所有权转移 |
| [`02_static.rs`](07_threads/02_static.rs) | `'static` 生命周期共享 | `'static` 切片、`static` 变量、线程安全借用 |
| [`03_leak.rs`](07_threads/03_leak.rs) | `Vec::leak` 泄漏为 `'static` | `leak()` 方法、`&'static mut [T]`、内存泄漏 |
| [`04_scoped_threads.rs`](07_threads/04_scoped_threads.rs) | `thread::scope` 借用线程 | `scope` 线程、借用局部变量、自动 join |
| [`05_channels.rs`](07_threads/05_channels.rs) | mpsc 通道基础 | `mpsc::channel`、`Sender`/`Receiver`、服务器循环 |
| [`06_interior_mutability.rs`](07_threads/06_interior_mutability.rs) | `Rc<RefCell<T>>` DropTracker | 内部可变性、`RefCell`、`Rc` 多所有者、`Drop` 计数 |
| [`14_sync.rs`](07_threads/14_sync.rs) | `Send` + `Sync` 标记 trait | `Send`（跨线程传输）、`Sync`（跨线程共享）、自动实现 |

## 第八章：异步编程 (08_futures)

| 文件 | 描述 | 考点 |
|------|------|------|
| [`00_intro.rs`](08_futures/00_intro.rs) | 章节介绍 | |
| [`01_async_fn.rs`](08_futures/01_async_fn.rs) | async echo 服务器 | `async fn`、`TcpListener`、`tokio::io::copy`、`.await` |
| [`02_spawn.rs`](08_futures/02_spawn.rs) | `tokio::spawn` 并发 | `spawn` 并发、`JoinSet`、双 listener |
| [`03_runtime.rs`](08_futures/03_runtime.rs) | 运行时与 `Arc` 共享 | tokio 运行时、`Arc` 共享数据、`'static` + `Send + Sync` |
| [`04_future.rs`](08_futures/04_future.rs) | Future 惰性与 `Rc` 限制 | Future 惰性、`Send` 约束、`Rc` 跨 await 问题、语句重排 |
| [`05_blocking.rs`](08_futures/05_blocking.rs) | `spawn_blocking` 处理同步 IO | `spawn_blocking`、同步 IO 阻塞、死锁避免 |
| [`06_async_aware_primitives.rs`](08_futures/06_async_aware_primitives.rs) | 异步通道 vs 同步通道 | `std::sync::mpsc` 阻塞问题、`tokio::sync::mpsc`、死锁 |
| [`07_cancellation.rs`](08_futures/07_cancellation.rs) | `tokio::time::timeout` 超时 | 超时、Future 取消、协作式取消、部分读取 |
| [`08_outro.rs`](08_futures/08_outro.rs) | 综合项目：异步 REST API | axum/actix-web、CRUD、端点设计 |
