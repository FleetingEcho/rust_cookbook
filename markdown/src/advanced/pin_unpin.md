# Pin 和 Unpin

## 1. Pin 和 Unpin 的概念

`Pin` 是 Rust 的 `std::pin::Pin` 模块提供的一种机制，用来确保一个对象在内存中不会被移动。它与 `Unpin` 标记特征配合使用。

- `Unpin` 是一个标记特征，默认所有类型都实现了它，意味着可以安全移动。
- 如果类型没有实现 `Unpin`（即 `!Unpin`），那么它被 `Pin` 包裹后就不能再被移动。

## 2. 为什么需要 Pin？

自引用结构体（self-referential struct）内部包含指向自身其他字段的指针。如果这样的结构体被移动，内部指针就会变成悬垂指针（dangling pointer）。

```rust
struct Test {
    a: String,
    b: *const String, // 指向 a
}
```

`Pin` 是一个结构体，它包裹一个指针，确保该指针指向的数据不会被移动。

## 3. Pin 解决自引用问题

### 3.1 直接使用 Pin 保护自引用类型

```rust
#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
}

impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
        }
    }

    fn init(&mut self) {
        let self_ref: *const String = &self.a;
        self.b = self_ref;
    }

    fn a(&self) -> &str {
        &self.a
    }

    fn b(&self) -> &String {
        assert!(!self.b.is_null(), "Test::b 未初始化");
        unsafe { &*(self.b) }
    }
}
```

### 3.2 移动后的问题

如果 `Test` 发生移动，`b` 仍然指向原来的地址，造成悬垂指针：

```rust
std::mem::swap(&mut test1, &mut test2);
// 导致 test2.b 仍然指向 test1 原来的 a。
```

## 4. 使用 Pin 解决自引用问题

### 4.1 让结构体变为 !Unpin

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned, // 使类型变为 `!Unpin`
}
```

### 4.2 Pin 保护 Test

```rust
impl Test {
    fn new(txt: &str) -> Self {
        Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    fn init(self: Pin<&mut Self>) {
        let self_ptr: *const String = &self.a;
        let this = unsafe { self.get_unchecked_mut() };
        this.b = self_ptr;
    }

    fn a(self: Pin<&Self>) -> &str {
        &self.get_ref().a
    }

    fn b(self: Pin<&Self>) -> &String {
        assert!(!self.b.is_null(), "Test::b 未初始化");
        unsafe { &*(self.b) }
    }
}
```

### 4.3 Pin 确保安全

```rust
fn main() {
    let mut test1 = Test::new("test1");
    let mut test1 = unsafe { Pin::new_unchecked(&mut test1) };
    Test::init(test1.as_mut());

    let mut test2 = Test::new("test2");
    let mut test2 = unsafe { Pin::new_unchecked(&mut test2) };
    Test::init(test2.as_mut());

    println!("a: {}, b: {}", Test::a(test1.as_ref()), Test::b(test1.as_ref()));
    std::mem::swap(test1.get_mut(), test2.get_mut()); // 这里会报错
}
```

`std::mem::swap` 失败，因为 `test1` 和 `test2` 已经被 `Pin` 保护，不能再被移动。

## 5. 固定到堆上

除了固定到栈上，我们也可以将 `!Unpin` 类型固定到堆上，使其生命周期内地址不会改变：

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

#[derive(Debug)]
struct Test {
    a: String,
    b: *const String,
    _marker: PhantomPinned,
}

impl Test {
    fn new(txt: &str) -> Pin<Box<Self>> {
        let t = Test {
            a: String::from(txt),
            b: std::ptr::null(),
            _marker: PhantomPinned,
        };
        let mut boxed = Box::pin(t);
        let self_ptr: *const String = &boxed.as_ref().a;
        unsafe { boxed.as_mut().get_unchecked_mut().b = self_ptr };

        boxed
    }
}

fn main() {
    let test1 = Test::new("test1");
    let test2 = Test::new("test2");

    println!("a: {}, b: {}", test1.as_ref().a(), test1.as_ref().b());
    println!("a: {}, b: {}", test2.as_ref().a(), test2.as_ref().b());
}
```

- 通过 `Box::pin(t)` 固定 `Test` 在堆上。
- `Pin<Box<Self>>` 保证 `Test` 绝不会被移动。

## 6. 使 Future 变为 Unpin

默认情况下，`async` 函数返回的 `Future` 是 `!Unpin`。但某些 API 需要 `Future: Unpin`，我们可以使用以下方法：

### 6.1 使用 Box::pin

```rust
use std::future::Future;

fn execute_unpin_future(x: impl Future<Output = ()> + Unpin) { /* ... */ }

let fut = async { /* ... */ };
let fut = Box::pin(fut);
execute_unpin_future(fut);
```

### 6.2 使用 pin_utils::pin_mut!

```rust
use pin_utils::pin_mut;

let fut = async { /* ... */ };
pin_mut!(fut);
execute_unpin_future(fut);
```

## 7. 总结

- `Pin` 主要用于防止某些类型被移动，保护自引用数据结构。
- 默认情况下，Rust 类型是 `Unpin`，可以安全移动。
- 实现 `!Unpin`（如 `PhantomPinned`）后，类型不能再被移动。
- 可以使用 `Pin<&mut T>` 或 `Pin<Box<T>>` 保护 `!Unpin` 类型。
- `async` 生成的 `Future` 默认是 `!Unpin`，但可以使用 `Box::pin` 变为 `Unpin`。

## 📘 TypeScript 对比

`Pin`/`Unpin` ≈ TS 中**完全不存在的概念**。

JavaScript 中所有对象都是通过指针访问的，移动对象只是复制引用，不存在"移动后指针失效"的问题。

**为什么 Rust 需要 Pin？**

1. Rust 允许结构体包含指向自身字段的指针（自引用类型）
2. 如果结构体被移动，内部指针变成悬垂指针
3. `Pin` 保证内存地址固定，防止移动
4. 主要用在 `async` `Future` 中（`Future` 内部可能引用自己的字段）

TS 不需要这个——所有操作都是 GC 托管的引用，对象地址对开发者是透明的。

详细对照 → `rust_vs_typescript.rs §18 "Pin/Unpin"`
