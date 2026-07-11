# Rust 知识串联：散点如何连成一张网

> 这份文档不教新知识。它做一件事：把你已经学过的散落知识点串起来，
> 让你看到它们其实是 **同一条公理的不同投影**。
> 读完的目标是「恍然大悟」：原来 unwrap、迭代器、闭包、方法调用、智能指针……
> 背后是同一套逻辑在反复出现。

---

## 0. 一切的源头：一条公理

Rust 所有设计都能推导自这一句话：

> **每个值在任意时刻，有且只有一个所有者；访问它只有三种方式。**

| 访问方式 | 写法 | 含义 | 你能做什么 |
|---------|------|------|-----------|
| 拿走 | `T` | 获得所有权（move） | 随便用，用完负责销毁 |
| 借来看 | `&T` | 共享借用 | 只能读，可以同时有很多个 |
| 借来改 | `&mut T` | 独占借用 | 能读能写，但同一时刻只能有一个 |

这三种方式，下文称为 **三元组 `(T, &T, &mut T)`**。

**恍然大悟点 #1**：Rust 里几乎每一组「看起来长得像、又有点不一样」的 API，
都是这个三元组在某个场景下的投影。后面每一章都在验证这句话。

---

## 1. 三元组无处不在（本文档最重要的一张表）

先看总表，后面章节逐个展开：

| 场景 | 拿走 `T` | 借来看 `&T` | 借来改 `&mut T` |
|------|----------|-------------|-----------------|
| 方法接收者 | `fn f(self)` | `fn f(&self)` | `fn f(&mut self)` |
| 迭代器 | `into_iter()` → `T` | `iter()` → `&T` | `iter_mut()` → `&mut T` |
| for 循环 | `for x in vec` | `for x in &vec` | `for x in &mut vec` |
| 闭包 trait | `FnOnce` | `Fn` | `FnMut` |
| 函数传参 | `fn f(v: Vec<T>)` | `fn f(v: &[T])` | `fn f(v: &mut Vec<T>)` |
| 模式匹配绑定 | `Some(x)` (move) | `Some(ref x)` / match on `&opt` | `Some(ref mut x)` |
| 解构赋值 | `let s = t.0` | `let s = &t.0` | `let s = &mut t.0` |

**同一个问题被问了七遍：「你要拿走它，还是借它？借了要不要改？」**

几个具体的对应关系值得单独说：

### 1.1 迭代器三兄弟 = 方法接收者三兄弟

```rust
let v = vec![1, 2, 3];

v.iter()       // 产出 &i32   —— 相当于每个元素被 &self 借走看
v.iter_mut()   // 产出 &mut i32 —— 相当于每个元素被 &mut self 借走改
v.into_iter()  // 产出 i32    —— 相当于每个元素被 self 拿走，v 从此没了
```

`for` 循环只是它们的语法糖：

```rust
for x in &v      // 等价于 for x in v.iter()
for x in &mut v  // 等价于 for x in v.iter_mut()
for x in v       // 等价于 for x in v.into_iter()，循环后 v 不可再用
```

所以「为什么 `for x in v` 之后 v 就用不了了」不是什么特殊规则——
它就是所有权被 move 进了循环，和 `let w = v;` 之后 v 用不了是同一件事。

### 1.2 闭包三兄弟 = 同一个问题问捕获的变量

闭包捕获环境变量时，编译器问的还是那个问题：「拿走还是借？借了改不改？」

```rust
let s = String::from("hi");

let f = || println!("{s}");        // 只读 → 按 &s 捕获   → 实现 Fn
let mut g = || s.push('!');        // 要改 → 按 &mut s 捕获 → 实现 FnMut
let h = move || drop(s);           // 要消耗 → 按 s 捕获    → 实现 FnOnce
```

对应关系（注意包含关系）：

