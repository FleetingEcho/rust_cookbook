# Rust vs TypeScript：函数对比

> **运行命令**：`cargo run -p learning_notes --example rts_functions`

---

## TypeScript 参考版本

```ts
// 函数声明
function add(x: number, y: number): number { return x + y; }

// 箭头函数
const double = (x: number): number => x * 2;

// 默认参数（Rust 不支持）
function greet(name: string, greeting: string = "Hello"): string {
    return `${greeting}, ${name}!`;
}

// 可选参数（Rust 用 Option）
function findUser(id: number, includeDeleted?: boolean): User { ... }

// rest 参数（Rust 用 &[T]）
function sumAll(...numbers: number[]): number { ... }

// 函数类型
type MathOp = (a: number, b: number) => number;
const add: MathOp = (a, b) => a + b;

// never 类型
function throwError(msg: string): never { throw new Error(msg); }

// 重载（TS 特有，但 Rust 有更严格的替代）
function len(s: string): number;
function len(arr: number[]): number;
function len(x: any): number { return x.length; }
```

---

## 一、基本函数

**TS**: `function add(x: number, y: number): number { return x + y; }`

**关键差异**：Rust 用表达式（最后一个表达式不写 `return` 自动返回）。

```rust
// 带 return 的风格（与 TS 最接近）
fn add_return(x: i32, y: i32) -> i32 {
    return x + y;
}

// Rust 惯用风格：最后一个表达式就是返回值（无分号，无 return）
// TS 箭头函数也有类似写法: const add = (x: number, y: number) => x + y;
fn add(x: i32, y: i32) -> i32 {
    x + y   // 不加分号，这就是返回值！(TS: return x + y)
}

// 表达式体函数（单表达式时甚至可以省略 {}）
fn add_short(x: i32, y: i32) -> i32 { x + y }
```

### 对比速查

| TypeScript | Rust |
|---|---|
| `function f(x: number): number { return x * 2; }` | `fn f(x: i32) -> i32 { x * 2 }`（无 `return`，无分号） |
| `const f = (x: number) => x * 2;` | `fn f(x: i32) -> i32 { x * 2 }` |

---

## 二、无返回值函数（void / ()）

**TS**: `function log(msg: string): void { console.log(msg); }`

**Rust**: 返回 `()`，称为「单元类型」，可以省略 `-> ()`。

```rust
// 显式返回单元类型 (TS: void)
fn log_message(msg: &str) -> () {
    println!("{msg}");
}

// 省略返回类型，等价于 -> () (TS: 省略也返回 void)
fn log_message_short(msg: &str) {
    println!("{msg}");
}
```

---

## 三、参数模式：类型注解

| 特性 | TypeScript | Rust |
|---|---|---|
| 类型注解 | `function f(x, y)` 可以不写类型（但建议写） | 每个参数**必须**标注类型 |

```rust
fn print_coord(x: i32, y: i32) {
    println!("坐标: ({x}, {y})");
}
```

---

## 四、TS 默认参数 vs Rust 惯用模式

**TS**: `function greet(name: string, greeting = "Hello"): string`

**Rust** 不支持默认参数，但有常用替代方案：

### 方案 A：Option 参数

```rust
fn greet(name: &str, greeting: Option<&str>) -> String {
    let g = greeting.unwrap_or("Hello");  // TS: greeting ?? "Hello"
    format!("{g}, {name}!")
}
```

### 方案 B：便捷包装函数

```rust
fn greet_default(name: &str) -> String {
    greet(name, None)  // 内部调用完整版本
}
```

### 方案 C：Builder 模式

适用于参数很多的场景。（详见 `structs.rs` 中的结构体更新语法）

---

## 五、TS rest 参数 vs Rust 切片参数

**TS**: `function sumAll(...numbers: number[]): number`

**Rust**: 使用 `&[T]` 切片参数。

```rust
fn sum_all(numbers: &[i32]) -> i32 {
    numbers.iter().sum()  // TS: numbers.reduce((a, b) => a + b, 0)
}
```

---

## 六、发散函数（never）

**TS**: `function throwError(msg: string): never { throw new Error(msg); }`

**Rust**: `-> !` 表示函数永不返回正常值。

```rust
fn panic_if_negative(n: i32) -> ! {
    panic!("数值不能为负: {n}")  // TS: throw new Error(...)
}
```

---

## 七、函数指针类型（fn 类型）

**TS**: `type MathOp = (a: number, b: number) => number;`

**Rust**: `fn(i32, i32) -> i32`（注意是小写 `fn`）。

```rust
fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))  // TS: f(f(x))
}

fn square(x: i32) -> i32 { x * x }
fn double(x: i32) -> i32 { x * 2 }

// 函数名自动转为函数指针
println!("do_twice(square, 3): {}", do_twice(square, 3));   // 81
println!("do_twice(double, 3): {}", do_twice(double, 3));   // 12

// 也可以传入闭包（如果闭包不捕获变量）
println!("do_twice(|x| x+1, 5): {}", do_twice(|x| x + 1, 5)); // 7
```

