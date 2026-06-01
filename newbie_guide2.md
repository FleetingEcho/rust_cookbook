# Rust 概念脉络图 — 从零到系统理解

---

## 一、Rust 的三大支柱（一条线，三个台阶）

所有权 → 借用 → 生命周期，**不是三个独立概念，是同一个问题的三个层次**。

```
问题：谁负责释放内存？

第1层 所有权
  └─ 每个值只有一个所有者，离开作用域自动释放
  └─ 转移（move） = 换了个所有者，原变量作废
  └─ 复制（Copy 类型） = 不受所有权限制，赋值就是复制

第2层 借用
  └─ 不转移所有权，临时借来用
  └─ &T = 只读借，随便借（任意多个读者）
  └─ &mut T = 改写借，只能有一个（不能同时读和写）
  └─ 借用必须过期后才能再用原变量

第3层 生命周期
  └─ 编译器需要确保：借用不会比被借的值活得更久
  └─ 'a = "这个东西至少要活这么久"
  └─ 大多数时候编译器自己推断（省略规则）
  └─ 需要手写 'a 的场景：函数返回引用 / 结构体存引用
```

**三句话串起来：**

> **谁拥有这个值？**（所有权）
> **别人能临时看看/改改吗？**（借用）
> **借用期间原东西不会丢吧？**（生命周期）

### 实战：这三个东西怎么一起出现

```rust
// 1. 只有所有权：简单，值传来传去
fn process(s: String) -> String { s }

// 2. 借用加入：加 &，不转移所有权
fn process(s: &str) -> &str { s }

// 3. 生命周期出现：函数返回引用时，编译器分不清引用从哪里来
fn longest(x: &str, y: &str) -> &str {  // ❌ 编译器不知道返回谁的引用
    if x.len() > y.len() { x } else { y }
}
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // ✅ 显式标注
    if x.len() > y.len() { x } else { y }
}
```

你不需要每次都想生命周期，但你需要知道：**当错误信息出现 "lifetime" 时，是编译器在确认"这个引用不会失效"**，不是玄学。

---

## 二、类型系统的三层抽象

Rust 的类型系统是**逐层约束**的：

```
最自由          T                 任何类型
                 ↓
加行为约束     T: Trait          必须实现某 trait
                 ↓
固定具体       ConcreteType      就是一个具体类型
```

### 泛型 ≠ 动态类型

虽然写法像 TS 的泛型，但 Rust 泛型是**编译期展开**的（单态化）：

```rust
fn max<T: PartialOrd>(a: T, b: T) -> T { if a > b { a } else { b } }

// 编译后实际上展开成：
fn max_i32(a: i32, b: i32) -> i32   { if a > b { a } else { b } }
fn max_f64(a: f64, b: f64) -> f64   { if a > b { a } else { b } }
fn max_str(a: &str, b: &str) -> &str { if a > b { a } else { b } }
```

好处：无运行时开销，跟手写具体类型一样快。
代价：编译出的二进制更大（每个具体类型一份代码），编译更慢。

### 泛型的三种用法对应三种场景

```rust
// 1. 泛型函数：一个算法服务多种类型
fn first<T>(v: Vec<T>) -> Option<T> { v.into_iter().next() }

// 2. 泛型结构体：一种容器存不同类型
struct Wrapper<T>(T);

// 3. 泛型 impl：只有某些特化才有某方法
impl<T: Display> Wrapper<T> {
    fn print(&self) { println!("{}", self.0); }
}
// T 没有 Display 时就没有 print 方法——这叫"条件方法"
```

### 三种"多态"对比

| 方式 | 分发时机 | 性能 | 灵活性 |
|------|---------|------|--------|
| 泛型（单态化） | 编译期 | 最优（可内联） | 有限（T 必须在编译期确定） |
| `dyn Trait` | 运行时（虚表） | 有间接调用开销 | 灵活（运行时决定类型） |
| enum | 编译期（match） | 最优 | 固定（变体编译期确定） |

```rust
// 泛型：编译期生成两份
fn process<T: Display>(t: T) { println!("{t}"); }

// dyn Trait：运行时查虚表
fn process(t: &dyn Display) { println!("{t}"); }

// enum：match 展开
fn process(t: Value) {
    match t { Value::Int(n) => ..., Value::Str(s) => ... }
}
```