| trait | 调用时接收者 | 能调几次 | 类比 |
|-------|-------------|----------|------|
| `FnOnce` | `self` | 一次 | 消耗所有权的方法 |
| `FnMut` | `&mut self` | 多次，需独占 | `&mut self` 方法 |
| `Fn` | `&self` | 多次，可并发 | `&self` 方法 |

`Fn` ⊂ `FnMut` ⊂ `FnOnce`：能只读调用的闭包当然也能以独占方式调用，
就像有 `&T` 的地方传 `&mut T` 降级使用一样自然。

**恍然大悟点 #2**：`FnOnce/FnMut/Fn` 不是三个新概念，
它们就是 `self / &mut self / &self` 穿了件闭包的外衣。

---

## 2. 「为什么数组方法这么多，调用还不一样」

这是两个问题，分开回答。

### 2.1 方法多：因为方法根本不在 Vec/array 上，而在切片 `[T]` 上

```
[T]  切片（unsized）：sort、iter、len、first、windows、chunks…… 绝大多数方法在这
 ↑ Deref                    ↑ unsized coercion
Vec<T>                   [T; N] 数组
（自己只有 push/pop/insert   （自己几乎没有方法，
  等改变长度的方法）           全部借来）
```

`Vec<T>` 实现了 `Deref<Target = [T]>`。当你写 `v.sort()` 而 `Vec` 上没有
`sort` 时，编译器自动解引用成 `[T]` 再找——找到了。数组 `[T; N]` 同理
（通过 unsized coercion 变成 `&[T]`）。

所以查文档的正确姿势：**想知道「数组/Vec 能干什么」，去看 `[T]` 的文档页。**

同一个模式再看一遍就通了：

```
str   字符串切片：len、trim、split、find、starts_with…… 方法都在这
 ↑ Deref
String（自己只有 push_str/push 等改变长度的方法）
```

**恍然大悟点 #3**：`String : str = Vec<T> : [T]`。完全平行的关系。
「拥有容量、可增长的堆分配容器」Deref 到「一段连续数据的视图」。
方法定义在视图上，所有者免费继承。

### 2.2 调用不一样：看签名里的接收者，行为可以直接预测

```rust
fn sort(&mut self)                 // 原地改，不返回 → 需要 mut 变量
fn sorted(self) -> …               // （itertools）消耗自己，返回新的
fn iter(&self) -> Iter<T>          // 只是借来看看
fn into_iter(self) -> IntoIter<T>  // into_ 前缀 = 消耗自己
fn to_vec(&self) -> Vec<T>         // to_ 前缀 = 借来看，克隆出新的
fn as_slice(&self) -> &[T]         // as_ 前缀 = 免费换个视角，零开销
```

命名约定和接收者是绑定的：

| 前缀 | 接收者 | 开销 | 例子 |
|------|--------|------|------|
| `as_` | `&self` | 零开销，换个类型视角 | `as_str`, `as_slice`, `as_ref` |
| `to_` | `&self` | 有开销，克隆/转换出新值 | `to_string`, `to_vec`, `to_owned` |
| `into_` | `self` | 拿走你，变成别的 | `into_iter`, `into_bytes`, `into_inner` |

**恍然大悟点 #4**：不用背哪个方法怎么调。看一眼签名的 `self` 形式
（或方法名前缀），就知道调用后原值还在不在、要不要 `mut`、有没有拷贝开销。
这又是三元组：`as_`=&T，`to_`=&T→新T，`into_`=T。

### 2.3 隐藏的润滑剂：自动引用/解引用

为什么 `v.len()` 和 `(&v).len()` 和 `(&&&v).len()` 都能编译？
方法调用时编译器会自动加 `&`/`&mut`/解引用，直到找到匹配的方法。
这就是为什么 Rust 里没有 `->` 运算符——`.` 已经把 C++ 里
`obj.f()` 和 `ptr->f()` 的区别抹平了。

---

## 3. 成对出现的类型家族：owned / borrowed