> **`fn` 类型 vs `Fn` trait**：`fn` 是函数指针，不能捕获变量；`Fn`/`FnMut`/`FnOnce` 是闭包 trait，可以捕获变量。（详见 `closures_iter.rs`）

---

## 八、方法：&self / &mut self / self

**TS**: `class` 方法用 `this`，默认可变。

**Rust**: 需要显式声明 `self` 的所有权方式。

```rust
struct Counter {
    value: i32,
}

impl Counter {
    fn new(value: i32) -> Self { // 关联函数，类似 TS 的 static 方法/constructor
        Counter { value }
    }

    // &self：不可变借用，只读 (TS: 普通方法，能读 this)
    fn get(&self) -> i32 {
        self.value
    }

    // &mut self：可变借用，可修改 (TS: 普通方法，能改 this)
    fn increment(&mut self) {
        self.value += 1;  // TS: this.value++
    }

    // self：消耗所有权 (TS 没有对应，但可以用 return this 链式调用)
    fn into_display(self) -> String {
        format!("Counter: {}", self.value)
        // self 在此被消耗，调用后不能再使用
    }
}
```

---

## 九、泛型函数

**TS**: `function identity<T>(x: T): T { return x; }`

```rust
fn identity<T>(x: T) -> T {
    x  // TS: return x
}

fn first<T>(list: &[T]) -> Option<&T> {
    list.first()  // TS: arr.length > 0 ? arr[0] : undefined
}

fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);  // TS: [a, b] = [b, a]（解构交换）
}
```

---

## 十、impl Trait（返回位置的高级特性）

**TS 没有直接对应**（TS 用抽象类或接口）。

```rust
fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // 返回一个闭包，但调用者不需要知道闭包的具体类型
    move |x| x + n
}

// impl Trait 也可以用于参数位置（语法糖）
fn print_displayable(value: impl Display) {
    println!("值: {value}");
}
```

---

## 十一、高阶函数（函数作为参数和返回值）

**TS**: 高阶函数非常常见，Rust 也一样。

```rust
// 接收函数
fn apply_twice<F>(f: F, x: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    f(f(x)) // TS: f(f(x))
}

// 返回函数（工厂函数）
fn make_multiplier(n: i32) -> impl Fn(i32) -> i32 {
    move |x| x * n
}
```

---

## 十二、TS 函数重载 vs Rust 替代方案

**TS**:
```ts
function len(s: string): number;
function len(arr: number[]): number;
```

**Rust** 不支持同名不同参的重载，但可以用 trait 或枚举。

### 方案 A：trait（静态分发，零开销）

```rust
trait Length {
    fn length(&self) -> usize;
}

impl Length for String {
    fn length(&self) -> usize { self.len() }
}

impl<T> Length for Vec<T> {
    fn length(&self) -> usize { self.len() }
}

fn print_len<T: Length>(item: &T) {
    println!("长度: {}", item.length());
}
```

### 方案 B：枚举（运行时分发，类似 TS union type）

```rust
enum Input<'a> {
    Text(&'a str),
    Numbers(&'a [i32]),
}

fn len_input(input: &Input) -> usize {
    match input {
        Input::Text(s)    => s.len(),
        Input::Numbers(v) => v.len(),
    }
}
```

---

## 十三、内嵌函数（TS 不支持函数内的函数）

**TS**: 没有嵌套函数，但可以用 `const inner = () => ...`

**Rust**: 函数内可以定义函数。

```rust
fn outer(x: i32) -> i32 {
    fn inner(y: i32) -> i32 {
        y * 2
    }
    inner(x) + 1
}
```

> **注意**：内嵌函数不能捕获外部变量（要用闭包）。
> ```rust
> let factor = 3_i32;
> // fn cant_capture(y: i32) -> i32 { y * factor } // ❌ 编译错误
> let can_capture = |y: i32| y * factor;             // ✅ 闭包才能捕获
> ```

---

## 十四、条件编译函数（TS 没有对应，预处理指令）

```rust
#[cfg(target_os = "linux")]
fn platform_specific() {
    println!("运行在 Linux");
}

#[cfg(not(target_os = "linux"))]
fn platform_specific() {
    println!("运行在非 Linux 系统");
}
```

---

## 总结对照表

| TypeScript | Rust |
|---|---|
| `function add(a,b) {...}` | `fn add(a: i32, b: i32) -> i32` |
| `return x + y` | `x + y`（无分号，表达式返回） |
| 参数不强制写类型 | 每个参数必须有类型注解 |
| `void` | `()` 单元类型 / 省略返回 |
| `never`（throw） | `-> !`（发散函数） |
| `type Fn = (i32) => i32` | `fn(i32) -> i32`（函数指针） |
| 默认参数 / 可选参数 | `Option<T>` / Builder 模式 |
| `...rest` 参数 | `&[T]` 切片参数 |
| 函数重载 | trait / 枚举替代 |
| `static method` | 关联函数（`impl` 内的 `fn`） |
| `this`（默认可变） | `&self` / `&mut self` / `self` |
| 嵌套函数 | 支持内嵌 `fn`，但不能捕获变量 |
