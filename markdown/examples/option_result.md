# Rust vs TypeScript：Option 与 Result

> **运行命令**：`cargo run -p learning_notes --example rts_option_result`

---

## TypeScript 参考版本

```ts
// Option<T> 对应 TS 的 T | null | undefined
function findUser(id: number): User | null {
    return db.find(u => u.id === id) ?? null;
}
const user = findUser(1);
if (user !== null) {
    console.log(user.name);  // 类型收窄
}
const name = user?.name ?? "游客";     // 可选链 + 空值合并
const upper = user?.name.toUpperCase(); // 可选链

// Result<T, E> 对应 TS 的 try/catch 或 返回 Error
function parseAge(s: string): number {
    const n = parseInt(s);
    if (isNaN(n)) throw new Error(`无效数字: ${s}`);
    return n;
}
try {
    const age = parseAge("abc");
} catch (e) {
    console.error((e as Error).message);
}

// 链式可能失败的操作
async function getCity(userId: number): Promise<string | null> {
    const user = await findUser(userId);
    const address = user?.address;
    return address?.city ?? null;
}
```

---

## 自定义错误类型

**TS**: `class AppError extends Error`

```rust
#[derive(Debug)]
enum AppError {
    NotFound(String),
    ParseError(String),
    InvalidInput(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AppError::NotFound(s)      => write!(f, "未找到: {s}"),
            AppError::ParseError(s)    => write!(f, "解析错误: {s}"),
            AppError::InvalidInput(s)  => write!(f, "无效输入: {s}"),
        }
    }
}
```

### `?` 运算符：自动传播错误

**相当于 TS 的 throw/rethrow**

```rust
fn parse_age(s: &str) -> Result<u32, AppError> {
    s.parse::<u32>()
        .map_err(|_| AppError::ParseError(format!("'{s}' 不是有效年龄")))
}

fn validate_age(age: u32) -> Result<u32, AppError> {
    if age > 150 {
        Err(AppError::InvalidInput(format!("年龄 {age} 不合理")))
    } else {
        Ok(age)
    }
}

// ? 运算符：自动传播错误
// TS 需要 try { const age = parseAge(s); validateAge(age); } catch(e) { throw e; }
fn parse_and_validate(s: &str) -> Result<u32, AppError> {
    let age = parse_age(s)?;       // 如果 Err，直接 return Err(...)
    let valid = validate_age(age)?;
    Ok(valid)
}
```

---

## 一、Option<T> 基础

**TS 对应**：`T | null | undefined`

```rust
let some_val: Option<i32> = Some(42);
let none_val: Option<i32> = None;
```

### match：最完整的处理方式

**TS**: `if (user !== null) { ... } else { ... }`

```rust
match find_user(1) {
    Some(name) => println!("找到用户: {name}"),
    None       => println!("用户不存在"),
}
```

### if let：只关心 Some 的情况

**TS**: `if (user !== null) { console.log(user.name) }`

```rust
if let Some(name) = find_user(2) {
    println!("if let: {name}");
}
```

### unwrap_or：提供默认值

**TS**: `findUser(99) ?? "游客"`

```rust
let name = find_user(99).unwrap_or(String::from("游客"));
```

### unwrap_or_else：懒惰求值

**TS**: `findUser(99) ?? expensiveCompute()`

```rust
let name2 = find_user(99).unwrap_or_else(|| String::from("计算出的默认值"));
```

### unwrap：直接取值，None 时会 panic

**TS**: `user!`（非空断言，同样不安全）

```rust
let name3 = find_user(1).unwrap();
```

### map：对 Some 内的值做变换

**TS**: `user?.name.toUpperCase()`

```rust
let upper = find_user(1).map(|n| n.to_uppercase());
// Some("ALICE")
let upper_none = find_user(99).map(|n| n.to_uppercase());
// None
```

### and_then：链式 Option 操作（可选链）

**TS**: `user?.address?.city`

```rust
fn get_address(name: &str) -> Option<String> {
    if name == "Alice" { Some(String::from("北京")) } else { None }
}
let city = find_user(1).and_then(|name| get_address(&name));
```

### filter：对 Some 的值加条件

**TS**: `user !== null && user.age > 18 ? user : null`

```rust
let long_name = find_user(1).filter(|n| n.len() > 3);
```

### 其他方法

```rust
println!("is_some: {}", find_user(1).is_some());   // TS: user !== null
println!("is_none: {}", find_user(99).is_none());  // TS: user === null

// ok_or：Option → Result
let result: Result<String, AppError> = find_user(99)
    .ok_or(AppError::NotFound("id=99".to_string()));
```

---

## 二、Result<T, E> 基础

**TS 对应**：try/catch 或 T | Error

```rust
let ok_val: Result<i32, String> = Ok(42);
let err_val: Result<i32, String> = Err(String::from("出错了"));
```

### match — TS: try { ... } catch (e) { ... }

```rust
match parse_age("25") {
    Ok(age)  => println!("解析成功: {age}"),
    Err(e)   => println!("解析失败: {e}"),
}
```

### unwrap_or — TS: try { parseAge(s) } catch { 0 }

```rust
let age = parse_age("bad").unwrap_or(0);
```

### map — 变换 Ok 内的值

```rust
let doubled = parse_age("21").map(|n| n * 2);
```

### map_err — 变换 Err 内的错误

```rust
let result: Result<i32, String> = "42".parse::<i32>()
    .map_err(|e| format!("解析失败: {e}"));
```

### and_then — 链式 Result

**TS**: `parseAge(s).then(validateAge)`（Promise 链）

```rust
let validated = parse_age("25").and_then(|age| validate_age(age));
```

### ? 运算符

```rust
println!("? 传播: {:?}", parse_and_validate("25"));
println!("? 传播: {:?}", parse_and_validate("200"));
println!("? 传播: {:?}", parse_and_validate("abc"));
```

---

## 三、批量处理

**TS** 需要手动 try/catch 循环。

```rust
let inputs = vec!["10", "abc", "20", "bad", "30"];

// 收集所有结果（包含 Ok 和 Err）
let results: Vec<Result<u32, AppError>> = inputs.iter()
    .map(|s| parse_age(s))
    .collect();

// 只收集成功的值（忽略错误）
// TS: inputs.map(parseInt).filter(n => !isNaN(n))
let successes: Vec<u32> = inputs.iter()
    .filter_map(|s| parse_age(s).ok())
    .collect();

// 只要有一个失败就整体失败（collect::<Result<Vec<_>, _>>()）
let all_valid: Result<Vec<u32>, AppError> = vec!["10", "20", "30"]
    .iter().map(|s| parse_age(s)).collect();

let any_invalid: Result<Vec<u32>, AppError> = vec!["10", "bad", "30"]
    .iter().map(|s| parse_age(s)).collect();
```