第 2 章的 `String/str`、`Vec<T>/[T]` 不是孤例，是一个贯穿标准库的模式：

| 拥有型（owned，可增长，在堆上） | 借用型（borrowed，只是视图） | 视图指什么 |
|------|------|------|
| `String` | `&str` | 一段 UTF-8 字节 |
| `Vec<T>` | `&[T]` | 一段连续的 T |
| `PathBuf` | `&Path` | 一个文件路径 |
| `OsString` | `&OsStr` | 一个系统字符串 |
| `CString` | `&CStr` | 一个 C 字符串 |
| `Box<[T]>` | `&[T]` | 同上，不可增长的拥有版 |

统一的规律：

- **函数参数用借用型**：`fn f(s: &str)` 比 `fn f(s: &String)` 好，
  因为 `&String` 能自动 Deref 成 `&str`，反之不行。`&[T]` 优于 `&Vec<T>` 同理。
- **结构体字段、返回值用拥有型**：视图不拥有数据，存起来就要标生命周期，通常不值得。
- 两者之间的桥梁是固定的一对 trait：`Deref`（owned → borrowed，免费）
  和 `ToOwned`（borrowed → owned，克隆）。

**恍然大悟点 #5**：`String` vs `&str` 的选择困难，和 `Vec<T>` vs `&[T]`、
`PathBuf` vs `&Path` 是同一道题。学会一对，六对全会。

---

## 4. struct / impl / trait：数据、行为、能力的三权分立

其他语言的 `class` 把三样东西焊死在一起。Rust 把它拆开了：

```
struct  →  只放数据（长什么样）
impl    →  只放行为（能做什么）
trait   →  只定能力（承诺做什么）——接口/契约
```

```rust
struct Circle { radius: f64 }          // 1. 数据

impl Circle {                           // 2. 固有行为（inherent impl）
    fn new(r: f64) -> Self { Circle { radius: r } }
    fn area(&self) -> f64 { PI * self.radius * self.radius }
}

trait Draw {                            // 3. 能力契约
    fn draw(&self);
}

impl Draw for Circle {                  // 4. 声明"Circle 具备 Draw 能力"
    fn draw(&self) { … }
}
```

拆开带来的推论，每一条都是你可能碰到过的「疑惑」：

### 4.1 「impl 可以写好几个？」——可以，行为是外挂的

数据定义只有一份，行为可以分散在多个 `impl` 块、甚至多个文件里。
所以标准库能对 `[T]` 写几百个方法，条件成立才生效：

```rust
impl<T: Ord> [T] { fn sort(&mut self) {…} }   // 只有 T 可比较，才有 sort
impl<T: Clone> [T] { fn to_vec(&self) -> Vec<T> {…} }  // 只有 T 可克隆，才有 to_vec
```

**这回答了「为什么有的 Vec 有 sort 有的没有」**：方法是否存在，
取决于元素类型满足哪些 trait bound。`Vec<f64>` 没有 `.sort()`
（f64 不是 Ord，因为 NaN），但有 `.sort_by(…)`。不是玄学，是条件外挂。

### 4.2 「trait 可以给别人的类型实现？」——可以，能力是后贴的

```rust
impl MyTrait for Vec<i32> { … }   // ✅ 我的 trait，贴在别人的类型上
impl Display for MyStruct { … }   // ✅ 别人的 trait，贴在我的类型上
impl Display for Vec<i32> { … }   // ❌ 孤儿规则：trait 和类型都是别人的
```

孤儿规则（orphan rule）：trait 和类型至少有一个得是你自己的。
**这直接解释了 newtype 模式为什么存在**：

```rust
struct Wrapper(Vec<i32>);          // 包一层，类型就是"我的"了
impl Display for Wrapper { … }     // ✅ 合法
```

你在 design_pattern/newtype.rs 里写过的东西，动机就在这。

### 4.3 泛型 + trait bound vs dyn Trait：同一个 trait 的两种用法

