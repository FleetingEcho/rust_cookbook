# Rust 枚举 (Enum)

## 1. 枚举定义与使用

Rust 的枚举比传统语言更强大，每个变体可以携带不同类型的数据：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let m1 = Message::Quit;
let m2 = Message::Move { x: 1, y: 1 };
let m3 = Message::ChangeColor(255, 255, 0);
```

## 2. Option 枚举处理空值

Rust 中没有 `null`，而是使用 `Option<T>` 枚举来安全表示可能为空的结果。Rust 强制你在使用 `Option<T>` 之前先处理 `None` 的情况，这避免了空指针错误。

```rust
fn get_username(id: u32) -> Option<String> {
    if id == 1 {
        Some("Rust".to_string())
    } else {
        None
    }
}

let user = get_username(1);
match user {
    Some(name) => println!("Username: {}", name),
    None => println!("No user found"),
}
```

优势：Rust 强制你处理 `None`，不会出现 `null` 访问导致的崩溃。

### 2.1 unwrap 方法

如果确定 `Option` 一定有值，可以用 `.unwrap()` 直接获取。但要注意，如果 `None.unwrap()`，程序会直接崩溃。

```rust
let username = Some("Olivia".to_string());
println!("{}", username.unwrap()); // Olivia
```

推荐使用 `.expect()` 提供错误信息：

```rust
println!("{}", username.expect("Username not found!"));
```

### 2.2 unwrap_or 提供默认值

```rust
let username = None;
println!("{}", username.unwrap_or("Guest".to_string())); // Guest
```

### 2.3 使用 `?` 运算符

```rust
fn first_char(s: Option<&str>) -> Option<char> {
    Some(s?.chars().next()?) // 如果 s 是 None，直接返回 None
}

println!("{:?}", first_char(Some("hello"))); // Some('h')
println!("{:?}", first_char(None));          // None
```

---

## TypeScript 对比

Rust 的枚举约等于 TypeScript 的 discriminated union，但功能更强大。

**Rust：**

```rust
enum Message { Quit, Move{x:i32,y:i32}, Write(String) }
match msg {
    Message::Quit => ...,
    Message::Move{x,y} => ...,
}
```

**TypeScript：**

```ts
type Message =
  | { kind: 'Quit' }
  | { kind: 'Move'; x: number; y: number }
  | { kind: 'Write'; value: string };
switch (msg.kind) {
  case 'Move': ...
}
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 变体类型 | 每个变体可带不同类型数据 | 用 discriminated union 模拟 |
| 穷尽检查 | match 必须覆盖所有分支 | switch 不强制 |
| match 返回值 | 是表达式 | 语句，无返回值 |
| Option<T> | 内置枚举 `Some(T)` / `None` | `null` / `undefined` |

详细对照 → [rust_vs_typescript.rs §5](../rust_vs_typescript.rs) "枚举与模式匹配"
