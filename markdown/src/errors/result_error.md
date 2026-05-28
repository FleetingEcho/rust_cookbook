# Result 与错误处理

## 1. Result 基础

`Result<T, E>` 是 Rust 处理错误的核心枚举，包含两个变体：

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

- `Ok(T)` 表示操作成功，携带返回值 `T`。
- `Err(E)` 表示操作失败，携带错误信息 `E`。

## 2. 文件操作中的 Result

```rust
use std::fs::File;
use std::io::Error;

fn read_file() -> Result<String, Error> {
    let f = File::open("hello.txt")?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}
```

## 3. 错误传播操作符 `?`

`?` 操作符用于简洁地传播错误：

```rust
use std::fs::File;

fn read_username_from_file() -> Result<String, std::io::Error> {
    let mut f = File::open("hello.txt")?;

    let mut username = String::new();
    f.read_to_string(&mut username)?;

    Ok(username)
}
```

`?` 放在 `Result` 值后面，如果值为 `Ok` 则提取内部值，如果值为 `Err` 则提前返回错误。

## 4. unwrap 与 expect

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
    let f = File::open("hello.txt").expect("打开 hello.txt 文件时出错");
}
```

- `unwrap()` — 成功返回内部值，失败则 panic。
- `expect(msg)` — 同 `unwrap()`，但可自定义 panic 消息。

## 5. 自定义错误类型

```rust
use std::fmt;

#[derive(Debug)]
enum AppError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Io(e) => write!(f, "IO 错误: {}", e),
            AppError::Parse(e) => write!(f, "解析错误: {}", e),
        }
    }
}
```

通过实现 `fmt::Display`，可以为自定义错误类型提供可读的错误描述。

---

## 📘 TypeScript 对比

Rust `Result<T, E>` ≈ TS 中的 `try/catch`。

**Rust：**

```rust
let result = do_something()?;
```

**TypeScript：**

```ts
try {
  const result = doSomething();
} catch (e) {
  // handle error
}
```

| 特性 | Rust | TypeScript |
|------|------|-----------|
| 错误表示 | `Result<T, E>` 枚举 | `throw` + `try/catch` |
| 错误传播 | `?` 操作符 | 异常自动冒泡 |
| 编译期检查 | 必须处理 `Result` | 无强制检查 |
| 错误类型 | 泛型 `E` 可自定义 | `any` / `Error` 对象 |

> ⚠️ Rust 强制你在编译期处理错误，而 TypeScript 的异常可以在运行时才被发现。

详细对照 → [rust_vs_typescript.rs §14 "错误处理"](../rust_vs_typescript.rs)