```rust
fn draw_all<T: Draw>(items: &[T])        // 静态分发：编译期为每种 T 生成一份代码
fn draw_all(items: &[Box<dyn Draw>])     // 动态分发：运行时查虚表
```

| | `impl Trait` / `<T: Trait>` | `dyn Trait` |
|---|---|---|
| 决议时机 | 编译期（单态化） | 运行时（虚表） |
| 性能 | 零开销，可内联 | 一次指针跳转 |
| 集合里混多种类型 | ❌ 一个 T 只能一种 | ✅ 这是它存在的理由 |
| 默认选哪个 | ✅ 默认用这个 | 需要异构集合/减少编译产物时 |

**恍然大悟点 #6**：`derive` 宏也不神秘了——`#[derive(Debug, Clone)]`
就是让编译器帮你写 `impl Debug for X` 和 `impl Clone for X` 的代码生成器。
数据(struct) 与能力(trait impl) 分离，才使得「自动帮你贴能力」成为可能。

---

## 5. Option / Result / unwrap：错误处理其实只有一张决策表

### 5.1 先看透本质：它们只是普通 enum

```rust
enum Option<T>    { Some(T), None }
enum Result<T, E> { Ok(T),   Err(E) }
```

没有任何编译器魔法（除了 `?`）。Rust 删掉了 null 和异常，
用「普通的枚举 + 你必须 match 才能拿到里面的值」替代。
所有 `unwrap/expect/map/and_then/...` 都只是预先帮你写好的 match。

### 5.2 unwrap 决策树（什么时候可以 unwrap）

```
拿到一个 Option / Result，问自己：

失败在这里可能发生吗？
├─ 逻辑上不可能（我有编译器看不到的不变量）
│    → expect("原因写清楚")           // 比 unwrap 好：炸了知道为什么
│    例：编译期写死的正则 Regex::new(r"\d+").expect("hardcoded regex")
│
├─ 可能，而且调用者应该处理
│    → 用 ? 向上传播                  // 库代码的默认答案
│
├─ 可能，但当前函数就能兜底
│    → unwrap_or / unwrap_or_else / unwrap_or_default / if let
│
└─ 这是 main / 测试 / 一次性脚本
     → unwrap 随便用                  // 炸了就炸了，正是你想要的
```

一句话版本：**库代码用 `?`，应用入口和测试随便 unwrap，
「不可能失败」用 expect 并写明为什么不可能。**

### 5.3 `?` 是怎么串起 thiserror 和 anyhow 的

`?` 展开后大致是：

```rust
match expr {
    Ok(v) => v,
    Err(e) => return Err(From::from(e)),   // ← 注意这个 From
}
```

关键在 `From::from(e)`：**`?` 会自动做一次错误类型转换**。
这就是整个错误处理生态的接缝：

- `thiserror` 的 `#[from]` 属性 = 帮你实现 `From<io::Error> for MyError`，
  于是 io 错误能被 `?` 自动转成你的错误类型；
- `anyhow::Error` 实现了 `From<E> for anyhow::Error`（对所有标准错误），
  于是任何错误都能 `?` 进 `anyhow::Result`。

**恍然大悟点 #7**：`?` + `From` + thiserror/anyhow 不是三个知识点，
是一个机制：`?` 在传播错误时顺手调用 `From` 换类型，
两个库只是从不同方向帮你把 `From` 写好了
（thiserror：精确的自定义错误，给库用；anyhow：万能袋子，给应用用）。

### 5.4 Option 和 Result 的方法是同一套

它们的组合子几乎一一对应，学一遍等于学两遍：

| 意图 | Option | Result |
|------|--------|--------|
| 变换里面的值 | `map` | `map` |
| 链式可能失败的操作 | `and_then` | `and_then` |
| 提供默认值 | `unwrap_or(_else)` | `unwrap_or(_else)` |
| 互相转换 | `ok_or(err)` → Result | `ok()` → Option |
| 过滤 | `filter` | — |
| 组合两个 | `zip` | — |

