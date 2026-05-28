# Rust 结构体 (Struct)

## 1. 结构体定义与初始化

初始化实例时，每个字段都需要进行初始化。初始化时的字段顺序不需要和结构体定义时的顺序一致。

```rust
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("anotheremail@example.com");
```

## 2. 结构体更新语法与所有权

```rust
// 这里 user1.username 的所有权被转移给 user2，user1 不能再使用 username
let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};

println!("user2: {:?}", user2);
```

`user1` 已经失去了 `username` 和其他字段的所有权，不能再访问。Rust 规定：如果结构体的某些字段的所有权被移动了，整个结构体都不能再被使用。即使 `active` 只是 `bool` 类型，它仍然属于 `user1`，但因为 `user1` 的一部分已经被移动了，所以 `user1` 整体都不能访问。

```rust
// 手动重新创建 user1
let user1 = User {
    email: String::from("recreated@example.com"),
    username: String::from("new_username"),
    active: false,
    sign_in_count: 0,
};

println!("user1: {:?}", user1);
```

### 2.1 如何解决所有权问题

**方案一：使用 clone**

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: user1.username.clone(),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

**方案二：使用 Arc<String> 共享所有权**

```rust
use std::sync::Arc;

struct User {
    active: bool,
    username: Arc<String>, // 共享所有权
    email: String,
    sign_in_count: u64,
}
```

## 3. 构造函数模式

```rust
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```

## 4. 元组结构体

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

## 5. dbg! 宏

`dbg!` 宏会拿走表达式的所有权，打印出文件名、行号等 debug 信息，以及表达式的求值结果，然后返回表达式值的所有权。`dbg!` 输出到标准错误输出 `stderr`，而 `println!` 输出到标准输出 `stdout`。

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

let scale = 2;
let rect1 = Rectangle {
    width: dbg!(30 * scale), // [src/main.rs:10] 30 * scale = 60
    height: 50,
};

dbg!(&rect1);
// [src/main.rs:14] &rect1 = Rectangle { width: 60, height: 50 }
```

---

## TypeScript 对比

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 定义 | `struct User { name: String }` | `class User { name: string }` |
| 方法 | 单独 `impl` 块 | class 内部定义 |
| 构造 | `User { name: "x".into() }` | `new User("x")` |
| 更新语法 | `User { name: "y", ..old }` | `{ ...old, name: "y" }` |
| 打印调试 | `#[derive(Debug)]` + `{:#?}` | `console.log(JSON.stringify(...))` |

Rust 的 struct 和 TS 的 class 最关键区别：

- Rust struct 只有数据，行为在 `impl` 中（类似 C 结构体 + 函数表）
- TS class 把数据和方法放在一起
- Rust 没有 `new` 关键字，构造就是直接赋值
