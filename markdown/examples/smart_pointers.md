# Rust 智能指针谱系：从简单到强大

> Rust 的所有权系统是其最核心的设计之一。智能指针则是在所有权规则之上，提供更灵活的内存管理策略的工具集。本文从"单一所有者"到"共享 + 内部可变"，逐级介绍各类智能指针的用途与选型逻辑。

---

## 一、全局谱系图

```
所有权策略
├── 单一所有者
│   ├── 栈上          → T（默认）
│   ├── 堆上          → Box<T>（装箱，大小不确定时用）
│   └── 写时复制      → Cow<T>（读用引用，写才复制）
│
├── 共享所有权（多所有者）
│   ├── 单线程        → Rc<T>（引用计数）
│   └── 多线程        → Arc<T>（原子引用计数）
│
├── 内部可变性
│   ├── 单线程 Copy   → Cell<T>（整存整取，零开销，不借出引用）
│   ├── 单线程任意    → RefCell<T>（运行时借用检查，能借出引用）
│   ├── 多线程互斥    → Mutex<T>（阻塞锁）
│   └── 多线程读写    → RwLock<T>（多个读 / 单个写）
│
└── 组合：共享 + 内部可变
    ├── 单线程        → Rc<RefCell<T>>
    └── 多线程        → Arc<Mutex<T>>
```

---

## 二、单一所有者

### 2.1 栈上的 `T` — 默认情况

最简单的情形：直接声明变量，值存在栈上，离开作用域自动释放。

```rust
fn main() {
    let x: i32 = 42;          // 存在栈上
    let s: String = String::from("hello"); // 堆上数据，但所有权唯一
    // s 和 x 在函数结束时自动 drop
}
```

**适用场景**：能在编译期确定大小、不需要共享的普通值。

---

### 2.2 堆上的 `Box<T>` — 装箱

`Box<T>` 在堆上分配空间，所有权仍是单一的。主要用于：
- **递归类型**（编译器无法确定大小）
- **大对象**（避免栈溢出）
- **Trait 对象**（动态分发）

```rust
// ✅ 用例 1：递归类型（链表节点）
enum List {
    Cons(i32, Box<List>),   // 没有 Box 编译器会报错：无限大小
    Nil,
}

fn build_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))))
}

// ✅ 用例 2：Trait 对象（动态分发）
trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog { fn speak(&self) { println!("汪！"); } }
impl Animal for Cat { fn speak(&self) { println!("喵～"); } }

fn make_animal(is_dog: bool) -> Box<dyn Animal> {
    if is_dog { Box::new(Dog) } else { Box::new(Cat) }
}

fn main() {
    let animal = make_animal(true);
    animal.speak(); // 汪！
}
```

---

### 2.3 写时复制 `Cow<T>` — 按需克隆

`Cow`（Clone on Write）在只读时持有引用，只有真正需要修改时才克隆数据。非常适合"大多数情况只读，偶尔需要修改"的场景。

```rust
use std::borrow::Cow;

fn ensure_uppercase(s: &str) -> Cow<str> {
    if s.chars().all(|c| c.is_uppercase()) {
        Cow::Borrowed(s)        // 已经是大写，直接借用，零拷贝
    } else {
        Cow::Owned(s.to_uppercase()) // 需要修改，才克隆
    }
}

fn main() {
    let a = ensure_uppercase("HELLO"); // Borrowed，无分配
    let b = ensure_uppercase("world"); // Owned，发生了一次分配
    println!("{} {}", a, b);           // HELLO WORLD
}
```

---

## 三、共享所有权（多所有者）

### 3.1 单线程 `Rc<T>` — 引用计数

`Rc`（Reference Counted）允许多个地方共同持有同一份数据。每次 `clone` 只是增加计数，不复制数据。当最后一个 `Rc` 离开作用域时，数据才被释放。

> ⚠️ `Rc<T>` **不能跨线程使用**，编译器会阻止你。

```rust
use std::rc::Rc;

fn main() {
    let data = Rc::new(vec![1, 2, 3]);

    let a = Rc::clone(&data);  // 引用计数: 2
    let b = Rc::clone(&data);  // 引用计数: 3

    println!("当前引用计数: {}", Rc::strong_count(&data)); // 3
    println!("a: {:?}", a);
    println!("b: {:?}", b);

    drop(a); // 引用计数: 2
    println!("drop a 后计数: {}", Rc::strong_count(&data)); // 2
}
// data 和 b 离开作用域，计数归零，vec 被释放
```

---

### 3.2 多线程 `Arc<T>` — 原子引用计数