而且 `map/and_then/filter` 和迭代器的同名方法是同一个思想：
**「容器里可能有值，我描述对值的操作，容器结构由它自己维护」**。
Option 是「最多装一个元素的容器」——它甚至真的实现了 `IntoIterator`。

---

## 6. 智能指针：一张表回答「选哪个」

Box/Rc/RefCell/Arc/Mutex 看起来是五个孤立的类型，
其实是三个独立问题的排列组合：

```
问题 1：几个所有者？        一个 → Box      多个 → Rc / Arc
问题 2：跨线程吗？          不跨 → Rc       跨   → Arc
问题 3：共享时还要改吗？    不改 → 到此为止  要改 → 包一层 Cell/RefCell/Mutex/RwLock
```

| 需求 | 单线程 | 多线程 |
|------|--------|--------|
| 独占所有权，堆分配 | `Box<T>` | `Box<T>` |
| 共享所有权，只读 | `Rc<T>` | `Arc<T>` |
| 共享所有权，还要改 | `Rc<RefCell<T>>` | `Arc<Mutex<T>>` / `Arc<RwLock<T>>` |
| Copy 类型的轻量内部可变 | `Cell<T>` | `AtomicUsize` 等 |

**恍然大悟点 #8**：`Rc<RefCell<T>>` 和 `Arc<Mutex<T>>` 不是要背的固定搭配，
是两个正交答案的拼接——外层回答「谁拥有」，内层回答「怎么改」。
`Arc<Mutex<T>>` 就是 `Rc<RefCell<T>>` 的线程安全版，一一对应：
Rc→Arc（引用计数加原子），RefCell→Mutex（借用检查从编译期挪到运行期/加锁）。

### 6.1 内部可变性为什么存在

第 0 章说 `&T` 只能读——这其实是「默认规则」而不是物理定律。
`Cell/RefCell/Mutex` 是官方开的后门：**在 `&T` 后面提供受控的可变性**，
代价是把「同时只有一个写者」的检查从编译期挪到运行期
（RefCell 违规会 panic，Mutex 违规会等锁）。

规则没有消失，只是换了执法时间。这就是为什么 `RefCell` 的方法叫
`borrow()` / `borrow_mut()`——名字都在提醒你：这还是那套借用规则。

---

## 7. 转换 trait 一家人：什么时候用哪个

| trait | 签名核心 | 开销 | 什么时候用 |
|-------|---------|------|-----------|
| `Deref` | `&self → &Target` | 零 | 智能指针/owned→视图（String→str）。**别滥用来模拟继承** |
| `AsRef<U>` | `&self → &U` | 零 | 函数参数想同时收 `String`/`&str`/`Path`…：`fn f(p: impl AsRef<Path>)` |
| `From`/`Into` | `T → U`（消耗） | 看情况 | 万无一失的转换。**只实现 From，Into 自动免费获得** |
| `TryFrom` | `T → Result<U>` | 看情况 | 可能失败的转换（i64→u8） |
| `ToOwned` | `&U → T` | 克隆 | 视图→拥有（&str→String），Clone 的泛化版 |
| `FromStr` | `&str → Result<T>` | 解析 | 就是 `"42".parse::<i32>()` 背后的 trait |

串联点：

- 你天天写的 `.into()`、`?` 的错误转换、`"x".parse()`、
  `collect::<Vec<_>>()`（背后是 `FromIterator`）—— 全是「实现一个 trait，
  换来一片标准库/生态的配合」。**trait 是 Rust 生态的插座标准。**
- `impl AsRef<Path>` 解释了为什么 `File::open("a.txt")` 和
  `File::open(path_buf)` 都能编译——不是重载（Rust 没有重载），
  是泛型 + 转换 trait 在模拟重载。

---

## 8. 生命周期：不是新知识，是把第 0 章写在纸面上

