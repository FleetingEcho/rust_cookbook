# Rust vs TypeScript 对比学习

> 🎯 专为 **TypeScript 开发者**设计的 Rust 学习路径
>
> 每个 `.md` 文件都是一篇独立的对照笔记，包含完整的 Rust 代码 + TypeScript 注释版本 + 差异总结。通过熟悉的 TS 语法快速理解 Rust 概念。

---

## 🗺️ 推荐学习路径

按 **认知依赖关系** 排列，建议从上往下依次学习。每个文件顶部都标注了运行命令。

### 🟢 第一阶段：基础语法（从 TS 平滑过渡）

| # | 文件 | 核心概念 | 与 TS 的关键差异 | 使用频率 | 掌握程度 |
|---|------|---------|----------------|----------|----------|
| 1 | [变量](./variables.md) | `let`、`mut`、Shadowing、`const`、解构 | `let` 默认不可变；Shadowing 可改类型 | `██████░░░░░░░░░░░░░░` 入门通读 | 熟悉概念 |
| 2 | [原始类型](./primitives.md) | `i32`/`u8`/`f64`/`bool`/`char` | 区分位宽和符号；`char` 是 Unicode 标量 | `██████░░░░░░░░░░░░░░` 入门通读 | 熟悉概念 |
| 3 | [元组](./tuples.md) | `(T1, T2)`、解构、元组结构体 | `.0` 索引 vs `[0]`；单元素必须逗号 | `██████░░░░░░░░░░░░░░` 入门通读 | 熟悉概念 |
| 4 | [字符串](./strings.md) | `&str` vs `String` | 两种字符串类型；`len()` 是字节数 | `████████████████████` 每天用 | **必须掌握** |
| 5 | [函数](./functions.md) | `fn`、表达式体、高阶函数、发散函数 `!` | 无 `return` 的表达式返回；方法用 `&self` | `████████░░░░░░░░░░░░` 入门通读 | 熟悉概念 |

### 🟡 第二阶段：Rust 核心特性（认知飞跃区）

| # | 文件 | 核心概念 | 与 TS 的关键差异 | 使用频率 | 掌握程度 |
|---|------|---------|----------------|----------|----------|
| 6 | [所有权与借用](./ownership_borrowing.md) | Move、`&`、`&mut`、`Clone` | ⚠️ Rust 独有；多读或一写的严格规则 | `████████████████████` **每天用** | **必须掌握** |
| 7 | [结构体](./structs.md) | `struct`、`impl`、`#[derive]`、泛型结构体 | `struct` + `impl` 代替 `class` | `████████████████████` 每天用 | 必须掌握 |
| 8 | [枚举](./enums.md) | `enum` 带数据、`Option`/`Result` 源码 | ⚠️ Rust 王牌特性，TS 判别联合增强版 | `████████████████████` 每天用 | 必须掌握 |
| 9 | [模式匹配](./pattern_matching.md) | `match`、守卫、`@` 绑定、`if let`、`let else` | 编译器强制穷举；`match` 是表达式 | `████████████░░░░░░░░` 经常用 | 熟练掌握 |
| 10 | [控制流](./control_flow.md) | `if`、`loop`、`while`、`for` | `if` 是表达式；`loop` 可返回值 | `████████░░░░░░░░░░░░` 入门通读 | 熟悉概念 |

### 🟠 第三阶段：类型系统深入

| # | 文件 | 核心概念 | 与 TS 的关键差异 | 使用频率 | 掌握程度 |
|---|------|---------|----------------|----------|----------|
| 11 | [Option 与 Result](./option_result.md) | `Some`/`None`、`Ok`/`Err`、`?` 运算符、combinators | 代替 `null`/`undefined` 和 `try/catch` | `████████████████████` 每天用 | **必须掌握** |
| 12 | [高级错误处理](./error_handling_advanced.md) | 自定义错误类型、`From` trait、`thiserror`、`anyhow` | 枚举错误 vs `extends Error` | `████████░░░░░░░░░░░░` 项目大了后 | 熟练掌握 |
| 13 | [Trait](./traits.md) | trait 定义/实现、默认方法、`dyn`、标准库 trait | 代替 `interface` + `abstract class` | `████████████████████` 每天用 | **必须掌握** |
| 14 | [泛型](./generics.md) | 泛型函数/结构体、trait bound、关联类型、const 泛型 | 单态化零开销 vs 运行时擦除 | `████████████████████` 每天用 | **必须掌握** |

