# Rust 生命周期

Rust 的生命周期是编译器用来检查引用有效性的规则，防止悬垂引用（指向无效数据）。生命周期一般用 `'a`、`'b` 这样的标注表示。

## 1. 为什么需要生命周期？

Rust 采用**所有权 + 借用 + 生命周期**来管理内存，不需要垃圾回收。但是：

- 可变引用 `&mut T` 不能与其他引用共存。
- 生命周期确保引用的作用域是安全的，防止悬垂引用。

## 2. 生命周期基本规则

Rust 编译器会自动推导生命周期，但在某些复杂情况需要手动标注：

- 所有引用必须在有效作用域内。
- 可变借用 `&mut T` 不能与不可变借用 `&T` 共存。
- 多个不可变借用是允许的。
- 生命周期参数不会改变实际的生命周期，只是告诉编译器不同引用的关系。

## 3. 生命周期标注示例

```rust
fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() { s1 } else { s2 }
}
```

这里的 `'a` 表示 `s1` 和 `s2` 的生命周期相同，返回值的生命周期必须不短于参数的生命周期。

## 4. 生命周期常见问题

### 问题 1：编译器"太笨"

```rust
#[derive(Debug)]
struct Foo;
impl Foo {
    fn mutate_and_share(&mut self) -> &Self { &*self }
    fn share(&self) {}
}
fn main() {
    let mut foo = Foo;
    let loan = foo.mutate_and_share();
    foo.share(); // ❌ 编译错误
    println!("{:?}", loan);
}
```

**原因：** `mutate_and_share()` 返回不可变引用，但它的 `&mut self` 借用仍然有效，Rust 认为 `foo` 还被可变借用，导致 `foo.share()` 出错。

**解决：** 手动分离作用域，缩小 `loan` 的生命周期：

```rust
fn main() {
    let mut foo = Foo;
    {
        let loan = foo.mutate_and_share();
        println!("{:?}", loan);
    }
    foo.share(); // ✅ 现在可以了
}
```

## 5. 无界生命周期

如果一个生命周期是从裸指针推导出来的，Rust 无法推断它的实际作用域，就会出现无界生命周期：

```rust
fn f<'a, T>(x: *const T) -> &'a T {
    unsafe { &*x } // ❌ 'a 是凭空产生的，无界生命周期
}
```

**问题：** Rust 无法保证 `x` 这个指针的值不会在 `'a` 生命周期内变成无效地址。

> 🔹 解决：使用 `unsafe` 时要小心，确保生命周期绑定到有效的引用上。

## 6. 生命周期约束

可以用 `'a: 'b` 约束生命周期的大小关系：

```rust
struct DoubleRef<'a, 'b: 'a, T> {
    r: &'a T,
    s: &'b T,
}
```

`'b: 'a` 代表 `'b` 至少和 `'a` 一样长，意味着 `s` 活得比 `r` 久。

## 7. NLL（非词法作用域）

Rust 1.31+ 引入了 NLL（Non-Lexical Lifetime），让借用的生命周期在最后一次使用后立即结束：

```rust
fn main() {
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2); // r1, r2 生命周期结束

    let r3 = &mut s; // ✅ 现在可以借用了！
    println!("{}", r3);
}
```

> 🔹 以前：`r1`, `r2` 生命周期会持续到 `main` 结束，导致 `r3` 借用失败。
>
> 🔹 现在：Rust 只让 `r1`, `r2` 活到 `println!` 结束，`r3` 可以安全借用。

## 8. Reborrow（再借用）

```rust
#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r; // ✅ 允许再借用
    println!("{:?}", rr);

    r.x = 10; // ✅ 这里 `rr` 已经被释放，可以修改 `r`
}
```

## 9. 生命周期消除规则

**impl 里的生命周期：**

```rust
impl<'a> Reader for BufReader<'a> {}
// 可以用匿名生命周期 '_ 省略：
impl Reader for BufReader<'_> {}
```

**生命周期自动推导（Rust 2018+）：**

```rust
struct Ref<'a, T> { field: &'a T } // ✅ 无需 T: 'a
```

## 10. 复杂生命周期示例

```rust
struct Interface<'b, 'a: 'b> {
    manager: &'b mut Manager<'a>,
}

impl<'b, 'a: 'b> Interface<'b, 'a> {
    pub fn noop(self) { println!("interface consumed"); }
}

struct Manager<'a> { text: &'a str }

struct List<'a> { manager: Manager<'a> }

impl<'a> List<'a> {
    pub fn get_interface<'b>(&'b mut self) -> Interface<'b, 'a> where 'a: 'b {
        Interface { manager: &mut self.manager }
    }
}

fn main() {
    let mut list = List { manager: Manager { text: "hello" } };
    list.get_interface().noop();
    println!("Interface should be dropped here and the borrow released");

    use_list(&list); // ✅ 现在可以借用了
}

fn use_list(list: &List) { println!("{}", list.manager.text); }
```