很多人觉得生命周期是独立的一大难关。串联之后你会发现：
**生命周期没有增加任何新规则，它只是把「借用不能活过所有者」这条
第 0 章的规则，在函数签名里显式标注出来。**

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str
```

这行不是在「设置」什么，是在「承诺」：返回的引用不会活得比 x 和 y 更久。
编译器在函数内部检查你守约，在调用处检查调用者守约。仅此而已。

- 为什么大部分函数不用写？——省略规则（elision）：一个输入引用时，
  输出默认跟它同寿命；有 `&self` 时，输出默认跟 self 同寿命。
  规则覆盖了 90% 的情况，剩下 10% 编译器分不清才要你标。
- 为什么结构体存引用要标 `struct Foo<'a> { s: &'a str }`？——
  同一条规则：结构体实例不能活过它借来的数据。这也是第 3 章
  「字段用拥有型」建议的根源：存 String 就没这些事。

**恍然大悟点 #9**：`'a` 读作「某段借用存活的范围」。
所有生命周期难题，最后都化简为第 0 章那句话：借的东西不能比主人活得久。

---

## 9. 一切皆表达式 & 模式匹配无处不在

这两个特性平时不显眼,但它们是很多「语法为什么长这样」的答案。

### 9.1 一切皆表达式:块有值,所以很多语法「消失」了

Rust 里 `if`、`match`、`loop`、`{}` 块都是**表达式**——它们有值。
一个块的值 = 最后一个**不带分号**的表达式;分号的作用就是「扔掉这个值」。

```rust
let x = if cond { 1 } else { 2 };        // 所以不需要三元运算符 ?:
let y = match n { 0 => "zero", _ => "many" };
let z = loop { if ready() { break 42; } };  // break 可以带值!
fn double(n: i32) -> i32 { n * 2 }       // 函数尾表达式即返回值,所以少见 return
```

**恍然大悟点 #10**:「函数最后一行为什么不写分号」「为什么没有三元运算符」
「为什么 `let x = if …` 合法」是同一件事:块是表达式。
加了分号值就变成 `()`,这也是那个经典报错
`expected i32, found ()` 的最常见来源——你多打了一个分号。

### 9.2 模式匹配:let 本身就是模式,match 只是它的完全体

你可能以为模式匹配 = `match`。其实**每一个 `let`、每一个函数参数、
每一个 for 循环变量,都是模式**:

```rust
let (a, b) = (1, 2);                  // let 后面是模式,不(只)是变量名
let Point { x, y } = p;               // 解构 struct
fn dist(Point { x, y }: Point) -> f64 // 函数参数也是模式!
for (i, v) in vec.iter().enumerate()  // for 循环变量也是模式
```

那 `match` / `if let` 特殊在哪?就一个概念:**可反驳性(refutability)**。

| 模式可能匹配失败吗? | 能用在哪 | 例子 |
|---------------------|----------|------|
| 不可能失败(irrefutable) | `let`、函数参数、`for` | `let (a, b) = tuple;` |
| 可能失败(refutable) | `match`、`if let`、`while let`、`let else` | `Some(x)`、`Ok(v)` |

```rust
let Some(x) = opt;                    // ❌ 编译错:let 不接受可能失败的模式
if let Some(x) = opt { … }            // ✅ 失败就不进分支
let Some(x) = opt else { return };    // ✅ let else:失败就提前退出
while let Some(x) = stack.pop() { … } // ✅ 失败就结束循环
match opt { Some(x) => …, None => … } // ✅ 所有失败情况都必须写出来(穷尽性)
```

**恍然大悟点 #11**:`let` / `if let` / `let else` / `while let` / `match`
不是五个语法,是**同一个模式机制配上五种「匹配失败怎么办」的策略**:
不许失败 / 失败跳过 / 失败早退 / 失败停止 / 失败也得处理。

### 9.3 穷尽性:enum + match 为什么是黄金搭档

