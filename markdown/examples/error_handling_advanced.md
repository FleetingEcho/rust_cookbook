# Rust vs TypeScript: 高级错误处理

**运行命令：** `cargo run -p learning_notes --example rts_error_handling_advanced`

## TypeScript 版本

```ts
class AppError extends Error {
    constructor(
        message: string,
        public readonly code: string,
        public readonly cause?: Error,
    ) {
        super(message);
        this.name = "AppError";
    }
}

class NetworkError extends Error { ... }
class ParseError extends Error { ... }
class NotFoundError extends Error { ... }

try {
    const data = JSON.parse(input);
    const result = await fetch(url);
} catch (e) {
    if (e instanceof SyntaxError) { ... }
    if (e instanceof NetworkError) { ... }
    throw new AppError("操作失败", "ERR_001", e as Error);
}

type Result<T, E extends Error = Error> =
    | { ok: true; value: T }
    | { ok: false; error: E };
```

## 一、自定义错误类型（枚举）

Rust 惯用法：用枚举定义所有可能的错误变体。

```rust
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)]
enum AppError {
    NotFound { resource: String, id: u32 },
    ParseError(ParseIntError),
    Network { url: String, status: u16 },
    InvalidInput(String),
    Internal { message: String, source: Box<dyn std::error::Error> },
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::NotFound { resource, id } =>
                write!(f, "未找到 {resource}（id={id}）"),
            AppError::ParseError(e) =>
                write!(f, "解析错误: {e}"),
            AppError::Network { url, status } =>
                write!(f, "网络错误 {status}: {url}"),
            AppError::InvalidInput(msg) =>
                write!(f, "无效输入: {msg}"),
            AppError::Internal { message, source } =>
                write!(f, "内部错误: {message}（原因: {source}）"),
        }
    }
}

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            AppError::ParseError(e) => Some(e),
            AppError::Internal { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}
```

## 二、From trait：自动错误类型转换

```rust
impl From<ParseIntError> for AppError {
    fn from(e: ParseIntError) -> Self {
        AppError::ParseError(e)
    }
}

fn parse_user_id(s: &str) -> Result<u32, AppError> {
    let id = s.parse::<u32>()?;
    if id == 0 {
        return Err(AppError::InvalidInput("ID 不能为 0".to_string()));
    }
    Ok(id)
}
```

## 三、错误传播（? 运算符）

```rust
fn find_user(id_str: &str) -> Result<String, AppError> {
    let id = parse_user_id(id_str)?;
    match id {
        1 => Ok(String::from("Alice")),
        2 => Ok(String::from("Bob")),
        _ => Err(AppError::NotFound { resource: "用户".to_string(), id }),
    }
}

fn greet_user(id_str: &str) -> Result<String, AppError> {
    let name = find_user(id_str)?;
    Ok(format!("你好，{name}！"))
}
```

## 四、多种错误类型的统一处理

```rust
fn flexible_parse(s: &str) -> Result<i64, Box<dyn std::error::Error>> {
    let n: i64 = s.trim().parse()?;
    Ok(n * 2)
}
```

## 五、错误处理组合子

```rust
let result: Result<i32, String> = "42"
    .parse::<i32>()
    .map_err(|e| format!("解析失败: {e}"));

let validated: Result<i32, String> = "10"
    .parse::<i32>()
    .map_err(|e| e.to_string())
    .and_then(|n| {
        if n > 0 { Ok(n) } else { Err("必须是正数".to_string()) }
    });

let val = "bad".parse::<i32>().unwrap_or(0);
let val2 = "bad".parse::<i32>().unwrap_or_else(|e| {
    eprintln!("解析失败: {e}");
    -1
});

let opt: Option<i32> = "42".parse::<i32>().ok();

let nested: Result<Result<i32, &str>, &str> = Ok(Ok(42));
let flat = nested.flatten();
```

## 六、在 main 中返回 Result

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let n: i32 = "42".parse()?;
    println!("{n}");
    Ok(())
}
```

## 七、生产代码推荐：thiserror + anyhow

| 库 | 适合场景 | TS 对应 |
|----|---------|---------|
| `thiserror` | 定义库/模块的错误类型 | `class extends Error` |
| `anyhow` | 应用层快速传播任意错误 | `catch (e: unknown)` + 重新抛出 |

### thiserror：消除样板代码

```toml
[dependencies]
thiserror = "1"
```

```rust
use thiserror::Error;

// 等价于之前手写的 Display + Error + From，但只需几行
#[derive(Error, Debug)]
enum AppError {
    #[error("未找到 {resource}（id={id}）")]
    NotFound { resource: String, id: u32 },

    #[error("解析错误: {0}")]          // {0} 引用第一个字段
    Parse(#[from] std::num::ParseIntError),  // #[from] 自动生成 From impl

    #[error("网络错误 {status}: {url}")]
    Network { url: String, status: u16 },

    #[error("IO 错误")]
    Io(#[from] std::io::Error),
}

fn parse_id(s: &str) -> Result<u32, AppError> {
    let id: u32 = s.parse()?;  // ParseIntError 自动转为 AppError::Parse
    Ok(id)
}
```

### anyhow：应用层快速错误处理

```toml
[dependencies]
anyhow = "1"
```

```rust
use anyhow::{Context, Result, bail, ensure, anyhow};

// anyhow::Result<T> = Result<T, anyhow::Error>，可承载任意错误类型
fn load_config(path: &str) -> Result<String> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("无法读取配置文件: {}", path))?;  // 附加上下文信息
    Ok(content)
}

fn validate_age(age: i32) -> Result<()> {
    ensure!(age >= 0, "年龄不能为负数，收到: {}", age);  // 条件失败则返回 Err
    ensure!(age <= 150, "年龄超出合理范围: {}", age);
    Ok(())
}

fn process_user(id: &str) -> Result<String> {
    if id.is_empty() {
        bail!("用户 ID 不能为空");  // 等价于 return Err(anyhow!("..."))
    }
    let num: u32 = id.parse().context("ID 必须是数字")?;
    Ok(format!("用户 #{}", num))
}

// main 返回 anyhow::Result，? 可传播任何错误
fn main() -> Result<()> {
    // 与 thiserror 配合：库用 thiserror 定义类型，应用层用 anyhow 处理
    let result = load_config("config.toml")
        .context("初始化失败")?;
    println!("{}", result);
    Ok(())
}
```

### 两者分工总结

```
写库/模块时（给别人调用）   →  thiserror：定义明确的错误类型
写应用程序时（最顶层代码）   →  anyhow：快速传播，附加上下文，不关心具体类型
```