`Arc`（Atomic Reference Counted）是线程安全版本的 `Rc`，使用原子操作更新计数，可以安全地在多线程间共享。

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data); // 每个线程持有一份引用
        thread::spawn(move || {
            println!("线程 {}: {:?}", i, data);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
```

---

## 四、内部可变性

Rust 默认"共享引用不可变"。内部可变性类型打破了这个限制，允许在 `&self` 下修改数据。单线程有两个选择：轻量的 `Cell<T>` 和功能更强的 `RefCell<T>`。

### 4.1 单线程 `Cell<T>` — 零开销整存整取

`Cell<T>` 是最轻量的内部可变性工具，只适合实现了 `Copy` 的类型（`i32`、`bool` 等）。它不借出引用，只能整个 `get` 出来、整个 `set` 回去，因此**永远不会 panic**，也没有运行时计数器开销。

```rust
use std::cell::Cell;

struct Counter {
    name: String,
    count: Cell<u32>, // 不需要把整个结构体声明为 mut
}

impl Counter {
    fn new(name: &str) -> Self {
        Counter { name: name.to_string(), count: Cell::new(0) }
    }

    fn increment(&self) { // &self，不是 &mut self
        self.count.set(self.count.get() + 1);
    }

    fn value(&self) -> u32 {
        self.count.get()
    }
}

fn main() {
    let c = Counter::new("点击数"); // 不需要 mut
    c.increment();
    c.increment();
    c.increment();
    println!("{}: {}", c.name, c.value()); // 点击数: 3
}
```

---

### 4.2 单线程 `RefCell<T>` — 运行时借用检查

允许在持有不可变引用的情况下修改内部数据。违反借用规则时会在**运行时 panic**，而不是编译期报错。

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(42);

    // 不可变借用
    {
        let r = data.borrow();
        println!("读取: {}", *r); // 42
    } // r 在这里释放

    // 可变借用
    {
        let mut w = data.borrow_mut();
        *w += 1;
    } // w 在这里释放

    println!("修改后: {}", data.borrow()); // 43

    // ❌ 下面的代码会在运行时 panic（同时持有可变和不可变借用）
    // let r = data.borrow();
    // let mut w = data.borrow_mut(); // panic!
}
```

---

### 4.3 多线程互斥 `Mutex<T>` — 阻塞锁

`Mutex`（互斥锁）保证同一时刻只有一个线程能访问数据。调用 `lock()` 会阻塞直到获得锁，使用完毕后自动释放。

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10).map(|_| {
        let counter = Arc::clone(&counter);
        thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // 阻塞等待锁
            *num += 1;
        }) // 锁在这里自动释放（num 离开作用域）
    }).collect();

    for h in handles { h.join().unwrap(); }

    println!("最终结果: {}", *counter.lock().unwrap()); // 10
}
```

---

### 4.4 多线程读写 `RwLock<T>` — 多读单写

`RwLock` 允许**多个线程同时读**，但写操作是独占的。读多写少时比 `Mutex` 效率更高。

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn main() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // 多个读线程并发执行
    let handles: Vec<_> = (0..3).map(|i| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            let r = data.read().unwrap(); // 多个读锁可以同时持有
            println!("读线程 {}: {:?}", i, *r);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    // 单个写线程独占
    {
        let mut w = data.write().unwrap(); // 等待所有读锁释放
        w.push(4);
    }

    println!("写入后: {:?}", *data.read().unwrap()); // [1, 2, 3, 4]
}
```

---

## 五、经典组合：共享 + 内部可变

仅有共享引用（`Rc`/`Arc`）时，数据是只读的。组合内部可变性类型，才能实现"多个地方共同拥有、并可修改"。

### 5.1 单线程：`Rc<RefCell<T>>`

```rust
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    children: Vec<Rc<RefCell<Node>>>,
}

fn main() {
    let root = Rc::new(RefCell::new(Node { value: 1, children: vec![] }));
    let child = Rc::new(RefCell::new(Node { value: 2, children: vec![] }));

    // 多个地方持有 child 的引用
    let child_ref = Rc::clone(&child);

    // 修改 root，将 child 加入其 children
    root.borrow_mut().children.push(Rc::clone(&child));

    // 通过另一个引用修改 child 的值
    child_ref.borrow_mut().value = 99;

    // root 的 children[0] 也看到了修改
    println!("child value: {}", root.borrow().children[0].borrow().value); // 99
}
```

---

### 5.2 多线程：`Arc<Mutex<T>>`

这是 Rust 多线程编程中最常见的模式，用于在多个线程间安全地共享并修改数据。

```rust
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

fn main() {
    // 多线程共享的可变 HashMap
    let map: Arc<Mutex<HashMap<&str, i32>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let keys = vec!["a", "b", "c"];

    let handles: Vec<_> = keys.into_iter().map(|key| {
        let map = Arc::clone(&map);
        thread::spawn(move || {
            let mut m = map.lock().unwrap();
            m.insert(key, 1);
        })
    }).collect();

    for h in handles { h.join().unwrap(); }

    println!("{:?}", *map.lock().unwrap());
    // {"a": 1, "b": 1, "c": 1}（顺序不定）
}
```

---

## 六、选型速查表