`match` 必须覆盖所有变体,漏一个编译不过。这把「状态处理是否完整」
从 code review 挪进了编译器——给 enum 加一个新变体,
所有没处理它的 match 全部报错,**编译器帮你找到所有要改的地方**。

第 5 章说 Option/Result 只是普通 enum——现在补全另一半:
它们能取代 null 和异常,靠的正是穷尽性。null 的问题从来不是「空」,
而是「你忘了检查空」;穷尽的 match 让「忘了」不可能编译通过。
(`_` 通配符会放弃这个保护,所以对自己的 enum 慎用 `_`。)

---

## 10. 迭代器:惰性、collect 的魔法与零开销

### 10.1 适配器是惰性的:链式调用只是在「造类型」

```rust
let iter = v.iter().map(|x| x * 2).filter(|x| x > &10);
// 到这里为止,一次计算都没发生!
```

`map` / `filter` 不计算任何东西,它们只是把迭代器**包一层**,
返回一个新类型。上面 `iter` 的真实类型是个洋葱:

```
Filter<Map<Iter<'_, i32>, {closure}>, {closure}>
```

只有**消费者**(consumer)出现,洋葱才被逐层拉动:

| 角色 | 例子 | 特征 |
|------|------|------|
| 适配器(懒) | `map` `filter` `take` `skip` `zip` `enumerate` `chain` | 返回新迭代器,不干活 |
| 消费者(干活) | `collect` `sum` `count` `fold` `for_each` `any` / `for` 循环 | 返回具体值,驱动整条链 |

**恍然大悟点 #12**:这和第二篇 Future 的惰性是同一个设计哲学——
`map(f)` 之于迭代器,恰如 `async fn` 之于执行:**都只是构造一个描述
「将要做什么」的值,等待某个驱动者(消费者 / executor)来拉动它**。
Rust 系统性地偏爱「先描述、后执行」,因为描述是零开销的纯数据。

惰性还有实际收益:`v.iter().map(expensive).find(|x| cond(x))`
找到第一个就停,后面的 `expensive` 根本不会执行——短路是免费的。

### 10.2 collect 的魔法:FromIterator 与 turbofish

为什么同一个 `collect()` 能变出 `Vec`、`String`、`HashMap`?
因为 collect 的签名是「谁实现了 `FromIterator`,我就能变成谁」:

```rust
fn collect<B: FromIterator<Self::Item>>(self) -> B
```

所以 collect 自己不知道要变成什么,**由你要的类型反推**——
这就是为什么它总要类型标注,两种写法等价:

```rust
let v: Vec<i32> = iter.collect();        // 让 let 告诉它
let v = iter.collect::<Vec<i32>>();      // turbofish ::<> 直接告诉它
let v = iter.collect::<Vec<_>>();        // 元素类型让编译器自己推
```

这是第 7 章「trait 是生态的插座标准」的又一实例:
`FromIterator` 之于 collect,恰如 `From` 之于 `?`、`FromStr` 之于 `parse`
——**方法是通用的,行为由目标类型的 trait 实现决定**。

collect 最惊艳的一招,是 `Result` 也实现了 `FromIterator`:

```rust
let nums: Result<Vec<i32>, _> = ["1", "2", "x"]
    .iter()
    .map(|s| s.parse::<i32>())           // 每个元素是 Result<i32, _>
    .collect();                           // Result<Vec<_>, _>:一错全错,短路!
```

一个 collect 完成「全部成功则收集,任一失败则带着第一个错误提前返回」
——这正是第 5 章 `?` 的迭代器版本。`Option` 同理。

### 10.3 零开销:整条链最后就是一个循环

惰性 + 单态化(第 4.3 章)意味着:`iter().map().filter().sum()`
编译后和手写 for 循环生成**几乎相同的机器码**,通常还更容易被向量化。
所以链式风格不是「优雅但慢的高级抽象」,放心用。

