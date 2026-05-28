# Rust vs TypeScript 对比学习

> 🎯 专为 **TypeScript 开发者**设计的 Rust 学习路径
>
> 每个文件都包含完整的 Rust 代码 + 对应的 TypeScript 注释版本，让你通过熟悉的 TS 语法理解 Rust 概念。

---

## 🗺️ 推荐学习顺序

按 **认知依赖关系** 排列，建议从上往下依次学习：

### 🟢 第一阶段：基础语法（从 TS 平滑过渡）

| # | 文件名 | 核心概念 | 与 TS 的差异 | 难度 |
|---|--------|---------|-------------|:----:|
| 1 | [`variables.rs`](./variables.rs) | 变量、可变性、Shadowing、常量 | `let` (默认不可变) vs `const/let` | ⭐ |
| 2 | [`primitives.rs`](./primitives.rs) | 整数/浮点/bool/char 类型 | `i32/u8/f64` vs 统一 `number` | ⭐ |
| 3 | [`functions.rs`](./functions.rs) | 函数、表达式、发散函数、泛型 | 表达式体 vs `return` | ⭐ |
| 4 | [`control_flow.rs`](./control_flow.rs) | if/loop/while/for/match | if 是表达式！loop 可返回值 | ⭐ |
| 5 | [`strings.rs`](./strings.rs) | `&str` vs `String` | 两种字符串类型 vs 一种 `string` | ⭐⭐ |

### 🟡 第二阶段：Rust 核心特性（认知飞跃区）

| # | 文件名 | 核心概念 | 与 TS 的差异 | 难度 |
|---|--------|---------|-------------|:----:|
| 6 | [`ownership_borrowing.rs`](./ownership_borrowing.rs) | 所有权、移动、借用、引用 | ⚠️ **Rust 独有**，TS 无对应 | ⭐⭐⭐ |
| 7 | [`structs.rs`](./structs.rs) | 结构体、方法、更新语法 | `struct` vs `class/interface` | ⭐⭐ |
| 8 | [`tuples.rs`](./tuples.rs) | 元组、解构、元组结构体 | `(T1,T2)` vs `[T1,T2]` 元组类型 | ⭐ |
| 9 | [`enums.rs`](./enums.rs) | 枚举、带数据枚举 | ⚠️ **Rust 王牌特性**，TS 判别联合的增强版 | ⭐⭐⭐ |
| 10 | [`pattern_matching.rs`](./pattern_matching.rs) | match、解构、守卫、@绑定 | `match` vs `switch`，穷举检查 | ⭐⭐ |

### 🟡 第三阶段：类型系统深入

| # | 文件名 | 核心概念 | 与 TS 的差异 | 难度 |
|---|--------|---------|-------------|:----:|
| 11 | [`option_result.rs`](./option_result.rs) | `Option<T>`、`Result<T,E>`、combinators | `Option` vs `T\|null`，`?` vs try/catch | ⭐⭐ |
| 12 | [`error_handling_advanced.rs`](./error_handling_advanced.rs) | 自定义错误、`From`、`?` 传播、anyhow | 枚举错误 vs `extends Error` | ⭐⭐⭐ |
| 13 | [`generics.rs`](./generics.rs) | 泛型函数/结构体/枚举、关联类型、const 泛型 | 单态化 vs 运行时擦除 | ⭐⭐ |
| 14 | [`traits.rs`](./traits.rs) | trait 定义、实现、trait 对象、标准 trait | `trait` vs `interface/abstract class` | ⭐⭐⭐ |

### 🔴 第四阶段：Rust 独有概念（TS 完全没有）

| # | 文件名 | 核心概念 | 与 TS 的差异 | 难度 |
|---|--------|---------|-------------|:----:|
| 15 | [`lifetimes.rs`](./lifetimes.rs) | 生命周期、`'a`、省略规则 | ⚠️ **Rust 独有**，编译器验证引用安全 | ⭐⭐⭐⭐ |
| 16 | [`raii_drop.rs`](./raii_drop.rs) | RAII、Drop trait、资源管理 | `drop()` vs `try/finally`、GC | ⭐⭐⭐ |
| 17 | [`smart_pointers.rs`](./smart_pointers.rs) | Box/Rc/RefCell/Arc/Mutex | 智能指针 vs GC | ⭐⭐⭐⭐ |

### 🟠 第五阶段：高阶特性

