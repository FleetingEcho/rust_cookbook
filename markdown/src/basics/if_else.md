# Rust 控制流

## 1. if / else if / else

Rust 中的 `if` 是表达式而非语句，可以返回值。条件表达式必须返回 `bool` 类型，不能使用隐式类型转换。

```rust
fn main() {
    let n = 6;

    if n % 4 == 0 {
        println!("number is divisible by 4");
    } else if n % 3 == 0 {
        println!("number is divisible by 3");
    } else if n % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

## 2. for 循环

Rust 的 `for` 循环遍历迭代器，使用 `1..=5` 表示包含边界的范围。`for` 循环有三种所有权模式：

| 使用方法 | 等价使用方式 | 所有权 |
|----------|-------------|--------|
| `for item in collection` | `for item in IntoIterator::into_iter(collection)` | 转移所有权 |
| `for item in &collection` | `for item in collection.iter()` | 不可变借用 |
| `for item in &mut collection` | `for item in collection.iter_mut()` | 可变借用 |

```rust
fn main() {
    for i in 1..=5 {
        println!("{}", i);
    }
}
```

## 3. while 循环

`while` 在条件为真时持续执行循环体。

```rust
fn main() {
    let mut n = 0;

    while n <= 5 {
        println!("{}!", n);
        n = n + 1;
    }

    println!("我出来了！");
}
```

### 3.1 索引遍历

使用 `while` 结合索引遍历数组：

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);
        index = index + 1;
    }
}
```

## 4. loop 循环

`loop` 是无限循环关键字，是一个表达式可以返回值。`break` 可以单独使用，也可以带一个返回值（类似 return）。

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}
```

---

## 📘 TypeScript 对比

**Rust：**

```rust
let x = if condition { 1 } else { 2 };
```

**TypeScript：**

```ts
const x = condition ? 1 : 2;
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| if 是表达式 | ✅ 有返回值 `let x = if cond { a } else { b }` | ❌ if 是语句，用三元 `cond ? a : b` |
| 循环 | `loop` / `while` / `for` | `while` / `for` / `do...while` |
| 无限循环 | `loop { }` 内置关键字 | `while (true) { }` |
| break 带值 | ✅ `break value;` | ❌ 不支持 |

> ⚠️ **Rust 没有 `do...while`，但有 `loop`（无限循环）和 `for`（遍历迭代器）。** TS 的 `for...of` 对应 Rust 的 `for item in iter`。

详细对照 → [rust_vs_typescript.rs](../rust_vs_typescript.rs)