顺带串一个第 1 章的回声:自己的类型实现 `Iterator`(只需 `next` 一个方法)
或 `IntoIterator`,就白拿全部适配器 + `for` 循环支持——
又是「实现一个 trait,换来一片生态」。

---

## 11. 总图

```
                    ┌─────────────────────────────┐
                    │  公理：值有唯一所有者        │
                    │  访问三式：T / &T / &mut T   │
                    └──────────────┬──────────────┘
        ┌──────────────┬───────────┼────────────┬──────────────┐
        ▼              ▼           ▼            ▼              ▼
   方法接收者      迭代器三兄弟   闭包三兄弟    函数传参      模式匹配
  self/&/&mut   into/iter/mut  Once/Fn/Mut   T/&[T]/&mut   move/ref
        │
        │  方法定义在视图类型上（[T], str），Deref 免费继承
        ▼
  owned/borrowed 类型对：String/str, Vec/[T], PathBuf/Path …
        │
        │  行为(impl)与数据(struct)分离，能力(trait)按条件外挂
        ▼
  trait 系统：bound 静态分发 / dyn 动态分发 / 孤儿规则→newtype / derive
        │
        │  错误也是普通 enum + trait（From）串起 ? / thiserror / anyhow
        ▼
  Option/Result：unwrap 决策树，组合子与迭代器同思想
        │
        │  借用规则的运行期版本（内部可变性）+ 所有权的共享版本（引用计数）
        ▼
  智能指针组合表：Box / Rc·Arc / RefCell·Mutex
        │
        │  借用规则写进签名 = 生命周期标注
        ▼
  生命周期：借的东西不能比主人活得久
        │
        ├──  块即表达式（if/match/loop 有值）；let/参数/for 都是模式，
        │    match 等五种语法 = 同一模式机制 × 五种失败策略；穷尽性守护 enum
        ▼
  迭代器：适配器惰性造类型，消费者驱动执行（与 Future 同哲学），
  collect 由 FromIterator 反推目标（trait 插座标准再现），零开销
```

## 12. 自测：如果这些问题你能秒答，说明真的串起来了

1. `v.into_iter()` 之后还能用 `v` 吗？为什么和 `let w = v;` 是同一个问题？
2. 为什么 `Vec<f64>` 没有 `.sort()` 但有 `.sort_by()`？（提示：条件外挂）
3. `fn f(s: &str)` 为什么比 `fn f(s: &String)` 好？哪个 trait 在起作用？
4. 一个闭包实现 `Fn` 还是 `FnOnce`，由什么决定？和 `&self`/`self` 什么关系？
5. `?` 为什么能把 `io::Error` 自动变成你的 `AppError`？thiserror 帮你写了什么？
6. `Arc<Mutex<T>>` 的两层分别在回答什么问题？单线程版是什么？
7. 为什么给 `Vec<i32>` 实现 `Display` 会报错？标准解法叫什么模式？
8. `as_str` / `to_string` / `into_bytes` 三个前缀分别对应三元组的哪一位？
9. `RefCell::borrow_mut()` 在运行期检查的，是编译期哪条规则？
10. `struct Foo<'a>` 里的 `'a` 在承诺什么？
11. `let Some(x) = opt;` 为什么编译不过，而 `let (a, b) = t;` 可以？一个词回答。
12. 函数最后一行多打了一个分号，报错会是什么样子？为什么？
13. `v.iter().map(expensive)` 执行了几次 `expensive`？什么时候才会执行？
14. 为什么 `collect()` 总要你写类型标注？`Vec<Result<T,E>>` 和
    `Result<Vec<T>,E>` 都能 collect 出来，靠的是什么机制？

答不上来的题号，回到对应章节
（1 / 4.1 / 3 / 1.2 / 5.3 / 6 / 4.2 / 2.2 / 6.1 / 8 / 9.2 / 9.1 / 10.1 / 10.2）。
