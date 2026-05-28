# Rc 和 Arc

Rust 的所有权机制规定，一个值只能有一个所有者，这样能避免很多错误。但在某些情况下，我们需要让多个地方共享一个数据，比如：

- **图结构**：多个边指向同一个节点，只有当所有边都不再指向它时，节点才应该被清理。
- **多线程**：多个线程需要访问同一数据，而 Rust 的安全机制不允许多个可变引用。

为了解决这些问题，Rust 提供了 Rc 和 Arc 这两种智能指针，它们通过引用计数的方式让多个所有者共享数据。

## Rc<T>

Rc（Reference Counting）适用于单线程，它会记录有多少地方在引用同一个数据，并在引用计数归零时释放数据。

### 示例

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello, world"));
    let b = Rc::clone(&a);

    println!("引用计数: {}", Rc::strong_count(&a)); // 输出 2
}
```

**解释：**

- `Rc::new` 创建一个智能指针，`a` 是第一个持有者。
- `Rc::clone(&a)` 不是深拷贝，而是复制指针，引用计数增加。
- `Rc::strong_count(&a)` 统计当前有多少个 Rc 智能指针在引用这个数据。

### 作用域变化

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("test ref counting"));
    println!("创建 a 之后: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("创建 b 之后: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("创建 c 之后: {}", Rc::strong_count(&a));
    }

    println!("c 作用域结束后: {}", Rc::strong_count(&a));
}
```

`c` 超出作用域后，引用计数减少。直到 `a` 和 `b` 都被释放后，数据才会真正被清理。

### Rc 不能修改数据

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("hello"));
    // *a.push_str(", world!"); // ❌ 错误，Rc 不能修改数据
}
```

如果想要修改数据，需要搭配 `RefCell<T>`。

## Arc<T>

Arc（Atomic Rc）是 Rc 的线程安全版本，它可以用于多线程环境，但比 Rc 慢，因为它使用了原子操作来保证线程安全。

### Rc 不能用于多线程

```rust
use std::rc::Rc;
use std::thread;

fn main() {
    let s = Rc::new(String::from("多线程漫游者"));

    for _ in 0..10 {
        let s = Rc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{}", s);
        });
    }
}
```

> ❌ 运行时报错！`Rc<T>` 不是线程安全的，因为它的计数器在多个线程中不能正确同步。

### 改用 Arc

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let s = Arc::new(String::from("多线程漫游者"));

    for _ in 0..10 {
        let s = Arc::clone(&s);
        let handle = thread::spawn(move || {
           println!("{}", s);
        });
        handle.join().unwrap();
    }
}
```

- `Arc::new` 创建一个线程安全的 `Arc<T>`。
- `Arc::clone` 复制指针，保证线程安全。
- `handle.join()` 确保所有线程都能完成执行。

## Rc 和 Arc 的区别

| 特性 | Rc<T> | Arc<T> |
|------|-------|--------|
| 是否线程安全 | ❌ 不是 | ✅ 是 |
| 适用环境 | 单线程 | 多线程 |
| 是否有性能损耗 | ✅ 更快 | ❌ 稍慢（需要原子操作） |

## 总结

- `Rc<T>` 适用于单线程，允许多个所有者共享数据，但不能修改数据。
- `Arc<T>` 适用于多线程，比 `Rc<T>` 慢，但安全。
- `Rc<T>` 和 `Arc<T>` 都是只读的，如果要修改数据，需要搭配 `RefCell<T>`（单线程）或 `Mutex<T>`（多线程）。
- 如果你的数据只在一个线程里共享，用 Rc 更高效；如果多个线程都要访问，用 Arc 才不会出错！

---

## 📘 TypeScript 对比

Rust 的 Rc/Arc ≈ TS 中完全不需要手动管理的东西。

```ts
// TS 中所有对象都是引用计数（垃圾回收器帮我们做）
let a = { value: 5 };
let b = a;  // 自动增加引用计数
```

```rust
// Rust 需要手动选择 Rc（单线程引用计数）
let a = Rc::new(5);
let b = Rc::clone(&a);  // 引用计数 +1
```

**关键区别：** Rust 没有 GC，

- 单线程共享用 `Rc<T>`
- 多线程共享用 `Arc<T>`（原子操作，稍慢）
- 想修改共享数据加 `RefCell<T>`（单线程）或 `Mutex<T>`（多线程）

TS 开发者用这个类比理解：`Rc` ≈ GC 的弱引用计数，`Arc` ≈ 线程安全版

详细对照 → [rust_vs_typescript.rs §16 "智能指针"](../rust_vs_typescript.rs)