---

## 三、Trait：行为契约的链条

Trait 不是"接口"，它是**一组行为的契约**。关键在于 trait 之间的**继承关系**：

```
Clone
  └─ Copy    (Clone 的超集，赋值=复制)

Drop
  └─ 不能同时实现 Copy（有 Drop 就不能 Copy）

Sized
  └─ ?Sized  （放宽限制，DST 用）

Iterator
  └─ IntoIterator  （能被 for 循环）
```

### 四大黄金 Trait 组合

**① 数据容器标配：`Debug + Clone + PartialEq`**

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }
// Debug → {:?} 能打印
// Clone → .clone() 能复制
// PartialEq → == 能比较
```

**② 错误类型标配：`Debug + Display + Error`**

```rust
#[derive(Debug)]
struct MyError(String);
impl Display for MyError { ... }       // 给用户看的错误信息
impl std::error::Error for MyError {}  // 可以向上传播
```

**③ 排序/比较标配：`PartialOrd + Ord + PartialEq + Eq`**

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Priority(u32);  // 可以直接 sort、min、max
```

**④ 哈希/集合标配：`Hash + Eq`**

```rust
#[derive(Hash, Eq, PartialEq)]
struct UserId(u64);  // 可以做 HashMap 的 key
```

---

## 四、错误处理：从 panic 到 thiserror 的进化链

Rust 的错误处理是一条**从原始到优雅的进化链**：

```
panic!　　　　　  →  不可恢复，最粗暴
   ↓
Option<T>　　　　→  可能没值，但不解释原因
   ↓
Result<T, E>　　→  可能失败，带错误信息
   ↓
? 运算符　　　　 →  自动向上传播错误，消除 match 嵌套
   ↓
自定义 Error 枚举 →  区分错误类型（网络错误/解析错误/权限错误）
   ↓
thiserror　　　　→  自动生成 Display + Error，减少模板代码
   ↓
anyhow　　　　　 →  不在乎具体错误类型，只关心"报错并向上传"
```

### 什么时候用哪个？

```rust
// 1. 简单的可能没值：用 Option
fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

// 2. 可能失败且调用方需要区分原因：用 Result + 自定义 error
fn parse_ticket(s: &str) -> Result<Ticket, TicketError> {
    // TicketError 可以是枚举 { EmptyTitle, InvalidStatus, ... }
}

// 3. 内部逻辑，不在乎具体错误类型：用 anyhow::Result
fn process_file(path: &str) -> anyhow::Result<()> {
    let content = std::fs::read_to_string(path)?;  // io::Error → anyhow::Error
    let n = content.parse::<i32>()?;               // ParseIntError → anyhow::Error
    Ok(())
}

// 4. 别人调用我的库：用自定义 error，返回 Result<T, MyError>
// 5. 我调用别人的库：用 anyhow，返回 anyhow::Result<T>
```

### ? 运算符的实质

```rust
// 写法一：match 手动处理
fn example() -> Result<i32, Error> {
    let n = match "42".parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e.into()),  // 手动转成函数返回的错误类型
    };
    Ok(n + 1)
}

// 写法二：? 自动做上面的事
fn example() -> Result<i32, Error> {
    let n = "42".parse::<i32>()?;  // 自动 Err → return Err(e.into())
    Ok(n + 1)
}
```

`?` 做了三件事：
1. 如果是 `Ok(v)` → 取出 `v` 继续
2. 如果是 `Err(e)` → `return Err(From::from(e))`（自动类型转换）
3. 可以在 `Option` 上用：`?` 对 None 也是 `return None`

---

## 五、容器与智能指针的谱系

### 容器类型的选用策略

```
数据存放策略
├── 固定个数？　　　　　→ 元组 (T, T, T) / 数组 [T; N]
├── 个数可变？
│   ├── 连续存放？　　 → Vec<T>
│   ├── 键值对？
│   │   ├── 需要排序？ → BTreeMap
│   │   └── 不需要排序？→ HashMap
│   └── 去重？　　　　 → HashSet / BTreeSet
└── 队列/双端？　　　　 → VecDeque
```