| 场景 | 推荐类型 |
|------|----------|
| 普通值，单一所有者 | `T` |
| 堆分配 / 递归类型 / Trait 对象 | `Box<T>` |
| 多数只读，偶尔写 | `Cow<T>` |
| 单线程，多个所有者，只读 | `Rc<T>` |
| 多线程，多个所有者，只读 | `Arc<T>` |
| 单线程，内部可变，Copy 类型（计数器、标志位） | `Cell<T>` |
| 单线程，内部可变，任意类型 | `RefCell<T>` |
| 多线程，互斥访问 | `Mutex<T>` |
| 多线程，读多写少 | `RwLock<T>` |
| 单线程，共享 + 可变 | `Rc<RefCell<T>>` |
| 多线程，共享 + 可变 | `Arc<Mutex<T>>` |

---

## 七、选型决策流程

```
需要在多个地方使用同一份数据？
├── 否 → 单一所有者
│        ├── 大小固定 → T（栈上）
│        └── 大小不定 / 递归 / Trait 对象 → Box<T>
│
└── 是 → 需要修改数据？
          ├── 否 → 只读共享
          │        ├── 单线程 → Rc<T>
          │        └── 多线程 → Arc<T>
          │
          └── 是 → 共享 + 可变
                   ├── 单线程 → Rc<RefCell<T>>
                   └── 多线程
                            ├── 读写均等 → Arc<Mutex<T>>
                            └── 读多写少 → Arc<RwLock<T>>

单独使用内部可变性（无需共享）？
├── Copy 类型（i32, bool...） → Cell<T>（零开销，不会 panic）
└── 任意类型                  → RefCell<T>（运行时检查，可能 panic）
```

---

> **一句话总结**：先考虑是否需要共享，再考虑是否需要修改，最后考虑是否跨线程。三个维度确定之后，选型就自然清晰了。

---

## 八、智能指针命名全解

搞清楚每个名字的来源，记忆和理解都会容易很多。

### 基础类型

| 缩写 | 全名 | 命名逻辑 |
|------|------|----------|
| `T` | 任意类型（Type） | 泛型占位符，约定俗成 |
| `Box<T>` | 堆箱子（Box） | 就是字面意思"盒子"——把值装进堆上的一个盒子里 |
| `Cow<T>` | 写时复制（Clone on Write） | 读时借用，写时才克隆，奶牛图标是 Rust 社区的梗 |

### 引用计数系列

| 缩写 | 全名 | 命名逻辑 |
|------|------|----------|
| `Rc<T>` | 引用计数（Reference Counted） | Reference = 引用，Counted = 被计数的 |
| `Arc<T>` | 原子引用计数（Atomically Reference Counted） | 在 `Rc` 前面加 Atomic（原子操作），保证线程安全 |

### 内部可变性系列

| 缩写 | 全名 | 命名逻辑 |
|------|------|----------|
| `Cell<T>` | 单元格（Cell） | 像电子表格里的一个"格子"——可以独立读写，不暴露内部引用 |
| `RefCell<T>` | 引用单元格（Reference Cell） | 在 `Cell` 基础上，加了"可以借出引用（Ref）"的能力 |
| `Ref<T>` | 不可变引用守卫（Reference Guard） | `borrow()` 的返回值，持有它就持有一把"不可变借用锁" |
| `RefMut<T>` | 可变引用守卫（Mutable Reference Guard） | `borrow_mut()` 的返回值，持有它就持有"可变借用锁" |

`Cell` 和 `RefCell` 的区别就藏在名字里：
- `Cell` — 只能整存整取，**不借出引用**
- `RefCell` — 能借出引用（**Ref**），所以叫 **Ref**Cell

### 锁系列

| 缩写 | 全名 | 命名逻辑 |
|------|------|----------|
| `Mutex<T>` | 互斥锁（Mutual Exclusion） | Mutual = 相互，Exclusion = 排斥——同时只有一个人能进 |
| `RwLock<T>` | 读写锁（Read-Write Lock） | 区分读（Read）和写（Write）两种锁，多读单写 |
| `MutexGuard<T>` | 互斥锁守卫 | `lock()` 的返回值，离开作用域时自动释放锁 |
| `RwLockReadGuard<T>` | 读锁守卫 | `read()` 的返回值 |
| `RwLockWriteGuard<T>` | 写锁守卫 | `write()` 的返回值 |

### "守卫（Guard）"模式

`Ref`、`RefMut`、`MutexGuard` 这些返回值都有一个共同身份——**RAII 守卫**。

**RAII** = Resource Acquisition Is Initialization（资源获取即初始化）：把资源的生命周期绑定到变量的作用域上，变量消失时资源自动归还，永远不会忘记释放。

```rust
{
    let guard = mutex.lock().unwrap(); // 加锁
    // ... 操作数据 ...
} // guard 在这里 drop → 自动解锁，不需要手动 unlock()
```

### 命名规律一览

| 前缀 / 关键词 | 含义 |
|--------------|------|
| **Atomic** | 线程安全版（`Arc` vs `Rc`） |
| **Ref** | 能借出引用（`RefCell` vs `Cell`） |
| **Mut** | 可变版本（`RefMut` vs `Ref`） |
| **Guard** | 离开作用域自动释放的"门卫" |
| **RW** | 区分读写（`RwLock` vs `Mutex`） |