### 🔴 第四阶段：Rust 独有概念（TS 完全没有）

| # | 文件 | 核心概念 | 与 TS 的关键差异 | 使用频率 | 掌握程度 |
|---|------|---------|----------------|----------|----------|
| 15 | [生命周期](./lifetimes.md) | `'a`、省略规则、`'static` | ⚠️ 编译期验证引用安全，零运行时开销 | `████████████░░░░░░░░` 经常用 | **必须掌握** |
| 16 | [RAII 与 Drop](./raii_drop.md) | `Drop` trait、资源自动释放、作用域守卫 | `drop()` vs `try/finally`；编译器确保释放 | `██████░░░░░░░░░░░░░░` 偶尔 | 理解概念 |
| 17 | [智能指针](./smart_pointers.md) | `Box`、`Rc`、`RefCell`、`Arc`、`Mutex` | 代替 GC；按场景选择合适指针 | `████████░░░░░░░░░░░░` 有需要时 | 理解概念 |
| 18 | [闭包与迭代器](./closures_iter.md) | `Fn`/`FnMut`/`FnOnce`、迭代器链 | 三大闭包 trait vs 统一箭头函数；惰性求值 | `████████████░░░░░░░░` 经常用 | 熟练掌握 |

### 🟣 第五阶段：进阶主题

| # | 文件 | 核心概念 | 与 TS 的关键差异 | 使用频率 | 掌握程度 |
|---|------|---------|----------------|----------|----------|
| 19 | [数组与 Vec](./arrays.md) | `[T;N]`、`Vec<T>`、切片、迭代器方法 | 固定数组 + Vec vs `Array<T>` | `████████████████████` 每天用 | 必须掌握 |
| 20 | [HashMap 与集合](./hashmaps.md) | `HashMap`、`HashSet`、`BTreeMap`、entry API | entry API 无 TS 对应 | `████████████████████` 每天用 | 必须掌握 |
| 21 | [模块系统](./modules.md) | `mod`、`pub`、`use`、可见性修饰符 | `mod` 显式构建树 vs 文件即模块 | `████████████░░░░░░░░` 经常用 | 熟练掌握 |
| 22 | [宏](./macros.md) | `println!`、`assert!`、`dbg!`、`macro_rules!` | 编译期展开，接受表达式和代码块 | `████░░░░░░░░░░░░░░░░` 偶尔 | 理解概念 |
| 23 | [异步](./async_await.md) | `async`/`await`、`Future`、tokio | ⚠️ 惰性 Future vs 立即执行的 Promise | `████████░░░░░░░░░░░░` 有需要时 | 熟练掌握 |
| 24 | [测试](./testing.md) | `#[test]`、`#[should_panic]`、文档测试 | `#[test]` vs `describe/it/expect` | `████████░░░░░░░░░░░░` 写测试时 | 熟练掌握 |
| 25 | [并发与多线程](./concurrency.md) | `thread::spawn`、`mpsc`、`Arc<Mutex<T>>` | ⚠️ 真正并行；编译期线程安全检查 | `████░░░░░░░░░░░░░░░░` 有需要时 | 理解概念 |
| 26 | [包管理与项目结构](./cargo.md) | `Cargo.toml`、features、workspace | 对应 npm/package.json 的完整工具链 | `██████░░░░░░░░░░░░░░` 入门通读 | 熟悉概念 |
| 27 | [文件 IO 与路径](./file_io.md) | `std::fs`、`Path`/`PathBuf`、`tokio::fs` | 同步为主；路径类型类似 `&str`/`String` | `████░░░░░░░░░░░░░░░░` 有需要时 | 理解概念 |
| 28 | [常见错误集锦](./common_mistakes.md) | 20 个高频坑：所有权、类型、async、错误处理 | 每条都有错误写法 vs 正确写法对比 | `██████░░░░░░░░░░░░░░` 遇到错误时 | 参考用 |