### 智能指针谱系：从简单到强大

```
所有权策略
├── 单一所有者
│   ├── 栈上　　　　 → T（默认）
│   ├── 堆上　　　　 → Box<T>（装箱，大小不确定时用）
│   └── 写时复制　　 → Cow<T>（读用引用，写才复制）
│
├── 共享所有权（多所有者）
│   ├── 单线程　　　 → Rc<T>（引用计数）
│   └── 多线程　　　 → Arc<T>（原子引用计数）
│
├── 内部可变性
│   ├── 单线程　　　 → RefCell<T>（运行时借用检查）
│   ├── 多线程互斥　 → Mutex<T>（阻塞锁）
│   └── 多线程读写　 → RwLock<T>（多个读/单个写）
│
└── 组合：共享 + 内部可变
    ├── 单线程　　　 → Rc<RefCell<T>>
    └── 多线程　　　 → Arc<Mutex<T>>
```

### 选型决策树

```rust
// 1. 就是一个值 → T
let x = 42;

// 2. 需要在堆上分配（递归类型 / 大对象 / trait 对象）→ Box<T>
let list: Box<ListNode> = Box::new(ListNode { val: 1, next: None });

// 3. 多个地方共同拥有 → 单线程用 Rc，多线程用 Arc
let shared = Rc::new("hello");
let a = Rc::clone(&shared);  // 引用计数 +1
let b = Rc::clone(&shared);

// 4. 需要"读取时共享、修改时独占"
//    单线程：RefCell
//    多线程：Mutex（同时只能一个读写）
//    多线程多读少写：RwLock

// 5. 经典模式：Rc<RefCell<T>> (单线程) / Arc<Mutex<T>> (多线程)
let shared = Rc::new(RefCell::new(42));
*shared.borrow_mut() += 1;          // 修改
println!("{}", shared.borrow());    // 读取
```

---

## 六、迭代器：Rust 的函数式编程骨架

迭代器是 Rust 里最常用的**组合子模式**。理解迭代器 = 理解 Rust 的数据处理方式。

```
数据源 → 迭代器（懒） → 适配器链（也懒） → 消费者（触发执行）
```

### 三条黄金路线

```rust
// 路线1：Vec → 变换 → 新 Vec
let result: Vec<i32> = vec![1,2,3]
    .into_iter()
    .filter(|x| x > 1)
    .map(|x| x * 10)
    .collect();

// 路线2：Option/Result → 变换 → 继续链式
let result: Option<i32> = Some(1)
    .map(|x| x + 1)
    .filter(|x| x > 0);

// 路线3：String → 拆分 → 处理 → 聚合
let sum: i32 = "1,2,3,4"
    .split(',')
    .filter_map(|s| s.parse::<i32>().ok())
    .sum();
```

### 消费 vs 适配

```
消费（触发迭代，收集结果）
├── .collect()      → 收集到容器（必须标注类型）
├── .sum()          → 求和
├── .count()        → 计数
├── .for_each()     → 遍历执行副作用
├── .reduce()       → 归约
├── .fold()         → 带初始值的归约
└── .all() / .any() → 短路判断

适配（返回新迭代器，不执行）
├── .map()          → 变换每个元素
├── .filter()       → 过滤
├── .take()         → 只取前 n 个
├── .skip()         → 跳过前 n 个
├── .flat_map()     → 每个元素展开成多个
├── .flatten()      → 嵌套结构展平
└── .chain()        → 拼接两个迭代器
```

**核心规律：** 适配器返回 `impl Iterator`（还是懒的），消费者返回最终值（触发实际计算）。

---

## 七、并发：从线程到 async 的演进

Rust 的并发模型是**递增复杂度**的，不要一上来就上 async：

```
单线程 → 多线程（scope）→ 多线程（channel）→ 多线程（共享状态）→ async
```

### 什么时候用什么

