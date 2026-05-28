# Rust 特征对象 (Trait Objects)

## 1. 动态分发

```rust
trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}
```

### 1.1 Box<dyn Trait>

这里的参数 `x: Box<dyn Draw>` 是一个特征对象，允许在运行时处理不同的类型。由于 `Box<T>` 具有 Deref 特性，所以 `Box<dyn Draw>` 可以自动解引用调用 `draw()` 方法。由于实现了 Deref 特征，`Box` 智能指针会自动解引用为它所包裹的值，然后调用该值对应的类型上定义的 `draw` 方法。

```rust
fn draw1(x: Box<dyn Draw>) {
    x.draw();
}
```

### 1.2 &dyn Trait

这里的 `x: &dyn Draw` 是一个 Trait Object（特征对象），表示 `x` 只要实现了 `Draw` 特征就可以作为参数传递。

```rust
fn draw2(x: &dyn Draw) {
    x.draw();
}
```

### 1.3 使用示例

```rust
let x = 1.1f64;
let y = 8u8;

// 基于 x 的值创建一个 Box<f64> 类型的智能指针，数据放置在堆上
draw1(Box::new(x));
// 基于 y 的值创建一个 Box<u8> 类型的智能指针
draw1(Box::new(y));
draw2(&x);
draw2(&y);
```

`x` 和 `y` 的类型都实现了 `Draw` 特征，因为 `Box<T>` 可以在函数调用时隐式地被转换为特征对象 `Box<dyn Draw>`。

`dyn Trait` 使得代码可以在运行时处理不同的类型，即动态分发：

- `dyn Trait` 表示特征对象，允许在运行时决定调用哪个 `draw()` 方法。
- `Box<dyn Draw>` 可以存储不同的类型，但它们都实现了 `Draw`。
- `&dyn Draw` 允许在编译时未知确切类型的情况下调用 `draw()` 方法。

## 2. Self 与 self

在 Rust 中，有两个 `self`：一个指代当前的实例对象，一个指代特征或者方法类型的别名。

```rust
trait Draw {
    fn draw(&self) -> Self; // Self 指代实现类型
}

#[derive(Clone)]
struct Button;

impl Draw for Button {
    fn draw(&self) -> Self {
        return self.clone()
    }
}

let button = Button;
let newb = button.draw();
```

`button.draw()` 中的 `button` 是实例，`Self` 则指代的是 `Button` 类型。

## 3. 对象安全 (Object Safety)

并不是所有的特征都能作为特征对象，只有对象安全的特征才能用于 `dyn Trait`。

### 3.1 对象安全的两个条件

**条件一：方法的返回类型不能是 `Self`**

因为 `dyn Trait` 代表一个不确定的具体类型，如果方法返回 `Self`，编译器无法知道 `Self` 具体是什么类型。

**条件二：方法不能有泛型参数**

由于特征对象在运行时动态分派，而泛型是编译时静态分派，它们是不兼容的。

### 3.2 不满足对象安全的情况

```rust
trait NotObjectSafe {
    fn create() -> Self;              // 违反规则 1
    fn execute<T>(&self, value: T);   // 违反规则 2
}

fn run(obj: &dyn NotObjectSafe) { // 编译错误
    println!("Running...");
}
```

### 3.3 如何修复

移除 `Self` 返回值，避免泛型参数：

```rust
trait ObjectSafe {
    fn execute(&self); // 移除了泛型
}

fn run(obj: &dyn ObjectSafe) {
    obj.execute();
}

trait ObjectSafe {
    fn draw(&self) -> String;        // 返回值不是 Self
    fn print_message(&self, msg: &str); // 没有泛型参数
}
```

---

## TypeScript 对比

Rust `dyn Trait` 约等于 TS interface 的运行时多态。

**Rust：**

```rust
fn draw(x: &dyn Draw) { x.draw(); } // 显式动态分发
```

**TypeScript：**

```ts
function draw(x: Draw) { x.draw(); } // 默认就是动态
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 默认多态 | 编译期泛型（单态化，零开销） | 运行时动态 |
| 运行时多态 | 显式 `dyn Trait`（虚表开销） | 默认行为 |
| 对象安全 | 有规则限制 | 无限制 |
| 性能 | 静态分发零开销 | 全部动态 |

TS 代码天生就是动态分发（vtable 在 JS 引擎里）。Rust 需要你明确选择静态（`impl Trait`）还是动态（`dyn Trait`）。Rust 默认走静态——性能更好，但需要理解两种模式的区别。

详细对照 → [rust_vs_typescript.rs §11](../rust_vs_typescript.rs) "Trait 对象"
