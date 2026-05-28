# Rust Trait 高级用法

## 1. 关联类型 (Associated Types)

关联类型减少泛型参数的使用，提高可读性。关联类型必须在 `impl` 里具体化，而泛型可以保持灵活性。适用于每个实现都必须绑定特定类型的场景。

```rust
trait Container {
    type A;
    type B;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool;
}

struct NumberContainer {
    item1: i32,
    item2: i32,
}

// Container 不需要泛型参数，提升可读性
// type A = i32 明确规定了 A 和 B 的具体类型
impl Container for NumberContainer {
    type A = i32;
    type B = i32;

    fn contains(&self, a: &Self::A, b: &Self::B) -> bool {
        self.item1 == *a && self.item2 == *b
    }
}

fn difference<C: Container>(container: &C) -> i32 {
    42
}

let container = NumberContainer { item1: 10, item2: 20 };
println!("{}", container.contains(&10, &20)); // true
```

### 1.1 关联类型 vs 泛型

如果使用泛型，需要写很多泛型参数：

```rust
struct NumberContainer<T, U> {
    item1: T,
    item2: U,
}

impl<T: PartialEq, U: PartialEq> Container<T, U> for NumberContainer<T, U> {
    fn contains(&self, a: T, b: U) -> bool {
        self.item1 == a && self.item2 == b
    }
}
```

泛型适用于灵活的类型适配（如 `Container<A, B>`）。关联类型适用于特定的类型约束（如 `type A; type B;`）。如果 impl 需要绑定具体类型，关联类型比泛型更直观，提升可读性。

特别说明关联类型 vs 泛型：

```rust
trait Iterator { type Item; }       // 关联类型：每个 impl 一个 Item
trait Iterator<T> { ... }           // 泛型：每个 impl 可能有多个 T
```

- 关联类型 = "这个 trait 对于这个类型，Item 是固定的"
- 泛型 trait = "这个类型可以为不同的 T 实现多次"

## 2. 默认泛型类型

```rust
struct Container<T = String> {
    value: T,
}

let a = Container { value: "Hello".to_string() }; // 默认是 String
let b = Container::<i32> { value: 42 };           // 显式指定为 i32

println!("{}", a.value); // Hello
println!("{}", b.value); // 42
```

## 3. 调用同名的方法

当多个 trait 或类型自身提供了相同的方法名时：

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

let person = Human;
Pilot::fly(&person);   // 调用 Pilot 特征上的方法
Wizard::fly(&person);  // 调用 Wizard 特征上的方法
person.fly();          // 调用 Human 类型自身的方法
```

## 4. 完全限定语法

当以下情况发生时，可能会遇到方法调用的歧义：

- 多个特征提供了相同的方法名
- 特征方法与结构体的方法名称相同
- 特征方法与 impl 里的方法重名

在这些情况下，Rust 无法自动推导你想调用的具体方法，因此需要用完全限定语法来消除歧义。

```rust
trait A {
    fn hello(&self);
}

trait B {
    fn hello(&self);
}

struct MyStruct;

impl A for MyStruct {
    fn hello(&self) {
        println!("Hello from A!");
    }
}

impl B for MyStruct {
    fn hello(&self) {
        println!("Hello from B!");
    }
}

let obj = MyStruct;

// obj.hello(); // 编译错误：方法调用存在歧义

// 解决歧义：使用完全限定语法
<MyStruct as A>::hello(&obj); // Hello from A!
<MyStruct as B>::hello(&obj); // Hello from B!
```

### 4.1 结构体与 trait 方法重名

```rust
trait Greet {
    fn hello(&self);
}

struct Person;

impl Person {
    fn hello(&self) {
        println!("Hello from struct!");
    }
}

impl Greet for Person {
    fn hello(&self) {
        println!("Hello from trait!");
    }
}

let p = Person;

p.hello();                          // 默认调用结构体的方法：Hello from struct!
<Person as Greet>::hello(&p);       // 调用特征的方法：Hello from trait!
```

### 4.2 泛型中的完全限定语法

```rust
trait Speak {
    fn talk();
}

trait Shout {
    fn talk();
}

struct Dog;

impl Speak for Dog {
    fn talk() {
        println!("Dog says: Woof!");
    }
}

impl Shout for Dog {
    fn talk() {
        println!("Dog shouts: WOOF!");
    }
}

// 泛型约束
fn make_noise<T: Speak + Shout>() {
    // <T>::talk(); // Rust 无法推断调用哪个 talk()

    <T as Speak>::talk();  // 调用 Speak 版本
    <T as Shout>::talk();  // 调用 Shout 版本
}

make_noise::<Dog>(); // 输出 Woof! 和 WOOF!
```

## 5. 孤儿规则与 Newtype 模式

孤儿规则简单来说，就是特征或者类型必需至少有一个是本地的，才能在此类型上定义特征。

Newtype 模式的作用：

- 绕过孤儿规则，允许在 `MyString` 上实现 `Display`
- 防止与标准库冲突，避免对 `String` 进行不受控的修改

```rust
use std::fmt;

// 定义 Newtype 结构体，封装 String
struct MyString(String);

// 为 MyString 实现 Display 特征
impl fmt::Display for MyString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Custom String: {}", self.0)
    }
}

let s = MyString("Hello, world!".to_string());
println!("{}", s); // Custom String: Hello, world!
```

### 5.1 为什么需要 Newtype 模式

- 绕过孤儿规则，允许在外部类型上实现外部特征（如 `Display`）
- 限制访问，隐藏原始类型的方法，只暴露需要的方法
- 扩展功能，添加额外的方法，如 `shout()`
- 提高类型安全性，区分 `UserId(u32)` 和 `OrderId(u32)`

**如何使用：**

1. 定义 Newtype：`struct MyType(OriginalType);`
2. 实现外部特征：`impl Display for MyType { ... }`

---

## TypeScript 对比

进阶 trait 特性在 TS 中很难直接对应：

| Rust 特性 | 说明 | TS 对应 |
|-----------|------|---------|
| 关联类型 `type Item` | 每个 impl 指定一次 | 无直接等价 |
| 默认泛型参数 `T = i32` | 提供默认类型 | 泛型默认 `T = number` |
| 完全限定语法 | `<Type as Trait>::method()` | 无（无歧义问题） |
| supertrait | `trait A: B` 表示 A 需要 B | `interface A extends B` |
| newtype 模式 | 元组结构体包裹外部类型 | 继承/组合 |

详细对照 → [rust_vs_typescript.rs §10](../rust_vs_typescript.rs) "Trait"
