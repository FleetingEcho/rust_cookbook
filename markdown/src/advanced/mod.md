# Advanced 进阶主题索引

这个目录收录了所有高级主题的学习笔记。

## 使用说明

> 这里的大多数文件是**片段集合**：同一个文件里有多个独立的 `fn main()`，
> 用来演示同一主题的不同侧面。  
> **阅读学习用**，不要直接整体接入模块树编译。  
> 需要可运行的代码，把感兴趣的片段复制到自己的项目里单独跑。

| 标记 | 含义 |
|------|------|
| 📖 片段集合 | 多个独立 `main()`，阅读用 |
| ✅ 可编译模块 | 已组织为 `pub fn`，可直接 `use` |

唯一可整体编译的文件：**[compiled_examples.md](./compiled_examples.md)**

---

## 按主题分类

### 内存管理

| 文件 | 主题 | 状态 |
|------|------|------|
| [smart_pointer.md](./smart_pointer.md) | `Box`、`Rc`、`Ref`、`RefMut` 基础用法 | 📖 片段集合 |
| [rc_arc.md](./rc_arc.md) | 引用计数 `Rc` 与原子引用计数 `Arc` | 📖 片段集合 |
| [weak.md](./weak.md) | `Weak` 弱引用，打破循环引用 | 📖 片段集合 |
| [cell_refcell.md](./cell_refcell.md) | 内部可变性：`Cell` 与 `RefCell` | 📖 片段集合 |
| [deref.md](./deref.md) | `Deref` trait，自动解引用 | 📖 片段集合 |
| [drop.md](./drop.md) | `Drop` trait，RAII 资源自动释放，析构顺序 | 📖 片段集合 |
| [pin_unpin.md](./pin_unpin.md) | `Pin`/`Unpin`，内存固定，async Future 背后机制 | 📖 片段集合 |
| [self-referential.md](./self-referential.md) | 自引用结构体及其解决方案 | 📖 片段集合 |
| [sized.md](./sized.md) | `Sized` trait 与动态大小类型（DST） | 📖 片段集合 |

### 并发

| 文件 | 主题 | 状态 |
|------|------|------|
| [concurrency_with_threads.md](./concurrency_with_threads.md) | 线程创建、`join`、`move`、`thread::sleep` | 📖 片段集合 |
| [concurrency_2.md](./concurrency_2.md) | `mpsc` channel，消息传递并发 | 📖 片段集合 |
| [concurrency_3.md](./concurrency_3.md) | `Mutex`、`RwLock`、条件变量 `Condvar` | 📖 片段集合 |
| [concurrency_4.md](./concurrency_4.md) | 原子操作 `AtomicUsize`，`Ordering` | 📖 片段集合 |
| [concurrency_5.md](./concurrency_5.md) | `Send`/`Sync` trait，线程安全的类型边界 | 📖 片段集合 |
| [global_variable.md](./global_variable.md) | `const`、`static`、`OnceLock`、`LazyLock` | 📖 片段集合 |

### 异步编程

| 文件 | 主题 | 状态 |
|------|------|------|
| [async.md](./async.md) | `async fn`、`Future`、tokio 运行时基础 | 📖 片段集合 |
| [multi-futures-simultaneous.md](./multi-futures-simultaneous.md) | `join!`、`select!`，并发执行多个 Future | 📖 片段集合 |
| [stream.md](./stream.md) | `Stream` trait，异步迭代器 | 📖 片段集合 |

### 类型系统

| 文件 | 主题 | 状态 |
|------|------|------|
| [closure.md](./closure.md) | 闭包捕获、`Fn`/`FnMut`/`FnOnce`、`move` | 📖 片段集合 |
| [iterator.md](./iterator.md) | 自定义 `Iterator`、适配器、消费者 | 📖 片段集合 |
| [lifetime.md](./lifetime.md) | 生命周期省略、`'static`、结构体中的引用 | 📖 片段集合 |
| [converse_type.md](./converse_type.md) | `as`、`From`/`Into`、`TryFrom`/`TryInto`、`transmute` | 📖 片段集合 |
| [custom_type.md](./custom_type.md) | Newtype 模式，类型别名，类型系统设计 | 📖 片段集合 |
| [enum_int.md](./enum_int.md) | 枚举与整数互转，`#[repr]` 属性 | 📖 片段集合 |

### 错误处理

| 文件 | 主题 | 状态 |
|------|------|------|
| [errors.md](./errors.md) | 自定义错误枚举、`From`、`Box<dyn Error>`、`thiserror` | 📖 片段集合 |

### 宏

| 文件 | 主题 | 状态 |
|------|------|------|
| [macro.md](./macro.md) | `macro_rules!` 声明宏，过程宏简介 | 📖 片段集合 |

### 底层 / Unsafe

| 文件 | 主题 | 状态 |
|------|------|------|
| [unsafe_superpowers.md](./unsafe_superpowers.md) | 裸指针、unsafe fn、extern "C"、可变静态变量 | 📖 片段集合 |
| [inline_assembly.md](./inline_assembly.md) | `asm!` 宏，内联汇编 | 📖 片段集合 |

### 可直接编译的参考实现

| 文件 | 内容 | 状态 |
|------|------|------|
| [compiled_examples.md](./compiled_examples.md) | Rc/RefCell 计数器、Arc/Mutex 计数器、Box Deref | ✅ 可编译模块 |

---

## 推荐阅读顺序

如果你刚开始看进阶内容，建议按这个顺序：

```
所有权深入     deref → drop → cell_refcell → rc_arc → weak
智能指针        smart_pointer → pin_unpin
类型系统        closure → iterator → converse_type → custom_type
并发基础        concurrency_with_threads → concurrency_2 → concurrency_3
并发进阶        concurrency_4 → concurrency_5 → global_variable
异步            async → multi-futures-simultaneous → stream
错误处理        errors
底层            sized → unsafe_superpowers → inline_assembly
```

---

## 与 TS 对比系列的对应关系

如果想看同一主题的 TypeScript 对照版本：

| 本目录文件 | TS 对比系列 |
|-----------|------------|
| closure.md / iterator.md | [closures_iter.md](../../../examples/rust_vs_typescript/closures_iter.md) |
| rc_arc.md / cell_refcell.md | [smart_pointers.md](../../../examples/rust_vs_typescript/smart_pointers.md) |
| lifetime.md | [lifetimes.md](../../../examples/rust_vs_typescript/lifetimes.md) |
| concurrency_*.md | [concurrency.md](../../../examples/rust_vs_typescript/concurrency.md) |
| async.md / multi-futures-simultaneous.md | [async_await.md](../../../examples/rust_vs_typescript/async_await.md) |
| errors.md | [error_handling_advanced.md](../../../examples/rust_vs_typescript/error_handling_advanced.md) |
| drop.md | [raii_drop.md](../../../examples/rust_vs_typescript/raii_drop.md) |