---

## 🧠 三大核心理念对比

### 1. 所有权系统（Rust 最大认知跨越）

```typescript
// TypeScript — GC 处理一切，不需要思考
let a = { name: "Alice" };
let b = a;  // 引用拷贝，两个变量都能用
```

```rust
// Rust — 每个值只有一个所有者
let a = String::from("hello");
let b = a;  // 所有权移动！a 不再有效
            // 要用 a 的话：let b = a.clone();
```

### 2. 表达式 vs 语句

```typescript
// TypeScript — if 是语句，三元是表达式
const label = x > 0 ? "正" : "负";
```

```rust
// Rust — if 是表达式，可以直接赋值
let label = if x > 0 { "正" } else { "负" };
```

### 3. 错误处理

```typescript
// TypeScript — try/catch 或可选链
try {
    const user = await findUser(id);
} catch (e) { console.error(e.message); }
```

```rust
// Rust — Result + ? 运算符，编译期强制处理
fn get_user(id: u32) -> Result<String, Error> {
    let user = find_user(id)?;  // 失败自动 return Err
    Ok(user)
}
```

---

## 📖 快速参考

### 运行示例

```bash
cargo run -p learning_notes --example rts_variables           # 变量
cargo run -p learning_notes --example rts_ownership_borrowing # 所有权
cargo run -p learning_notes --example rts_traits              # Trait
```

### 运行测试

```bash
cargo test -p learning_notes              # 运行所有测试
cargo test -p learning_notes -- --nocapture   # 显示 println! 输出
cargo test -p learning_notes -- --test-threads=1  # 单线程
```

---

## 🔗 文件间交叉引用

| 学习这个 | 建议同时参考 |
|---------|------------|
| [结构体](./structs.md) | [元组](./tuples.md)（元组结构体）、[泛型](./generics.md)（泛型结构体） |
| [枚举](./enums.md) | [Option 与 Result](./option_result.md)、[模式匹配](./pattern_matching.md) |
| [所有权与借用](./ownership_borrowing.md) | [生命周期](./lifetimes.md)、[智能指针](./smart_pointers.md) |
| [Trait](./traits.md) | [泛型](./generics.md)（trait bound）、[高级错误处理](./error_handling_advanced.md) |
| [字符串](./strings.md) | [所有权与借用](./ownership_borrowing.md)（String 的所有权） |
| [闭包与迭代器](./closures_iter.md) | [数组与 Vec](./arrays.md)、[函数](./functions.md) |
| [异步](./async_await.md) | [高级错误处理](./error_handling_advanced.md)（`?` 在 async 中）、[并发](./concurrency.md)（`spawn`） |
| [RAII 与 Drop](./raii_drop.md) | [智能指针](./smart_pointers.md)（MutexGuard、Box） |
| [并发与多线程](./concurrency.md) | [智能指针](./smart_pointers.md)（`Arc`/`Mutex`）、[异步](./async_await.md)（tokio 并发） |
| [包管理](./cargo.md) | [模块系统](./modules.md)（crate 结构） |

---

## ⚠️「Rust 独有」概念速查

以下概念在 TS 中 **完全没有对应**，需要全新的思维模型：

| 概念 | 难度 | 一句话说明 |
|------|:----:|----------|
| 所有权 | ⭐⭐⭐⭐ | 每个值只有一个所有者，离开作用域自动释放 |
| 借用 | ⭐⭐⭐⭐ | `&` 多读者 / `&mut` 一写者，不可共存 |
| 生命周期 | ⭐⭐⭐⭐⭐ | 编译期验证引用安全，零运行时开销 |
| 模式匹配穷举 | ⭐⭐⭐ | `match` 必须覆盖所有变体 |
| RAII + Drop | ⭐⭐⭐ | 离开作用域 = 自动释放资源 |
| 移动语义 | ⭐⭐⭐⭐ | 赋值 = 所有权转移，不是引用拷贝 |

---

> 💡 **自学提示**：如果某个概念看不懂，先跳过去学后面的。Rust 的很多概念是相互交织的，第二次路过时自然就理解了。
