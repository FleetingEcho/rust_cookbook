# Rust ↔ TypeScript 对照指南

本文件按主题分类，将每个 Rust 概念映射到 TypeScript 中的对应概念，
并标注关键差异。适合**从 TS 转学 Rust** 的开发者快速上手。

每个章节结构：
- Rust 概念一句话总结
- TS 对应概念（如果有）
- 关键差异
- 简短代码对照
- 关联的项目文件

## 目录

1. [变量与常量](#1-变量与常量)
2. [基本类型系统](#2-基本类型系统)
3. [字符串](#3-字符串)
4. [复合类型：元组、数组、结构体](#4-复合类型)
5. [枚举与模式匹配](#5-枚举与模式匹配)
6. [函数与方法](#6-函数与方法)
7. [所有权与借用](#7-所有权与借用)
8. [生命周期](#8-生命周期)
9. [泛型](#9-泛型)
10. [Trait（特征）](#10-trait-特征)
11. [Trait 对象与动态分发](#11-trait-对象与动态分发)
12. [错误处理：Option 与 Result](#12-错误处理)
13. [集合：Vec 与 HashMap](#13-集合)
14. [迭代器](#14-迭代器)
15. [闭包](#15-闭包)
16. [智能指针：Box / Rc / Arc / RefCell](#16-智能指针)
17. [Drop 与 Deref](#17-drop-与-deref)
18. [生命周期高级与 Pin/Unpin](#18-生命周期高级与-pinunpin)
19. [并发：线程与 Send/Sync](#19-并发)
20. [异步：async/await](#20-异步-asyncawait)
21. [宏编程](#21-宏编程)
22. [unsafe](#22-unsafe)
23. [模块与包管理](#23-模块与包管理)
24. [测试](#24-测试)
25. [常用派生宏](#25-常用派生宏)
26. [const 泛型](#26-const-泛型)
27. [属性标注](#27-属性标注)

---

## 1. 变量与常量

### Rust: `let` / `let mut` / `const`

```rust
let x = 5;           // 不可变绑定（默认）
let mut y = 10;      // 可变绑定
const MAX: u32 = 100; // 编译期常量
```

### TS 对应
```ts
const x = 5;          // TS 的 const 是"引用不可变"，但对象属性可变
let y = 10;           // TS 的 let 相当于 Rust 的 let mut
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 默认可变性 | 不可变 (`let`) | 可变 (`let`) |
| 常量 | `const MAX: u32 = 100` 编译期常量 | `const MAX = 100` 运行时常量 |
| 变量遮蔽 | ✅ 同名变量可重复 `let` | ❌ 同一作用域不可重复声明 |
| 类型标注 | 经常需要 | TS 通常能推断 |

### 关联文件
- `basics/variable.rs` — 变量遮蔽、解构赋值、const
- `base_type/basic.rs` — 基础类型

---

## 2. 基本类型系统

### Rust 基本类型
```rust
let a: i32 = -42;      // 有符号 32 位整数
let b: u8 = 255;       // 无符号 8 位整数
let c: f64 = 3.14;     // 浮点数（默认 f64）
let d: bool = true;    // 布尔
let e: char = '中';    // Unicode 字符（4 字节）
```

### TS 对应
```ts
let a: number = -42;    // TS 只有统一 number（IEEE 754 双精度）
let b: boolean = true;
let c: string = "中";   // TS 字符串是 UTF-16
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 整数类型 | i8/i16/i32/i64/isize + u8/u16/u32/u64/usize | 只有 `number` |
| 浮点类型 | f32 / f64 | 只有 `number` |
| 字符 | `char` (4 字节 Unicode) | `string` 长度为 1 |
| 布尔 | `bool` | `boolean` |
| 数字字面量分隔符 | `1_000_000` | ES2021+ 支持 `1_000_000` |
| 类型转换 | 必须显式 (`as`, `From`, `into`) | 隐式 + 显式混合 |

### 关联文件
- `base_type/basic.rs` — 数值类型
- `base_type/string_bool_unit.rs` — 字符串、布尔、单元类型
- `base_type/expression.rs` — 表达式
- `base_type/iteration.rs` — 循环

---

## 3. 字符串

### Rust 字符串类型
```rust
let s: String = String::from("hello");  // 堆分配，可变
let s2: &str = "hello";                 // 字符串切片（引用）
let s3: Box<str> = "hello".into();      // 堆上不可变 str
```

### TS 对应
```ts
let s: string = "hello";   // TS 只有 string（不可变、UTF-16）
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 默认编码 | UTF-8 | UTF-16 |
| 可变性 | `String` 可变 / `&str` 不可变 | 全部不可变 |
| 切片 | `&str` 是引用切片 | `.slice()` 返回新字符串 |
| 字符串拼接 | `push_str`, `format!`, `+` | `+`, 模板字面量 |
| 子串 | `&s[0..5]`（按字节，⚠️ 可能 panic） | `.substring()`, `.slice()` |

### 关联文件
- `base_type/string_str_difference.rs` — String vs &str vs Box\<str> 详细对比

---

## 4. 复合类型

### 元组 / 数组 / 结构体
```rust
let t: (i32, &str) = (1, "hello");          // 元组
let a: [i32; 3] = [1, 2, 3];                // 定长数组
let v: Vec<i32> = vec![1, 2, 3];            // 动态数组

struct User { name: String, age: u32 }       // 具名字段结构体
struct Point(i32, i32);                       // 元组结构体
```

### TS 对应
```ts
let t: [number, string] = [1, "hello"];       // Tuple
let a: number[] = [1, 2, 3];                  // 数组（动态）
class User { constructor(public name: string, public age: number) {} }
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 数组定长 | `[T; N]` 编译期定长 | 全部动态长度 |
| 结构体 vs class | `struct` — 只有数据，无继承 | `class` — 数据 + 方法 + 继承 |
| 方法定义 | 单独 `impl` 块 | class 内部定义 |
| 字段访问 | `user.name` | 相同 |
| 元组 | 一等类型，可解构 | 类似数组的类型标注 |
| 结构体更新语法 | `User { name: "a", ..old }` | `{ ...old, name: "a" }` |

### 关联文件
- `types/array.rs` — 数组
- `types/tuple.rs` — 元组
- `types/compound.rs` — 复合类型综合
- `structs_enums/structs.rs` — 结构体
- `basics/method.rs` — 方法

---

## 5. 枚举与模式匹配

### Rust 枚举 —— 比 TS 强大得多
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },   // 每个变体可以有不同的类型
    Write(String),
    ChangeColor(i32, i32, i32),
}

match msg {
    Message::Quit => println!("quit"),
    Message::Move { x, y } => println!("move to {x},{y}"),
    Message::Write(s) => println!("{s}"),
    Message::ChangeColor(r, g, b) => println!("{r},{g},{b}"),
}
```

### TS 对应
```ts
// TS 最接近的是 discriminated union
type Message =
  | { kind: 'Quit' }
  | { kind: 'Move'; x: number; y: number }
  | { kind: 'Write'; value: string }
  | { kind: 'ChangeColor'; r: number; g: number; b: number };

switch (msg.kind) {
  case 'Quit': ...
  case 'Move': ...
}
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 枚举能力 | 变体可携带任意类型数据 | 只能用 discriminated union 模拟 |
| 模式匹配 | `match` 必须穷尽所有分支 | `switch` 不强制穷尽 |
| 匹配返回值 | `match` 是表达式，有返回值 | `switch` 是语句 |
| 解构 | 枚举变体、元组、结构体都能解构 | 对象/数组解构 |
| `if let` | 简洁匹配单分支 | 无直接等价 |
| 匹配守卫 | `match x { n if n > 0 => ... }` | 无对应 |

### 关联文件
- `structs_enums/enums.rs` — 枚举基础
- `structs_enums/match_basics.rs` — match 基础
- `structs_enums/pattern_match.rs` — 模式匹配
- `structs_enums/all_pattern.rs` — 全部匹配模式
- `advanced/enum_int.rs` — 枚举与整数转换

---

## 6. 函数与方法

### Rust 函数
```rust
fn add(x: i32, y: i32) -> i32 { x + y }  // 最后一个表达式是返回值
```

### TS 对应
```ts
function add(x: number, y: number): number { return x + y; }
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 返回值 | 最后一个表达式，不用 `return`（除非提前返回） | 必须 `return` |
| 函数重载 | ❌ 不支持（用泛型或 trait） | ✅ 支持 |
| 默认参数 | ❌ 不支持 | ✅ 支持 |
| 方法定义 | `impl` 块 | class 内部 |

---

## 7. 所有权与借用（Rust 最独特的概念！）

### Rust 所有权三原则
1. 每个值在 Rust 中都有且只有一个所有者（owner）
2. 当所有者离开作用域，值被自动释放
3. 所有权可以移动（move）或借用（borrow）

```rust
let s1 = String::from("hello");
let s2 = s1;                // 所有权移动！s1 不再有效
// println!("{s1}");        // ❌ 编译错误

let s3 = s2.clone();        // 深拷贝（堆数据也复制）
println!("{s2}");           // ✅ s2 仍然有效
```

### TS 对应

**TypeScript 根本没有所有权概念！** JS/TS 全部使用垃圾回收（GC）：
- 对象赋值 `let b = a` 是**引用拷贝**，两个变量指向同一对象
- 没有"移动"语义，没有"借用"检查
- 内存由 V8 的 GC 自动回收

```ts
let a = { name: "hello" };
let b = a;          // b 和 a 指向同一个对象
a.name = "world";
console.log(b.name); // "world" — 有副作用！
```

### 借用规则
```rust
let s = String::from("hello");
let r1 = &s;          // 不可变借用（任意多个）
let r2 = &s;
// let r3 = &mut s;   // ❌ 不能同时有可变和不可变借用
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 内存管理 | 编译期所有权 + 借用检查 | 运行时 GC（V8） |
| 可变性控制 | 编译期强制：1 个 mut 或 N 个 immut | 运行时可随意修改 |
| 悬垂指针 | 编译期阻止 | GC 语言极少发生 |
| 数据竞争 | 编译期消除 | 靠开发者自己避免 |
| 学习曲线 | 陡峭，但安全性极高 | 平缓 |

### 关联文件
- `ownership/ownership.rs` — 所有权基础
- `learning_additions/ownership_borrowing.rs` — 进阶例子

---

## 8. 生命周期

### Rust 生命周期 —— TS 完全没有的概念
生命周期是 Rust **编译期**用来检查引用有效性的机制。

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

### TS 对应

**TypeScript 没有生命周期**。TS 的引用要么永远有效（直到 GC 回收），
要么因为闭包捕获而延长。不存在"引用比数据活得更久"的编译时检查。

```ts
// TS — 运行时才可能出问题
function longest(s1: string, s2: string): string {
  return s1.length > s2.length ? s1 : s2;
}
// 返回的是新字符串或原字符串，不存在悬垂引用
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 生命周期标注 | 手动或用 `'a` 标注引用关系 | 不需要 |
| `'static` | 引用存活整个程序 | `const` 字符串 |
| NLL | 借用作用域到"最后一次使用" | 不适用 |
| 悬垂引用 | 编译期阻止 | 不适用（GC 处理）|

### 理解技巧（从 TS 视角）
- 把生命周期 `'a` 想象成"这个引用至少在 'a 这段时间内是有效的"
- `<'a>` 声明就像是"我们有一个时间段 'a"；`&'a str` 说"这个引用在 'a 内有效"
- 大多数情况下 Rust 编译器会自动推导生命周期（生命周期消除规则）

### 关联文件
- `ownership/lifetime.rs` — 生命周期基础
- `advanced/lifetime.rs` — NLL、Reborrow、复杂生命周期

---

## 9. 泛型

### Rust 泛型
```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T { ... }
struct Point<T, U> { x: T, y: U }
```

### TS 对应
```ts
function largest<T extends Comparable & Copy>(list: T[]): T { ... }
interface Point<T, U> { x: T; y: U; }
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 泛型约束 | `T: TraitName`（trait bound） | `T extends Interface` |
| 特质边界 | 用 `+` 组合: `T: Display + Clone` | `extends A & B` |
| where 子句 | `where T: Display` 让签名更清晰 | 无等价语法 |
| 编译期单态化 | ✅ 每个类型生成独立代码（性能好） | 不适用（JS 运行时无类型）|
| const 泛型 | ✅ `const N: usize` | ❌ 不支持 |
| 泛型关联类型 | ✅ `type Output;` | ❌ 无 |

### 关联文件
- `types/generics.rs` — 泛型基础
- `learning_additions/const_generics.rs` — const 泛型

---

## 10. Trait（特征）

### Rust Trait ≈ TypeScript Interface + 部分抽象类功能
```rust
trait Summary {
    fn summarize(&self) -> String;
    fn author(&self) -> String { "unknown".to_string() }  // 默认实现
}
```

### TS 对应
```ts
interface Summary {
  summarize(): string;
  author?(): string;  // TS 可选方法，但无法提供默认实现
}
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 定义 | `trait` 关键字 | `interface` 关键字 |
| 默认实现 | ✅ trait 内可写默认方法 | ❌ interface 不能，需用抽象类 |
| 为外部类型实现 trait | ✅ 可为已有类型实现 trait | ✅ 可为已有类实现 interface（类型体操）|
| 方法重载 | ❌ 不支持同名不同参数 | ✅ 支持 |
| 关联类型 | ✅ `type Item;` | ❌ 无 |
| 运算符重载 | ✅ 通过 `std::ops::Add` 等 trait | ❌ 不支持 |
| 继承 | `trait A: B` 表示 A 要求 B | `interface A extends B` |
| 派生宏 | `#[derive(Debug, Clone)]` 自动实现 | 无（需手动写）|

### 关联文件
- `traits/basics.rs` — trait 基础
- `learning_additions/traits_generics.rs` — trait + 泛型

---

## 11. Trait 对象与动态分发

### Rust: `Box<dyn Trait>` / `&dyn Trait`
```rust
fn draw(x: &dyn Draw) { x.draw(); }   // 运行时多态
```

### TS 对应
```ts
function draw(x: Draw) { x.draw(); }  // TS 天然动态
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 默认多态方式 | 编译期泛型（单态化） | 运行时动态 |
| 运行时多态 | 显式 `dyn Trait`（动态分发） | 默认就是动态 |
| 性能 | 静态分发零开销；动态分发有虚表开销 | 全部动态 |
| 对象安全 | 只有对象安全的 trait 才能 `dyn` | 任何接口都行 |

### 关联文件
- `traits/trait_objects.rs` — 特征对象

---

## 12. 错误处理

### Rust: `Option<T>` / `Result<T, E>` + 组合子
```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 { Err("division by zero".into()) }
    else { Ok(a / b) }
}
// 链式调用
divide(10.0, 2.0)
    .map(|v| v * 2.0)
    .and_then(|v| if v > 0.0 { Ok(v) } else { Err("negative".into()) });
```

### TS 对应
```ts
// TS 没有内置的 Option/Result，通常用：
// 1. throw/catch（异常）
// 2. 返回 null/undefined
// 3. 返回 { data, error } 对象
// 4. 社区 Either/Result 类型（fp-ts 等）

function divide(a: number, b: number): { data: number } | { error: string } {
  if (b === 0) return { error: "division by zero" };
  return { data: a / b };
}
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 错误处理哲学 | 返回值（Result/Option），不抛异常 | 异常 throw 是主流 |
| 空值 | `Option<T>`（Some/None） | `null` / `undefined` |
| 强制处理 | ✅ `Result` 必须处理（`unused_must_use`） | ❌ 可忽略返回值 |
| `?` 运算符 | ✅ 自动传播错误 | ❌ 无（但 try/catch） |
| 异常安全性 | ❌ 无异常 | ⚠️ 异常可跳过清理代码 |
| 模式匹配 | `match` / `if let` | `if` / `switch` |

### 关联文件
- `errors/result_error.rs` — Result 基础
- `advanced/errors.rs` — 自定义错误、thiserror/anyhow
- `learning_additions/error_handling.rs` — 进阶
- `learning_additions/option_result_combinators.rs` — 组合子

---

## 13. 集合

### Rust: `Vec<T>` / `HashMap<K, V>`
```rust
let mut v = vec![1, 2, 3];
v.push(4);

use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("key", "value");
```

### TS 对应
```ts
let v: number[] = [1, 2, 3];
v.push(4);

let m = new Map<string, string>();
m.set("key", "value");
// 或普通对象：{ [key: string]: string }
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 动态数组 | `Vec<T>` | `Array<T>` |
| 切片 | `&[T]` 引用切片（不拥有数据） | `.slice()` 返回新数组 |
| HashMap | `std::collections::HashMap` | `Map` 或对象字面量 |
| 遍历 | `for item in &vec` 或迭代器 | `for...of` 或 `.forEach()` |
| 索引越界 | 运行时 panic！ | 返回 `undefined` |

### 关联文件
- `collections/vector.rs` — Vec
- `collections/hashmap.rs` — HashMap
- `learning_additions/collections_extra.rs` — 进阶

---

## 14. 迭代器

### Rust 迭代器（惰性 + 零开销抽象）
```rust
let v = vec![1, 2, 3, 4, 5];
let result: Vec<_> = v.iter()
    .filter(|x| *x % 2 == 0)
    .map(|x| x * 2)
    .collect();
```

### TS 对应
```ts
const v = [1, 2, 3, 4, 5];
const result = v
    .filter(x => x % 2 === 0)
    .map(x => x * 2);
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 惰性求值 | ✅ 不 `.collect()` 就不执行 | ❌ 数组方法即时执行 |
| 所有权 | 有 `iter()` / `into_iter()` / `iter_mut()` 三种 | 只有一种 |
| 消费 | `.collect()`, `.sum()`, `.for_each()` | `.reduce()`, `.forEach()` |
| 自定义迭代器 | ✅ 实现 `Iterator` trait | ✅ 实现 `[Symbol.iterator]()` |
| 性能 | 零开销抽象，编译期优化 | 每次创建中间数组 |

### 关联文件
- `advanced/iterator.rs` — 迭代器大全
- `learning_additions/iterators.rs` — 进阶

---

## 15. 闭包

### Rust 闭包
```rust
let add_one = |x: i32| x + 1;
let sum = |a, b| a + b;  // 类型自动推导
```

### TS 对应
```ts
const addOne = (x: number): number => x + 1;
const sum = (a: number, b: number) => a + b;
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 捕获方式 | 按所有权/借用（Fn/FnMut/FnOnce） | 按引用（闭包捕获的是引用）|
| 三种 Fn trait | `Fn`（只借用）/ `FnMut`（可变借用）/ `FnOnce`（消耗） | 只有一种 |
| `move` 关键字 | 强制闭包获取所有权 | 无，但 TS 箭头函数自动捕获 |
| 作为参数 | 泛型约束 `F: FnOnce() -> T` | 函数类型 `() => T` |

### 关联文件
- `advanced/closure.rs` — 闭包详解
- `rust_by_example/examples/closures.rs` — 更多例子

---

## 16. 智能指针

### Rust 智能指针家族
| 类型 | 用途 | TS 类比 |
|------|------|---------|
| `Box<T>` | 堆上分配数据 | `new` 对象（堆分配）|
| `Rc<T>` | 单线程引用计数共享 | 无直接对应 |
| `Arc<T>` | 多线程引用计数共享 | 无直接对应 |
| `RefCell<T>` | 运行时借用检查（内部可变性） | 无直接对应 |

### TS 没有引用计数和借用检查！

JS/TS 的垃圾回收器自动管理内存，开发者**不需要关心**数据在栈还是堆上。
Rust 需要手动选择 Box（堆）、Rc（共享所有权）等。

```rust
// Rc: 引用计数共享所有权
let a = Rc::new(5);
let b = Rc::clone(&a);  // 引用计数 +1
```

```ts
// TS: 所有对象都是引用计数的（由 GC 管理）
let a = { value: 5 };
let b = a;  // 同一对象，GC 跟踪
```

### 关联文件
- `advanced/smart_pointer.rs` — 智能指针总览
- `advanced/rc_arc.rs` — Rc / Arc
- `advanced/cell_refcell.rs` — Cell / RefCell

---

## 17. Drop 与 Deref

### Drop: 析构函数
```rust
struct Resource;
impl Drop for Resource {
    fn drop(&mut self) { println!("cleaning up"); }
}
```

### TS 对应
TS/JS 没有 RAII 和析构函数。资源清理需要手动 `try/finally` 或 `using`（ES2023）：
```ts
class Resource {
  [Symbol.dispose]() { console.log("cleaning up"); }
}
using r = new Resource();  // 块结束时自动调用 dispose
```

### Deref: 自动解引用
```rust
let b = Box::new(5);
let sum = *b + 1;  // `*b` 解引用拿到值
```

### TS 对应
TS 没有解引用操作符，也不需要——普通变量已经是值/引用透明。

### 关联文件
- `advanced/drop.rs` — Drop 详解
- `advanced/deref.rs` — Deref 详解

---

## 18. 生命周期高级与 Pin/Unpin

### Pin 和 Unpin
`Pin` 确保**内存位置不被移动**，主要用在自引用类型和 async Future 中。

```rust
let pinned = Box::pin(my_value);  // 内存固定，不能移动
```

### TS 对应
TypeScript 完全没有内存固定的概念。
JS 的所有对象都是通过指针访问，移动对象只是复制引用，不存在"移动后指针失效"问题。

### 关联文件
- `advanced/pin_unpin.rs` — Pin/Unpin 详解
- `advanced/self-referential.rs` — 自引用类型

---

## 19. 并发

### Rust 并发：线程 + Send/Sync
```rust
use std::thread;
let handle = thread::spawn(|| {
    println!("Hello from a thread!");
});
handle.join().unwrap();
```

### TS 对应
```ts
// 浏览器: Web Worker
const worker = new Worker('worker.js');
// Node.js: Worker Threads
const { Worker } = require('worker_threads');
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 线程模型 | 系统线程（1:1） | 单线程事件循环 + Worker |
| 共享内存 | ✅ `Arc<Mutex<T>>` | ❌ `SharedArrayBuffer` （受限）|
| 数据竞争 | 编译期通过 Send/Sync 防止 | 运行时可能出现 |
| 消息传递 | `std::sync::mpsc` 通道 | Worker postMessage |
| 内存安全性 | 编译期保证 | 不保证 |

### 关联文件
- `advanced/concurrency_with_threads.rs` — 线程基础
- `advanced/concurrency_2.rs~5.rs` — 进阶

---

## 20. 异步：async/await

### Rust async/await —— 与 TS 非常相似！
```rust
async fn fetch_data() -> String {
    let result = some_async_fn().await;
    result
}

// 需要运行时（tokio / async-std）
#[tokio::main]
async fn main() {
    let data = fetch_data().await;
}
```

### TS 对应
```ts
async function fetchData(): Promise<string> {
  const result = await someAsyncFn();
  return result;
}
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 运行时 | 需要显式选择（tokio, async-std） | 内置（事件循环） |
| 返回值 | `Future` trait | `Promise<T>` |
| 执行器 | block_on, tokio::spawn | JS 引擎内置 |
| 取消 | 通过 drop Future 实现 | 通过 AbortController |
| 惰性 | ✅ Future 不 poll 就不执行 | Promise 创建即执行 |

### 关联文件
- `advanced/async.rs` — async 入门
- `advanced/multi-futures-simultaneous.rs` — 多 Future 并发

---

## 21. 宏编程

### Rust 宏——比 TS 强大得多
```rust
macro_rules! say_hello {
    () => { println!("Hello!"); };
}

// 过程宏用 derive/attribute
#[derive(Debug)]
struct MyStruct;
```

### TS 对应
TypeScript 没有编译期宏系统！
- 装饰器（decorator）是最接近的，但功能弱很多
- TS 用函数/泛型复用代码，Rust 用宏在编译期生成代码

```ts
// TS 装饰器（只能用于类成员）
function log(target: any, key: string) {
  console.log(`called ${key}`);
}
```

### 常用宏
| Rust 宏 | 用途 | TS 对应 |
|---------|------|---------|
| `println!` / `format!` | 格式化输出 | 模板字面量 + console.log |
| `vec!` | 创建 Vec | 数组字面量 `[1,2,3]` |
| `#[derive(Debug)]` | 派生特征 | 无，需手动实现 toString |
| `panic!` | 运行时错误终止 | `throw new Error()` |

### 关联文件
- `advanced/macro.rs` — 宏详解

---

## 22. unsafe

### Rust unsafe —— TS 没有对应概念

Rust 的 unsafe 允许：
1. 解引用裸指针 `*const T`
2. 调用 unsafe 函数（如 FFI）
3. 实现不安全的 trait
4. 访问可变静态变量

### TS 对应
TypeScript 没有"unsafe"关键词。
- `any` 类型可以绕过类型检查，是 TS 中最接近 unsafe 的"逃生舱"
- `as any` / `as unknown as T` 相当于告诉编译器"别检查了，我知道我在做什么"
- 但 TS 没有任何内存安全问题（GC 语言的优势）

```ts
// TS 的 "unsafe" — 类型层面的逃生舱
const data: any = JSON.parse(jsonString);
const result: number = data.some.field as number;  // 运行时可能出问题
```

### 关联文件
- `advanced/unsafe_superpowers.rs` — unsafe 详解

---

## 23. 模块与包管理

### Rust 模块系统
```rust
mod my_module;         // 从 my_module.rs 加载
use crate::module::Item;
```

### TS 对应
```ts
import { Item } from './module';
export class MyClass {}
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 默认可见性 | 私有（`pub` 公开） | 导出才可见（`export`） |
| 文件系统映射 | 模块树对应文件树 | import 路径 |
| 包管理器 | Cargo（crates.io） | npm/yarn/pnpm |
| 工作空间 | Cargo workspace | npm workspaces / monorepo |
| 条件编译 | `#[cfg(feature = "...")]` | 无编译期条件 |

### 关联文件
- `package_module/crate.rs` — crate 详解
- `learning_additions/modules_and_testing.rs` — 模块 + 测试

---

## 24. 测试

### Rust 测试
```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```
运行: `cargo test`

### TS 对应
```ts
// Jest / Vitest
describe('math', () => {
  it('should add correctly', () => {
    expect(2 + 2).toBe(4);
  });
});
```

### 关键差异
| 维度 | Rust | TypeScript |
|------|------|-----------|
| 测试框架 | 内置（`#[test]`） | 第三方（Jest, Vitest, Mocha）|
| 文档测试 | ✅ `///` 中的代码可运行 | ❌ |
| 条件编译 | `#[cfg(test)]` 只编译测试 | 全部编译 |

### 关联文件
- `learning_additions/testing_advanced.rs` — 进阶测试

---

## 25. 常用派生宏

### Rust derive 宏
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }
```

### TS 对应
TS 没有自动派生。需手动实现：
```ts
class Point {
  constructor(public x: number, public y: number) {}
  // 手动实现
  clone(): Point { return new Point(this.x, this.y); }
  equals(other: Point): boolean { return this.x === other.x && this.y === other.y; }
}
```

### 常用派生
| derive | 作用 | TS 对应 |
|--------|------|---------|
| `Debug` | `{:?}` 格式化输出 | `toString()` 手动实现 |
| `Clone` | `.clone()` 深拷贝 | 展开运算符或手写 |
| `Copy` | 按位复制（栈上类型） | 不适用 |
| `PartialEq` / `Eq` | `==` 比较 | 手动 `equals()` |
| `Hash` | 哈希 | 无 |
| `Default` | `::default()` 默认值 | 构造函数默认参数 |

### 关联文件
- `learning_additions/derive_macros.rs` — 派生宏详解

---

## 26. const 泛型

### Rust const 泛型
```rust
fn display_array<T: std::fmt::Debug, const N: usize>(arr: [T; N]) {
    println!("{:?}", arr);
}
```

### TS 对应
TypeScript 4.0+ 有**模板字面量类型**和**const 类型参数**，但不同：
```ts
// TS 可以这样写（自 4.0 起）
function getLength<T extends readonly any[], N extends number>(
  arr: T & { readonly length: N }
): N {
  return arr.length as N;
}
```

Rust 的 const 泛型更强大，能在编译期计算数组大小等。

---

## 27. 属性标注

### Rust 属性（Attributes）
```rust
#![allow(dead_code)]          // 模块级别
#[derive(Debug)]              // 项级别
#[cfg(test)]                  // 条件编译
#[inline]                     // 内联提示
```

### TS 对应
TypeScript 中：
- `@deprecated` JSDoc 标签
- 装饰器 `@DecoratorName`（实验性）
- 没有条件编译、内联、允许 lint 等属性

### 关联文件
- `rust_by_example/examples/attribute.rs` — 属性详解