> 🔹 问题：如果 `get_interface` 用 `'a` 而不是 `'b`，Rust 认为 `list` 在 `main` 结束前都在可变借用，导致 `use_list(&list)` 报错。
>
> 🔹 解决：使用 `'b` 让 `get_interface` 生命周期短于 `List<'a>`，保证可变借用在用完后释放。

## 附录：&'static 与 T: 'static 的区别

简单来说，`&'static` 和 `T: 'static` 主要的区别在于**它们作用的对象不同**：

### &'static —— 适用于引用

`&'static` 代表引用的生命周期必须和整个程序一样长。适用于字符串字面量（因为它们存储在二进制文件中，程序运行期间不会被释放）。

```rust
fn print_author(author: &'static str) {
    println!("{}", author);
}

fn main() {
    let mark_twain: &str = "Samuel Clemens";  // 这个字符串字面量是 'static
    print_author(mark_twain);
}
```

这里 `mark_twain` 持有的是 `&'static str`，但 `mark_twain` 本身的作用域还是受 `main` 函数控制。变量不会存活整个程序生命周期，但字符串数据会。

### T: 'static —— 适用于泛型类型

`T: 'static` 不是引用，它是一个泛型生命周期约束。表示 `T` 本身（而不是引用）必须存活整个程序生命周期。

**示例1（报错情况）：**

```rust
fn print_it<T: 'static>(input: T) {
    println!("{:?}", input);
}

fn main() {
    let i = 5;
    print_it(&i); // ❌ 报错，因为 &i 不是 'static
}
```

`i` 只是 `main` 里的局部变量，它的引用 `&i` 不能活得和程序一样久，因此报错。

**示例2（不会报错）：**

```rust
fn print_it<T: 'static>(input: &T) {
    println!("{:?}", input);
}

fn main() {
    let i = 5;
    print_it(&i);  // ✅ 不报错，因为这里是 `&T`，编译器不会严格检查 `T` 的生命周期
}
```

这里 `T` 只是被引用 `&T` 约束了，而 `&T` 只要符合作用域规则即可，不需要 `T` 是 `'static`。

### 重点总结

| 区别 | `&'static` | `T: 'static` |
|------|-----------|-------------|
| 适用范围 | 引用 | 泛型类型 |
| 要求 | 引用必须活得和程序一样久 | T 不能包含非 'static 数据 |
| 使用场景 | 适用于字符串字面量、全局静态变量等 | 泛型约束，确保 T 不依赖短生命周期数据 |
| 变量作用域 | 变量的作用域受限制，但引用指向的数据可能是 'static | 只要 T 是 &T，编译器不会严格检查 |

**总结一句话：**

`&'static` 适用于引用，要求引用的数据必须活得和程序一样久，但变量作用域仍受限制。

`T: 'static` 适用于泛型类型，保证 `T` 本身不依赖短生命周期数据，但如果 `T` 只是 `&T`，编译器不会严格检查 `T` 的生命周期。

## 总结

1. 生命周期用于编译器检查引用的有效性，防止悬垂引用。
2. Rust 自动推导生命周期，但复杂情况下需手动标注。
3. 生命周期标注 `<'a>` 让不同引用的作用域明确。
4. NLL 让生命周期更加智能，避免过早报错。
5. Reborrow 允许安全地再借用但不能交叉使用。
6. 遇到生命周期错误，尝试缩短作用域或调整参数顺序。

## 📘 TypeScript 对比

Rust 生命周期 ≈ TS 开发者完全不需要关心的东西。

| 维度 | Rust | TypeScript |
|------|------|-----------|
| 生命周期标注 | 手动 `'a` 标注引用关系 | 不需要 |
| `'static` | 引用存活整个程序 | `const` 字符串自动活到程序结束 |
| NLL（非词法作用域） | 借用作用域到"最后一次使用" | 不适用（GC） |
| Reborrow | 安全低层借用再借用 | 标量赋值就是引用拷贝 |
| 悬垂引用 | 编译期阻止 | 不适用 |

> 重点：TS 开发者学生命周期时，不要试图找 "TS 对应物"——它不存在。把它当作 Rust 的编译期安全检查机制来理解就好：生命周期的存在 = 编译器帮你确保不会访问已释放的内存。

详细对照 → `rust_vs_typescript.rs §8 "生命周期"`