```rust
// 1. 计算密集型/CPU 并行 → thread::scope（最简单）
thread::scope(|s| {
    s.spawn(|| println!("子线程1"));
    s.spawn(|| println!("子线程2"));
});  // 所有子线程在这里结束

// 2. 任务间需要通信 → channel（mpsc）
let (tx, rx) = std::sync::mpsc::channel();
thread::spawn(move || { tx.send(42).unwrap(); });
println!("{}", rx.recv().unwrap());

// 3. 共享状态互斥 → Arc<Mutex<T>>
let data = Arc::new(Mutex::new(0));
let data2 = Arc::clone(&data);
thread::spawn(move || { *data2.lock().unwrap() = 1; });

// 4. IO 密集/大量并发连接 → async/tokio
#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async { /* IO 操作 */ });
    handle.await.unwrap();
}
```

### Send / Sync 的本质

```
Send:  能跨线程转移所有权
　　大多数类型默认 Send
　　例外：Rc<T>（引用计数非原子，不能跨线程）
　　修复：Rc → Arc

Sync:  能跨线程共享 &T
　　大多数类型默认 Sync
　　例外：RefCell<T>（运行时借用检查非线程安全）
　　修复：RefCell → Mutex / RwLock
```

**记住：** `Rc` 和 `RefCell` 是单线程的，`Arc` 和 `Mutex` 是多线程的版本。

---

## 八、async/await：协程模型

async 不是"更快的线程"，它是**协作式调度**——任务主动让出执行权（`.await`）。

### 关键差异

| | 线程 | async |
|--|------|-------|
| 调度 | OS 内核抢占式 | 运行时协作式（tokio） |
| 切换成本 | 微秒级（用户态→内核态） | 纳秒级（函数调用级别） |
| 栈 | 每个线程独立的栈（MB 级） | 所有任务共享栈 |
| 适合 | CPU 密集、阻塞 IO | IO 密集、大量连接 |
| Rust 实现 | std::thread | tokio / async-std |

### async 执行流程

```
async fn foo() { ... }  →  编译为 Future（状态机）
                              ↓
tokio::spawn(foo())     →  Future 被提交到运行时
                              ↓
foo().await             →  运行时 poll 这个 Future
                              ↓
                         遇到 IO 操作 → 注册回调 → yield
                              ↓
                         IO 完成 → 运行时唤醒 → 继续 poll
```

### 如何选择

```
CPU 计算 → thread
IO 等待 → async（用 tokio）
混合 → tokio::task::spawn_blocking 把 CPU 计算扔到线程池
```

---

## 九、Rust 的内存模型图

```
            栈（编译期确定大小）              堆（运行时分配）
            ┌─────────────────────┐      ┌──────────────────────┐
            │ i32: 4B             │      │ String: "hello"      │
            │ bool: 1B            │      │ Vec<i32>: [1,2,3]    │
            │                      │      │ Box<Trait>: vtable   │
            │ String 元数据 24B    │ ←──→ │ 堆上的字符串数据      │
            │  └ ptr | len | cap  │      │                      │
            │                      │      │                      │
            │ &T: 8B (引用=指针)   │      │                      │
            └─────────────────────┘      └──────────────────────┘
```

关键认知：
- **栈快、固定大小**：所有基本类型、引用、元组、数组都在栈上
- **堆慢、动态大小**：String、Vec、Box、Rc、Arc 等"容量可变"的数据在堆上
- **栈上只存引用（指针）**：String 的 24B 只是栈上的头，实际字符在堆上
- **引用（&T）永远是 8B（64 位）**：不管引用多大的数据

---

## 十、rustlings 之后的进阶路线

```
第一阶段（你现在的位置）
├── 能写简单 CRUD
├── 理解所有权
├── 会用 Option/Result 处理错误
└── 知道 trait 的概念

第二阶段：项目驱动
├── 写个 CLI 工具（clap + anyhow + serde）
├── 写个 HTTP 服务（axum + tokio + sqlx）
└── 写个简单的解析器（nom / pest）

第三阶段：深入理解
├── 生命周期高级（Variance / HRTB / GAT）
├── Pin / Unpin / 自引用类型
├── unsafe / FFI
├── 宏（声明宏 + 过程宏）
└── 异步运行时原理

第四阶段：安全与性能
├── Miri / loom（并发验证）
├── 基准测试（criterion）
├── 编译优化（PGO / LTO）
└── no_std / 嵌入式
```