| # | 文件名 | 核心概念 | 与 TS 的差异 | 难度 |
|---|--------|---------|-------------|:----:|
| 18 | [`closures_iter.rs`](./closures_iter.rs) | 闭包、`Fn/FnMut/FnOnce`、迭代器链 | 三大闭包 trait vs 统一箭头函数 | ⭐⭐⭐ |
| 19 | [`arrays.rs`](./arrays.rs) | `[T;N]`、`Vec<T>`、切片、迭代器方法 | 固定数组 + Vec vs `Array<T>` | ⭐⭐ |
| 20 | [`hashmaps.rs`](./hashmaps.rs) | `HashMap`、`HashSet`、`BTreeMap`、entry API | `entry` API vs `Map.has/get/set` | ⭐⭐ |
| 21 | [`modules.rs`](./modules.rs) | 模块系统、可见性、`use`、重导出 | `mod` 树 vs `import/export` 文件模块 | ⭐⭐ |
| 22 | [`macros.rs`](./macros.rs) | 打印宏、断言、`dbg!`、自定义宏 | 编译期宏 vs 普通函数 | ⭐⭐⭐ |
| 23 | [`async_await.rs`](./async_await.rs) | 异步、Future、tokio、join!/spawn | ⚠️ 惰性 Future vs 立即执行 Promise | ⭐⭐⭐⭐ |
| 24 | [`testing.rs`](./testing.rs) | `#[test]`、`cargo test`、文档测试 | `#[test]` vs `describe/it/expect` | ⭐⭐ |

---

## 🧠 核心理念对比

### 所有权系统（Rust 独有，最大认知跨越）

```typescript
// TypeScript — GC 处理一切，不需要思考
let a = { name: "Alice" };
let b = a;  // 引用拷贝，两个变量都能用
```

```rust
// Rust — 每个值只有一个所有者
let a = String::from("hello");
let b = a;           // 所有权移动！a 不再有效
                     // 要用 a 的话：let b = a.clone();
```

### 表达式 vs 语句

```typescript
// TypeScript — if 是语句，三元是表达式
const label = x > 0 ? "正" : "负";
```

```rust
// Rust — if 是表达式，可以直接赋值
let label = if x > 0 { "正" } else { "负" };
```

### 错误处理

```typescript
// TypeScript — try/catch
try {
    const user = await findUser(id);
} catch (e) {
    console.error(e.message);
}
```

```rust
// Rust — Result + ? 运算符
fn get_user(id: u32) -> Result<String, Error> {
    let user = find_user(id)?;  // 失败自动返回 Err
    Ok(user)
}
```

---

## 📖 如何使用

### 运行单个示例

```bash
# 运行变量的例子
cargo run -p learning_notes --example rts_variables

# 运行所有权的例子
cargo run -p learning_notes --example rts_ownership_borrowing
```

### 运行全部示例（查看编译是否正确）

```bash
cargo test -p learning_notes  # 运行测试
cargo build -p learning_notes  # 编译确认所有示例无错误
```

每个文件第一行都注释了运行命令，可以直接复制。

---

## 🔗 文件间交叉引用

| 如果你在学这个 | 建议同时参考 |
|---------------|------------|
| `structs.rs` | `tuples.rs`（元组结构体）、`generics.rs`（泛型结构体） |
| `enums.rs` | `option_result.rs`（Option/Result 是枚举）、`pattern_matching.rs` |
| `ownership_borrowing.rs` | `lifetimes.rs`（借用的生命周期）、`smart_pointers.rs`（Rc/RefCell） |
| `traits.rs` | `generics.rs`（trait bound）、`error_handling_advanced.rs`（From/Display） |
| `strings.rs` | `ownership_borrowing.rs`（String 的所有权） |
| `closures_iter.rs` | `arrays.rs`（迭代器方法）、`functions.rs`（Fn trait） |
| `async_await.rs` | `error_handling_advanced.rs`（? 在 async 中的使用） |
| `raii_drop.rs` | `smart_pointers.rs`（MutexGuard、Box）、`ownership_borrowing.rs` |

---

## 🏷️ 「Rust 独有」标记

以下概念在 TS 中 **完全没有对应**，需要全新的思维模型：

| 概念 | 难度 | 说明 |
|------|:----:|------|
| 所有权（Ownership） | ⭐⭐⭐⭐ | 每个值只有一个所有者，离开作用域自动释放 |
| 借用（Borrowing） | ⭐⭐⭐⭐ | `&` 和 `&mut` 的严格规则（多读或一写） |
| 生命周期（Lifetimes） | ⭐⭐⭐⭐⭐ | 编译器验证引用安全，零运行时开销 |
| 模式匹配穷举性 | ⭐⭐⭐ | `match` 必须覆盖所有可能（`_` 通配符） |
| RAII + Drop | ⭐⭐⭐ | 资源获取即初始化，离开作用域自动释放 |
| 所有权三态 | ⭐⭐⭐⭐ | 移动（Move）/ 借用（&）/ 可变借用（&mut） |

---

> 💡 **自学者提示**：如果某个概念看不懂，先跳过去学后面的，Rust 的很多概念是相互交织的。第二次路过时自然就理解了。
