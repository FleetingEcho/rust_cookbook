# 模式匹配

## 简介

模式匹配不只是 `match`，还包括 `if let`、`while let`、函数参数解构等。

## 示例代码

```rust
#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(u8, u8, u8),
}

pub fn describe_command(command: Command) -> String {
    match command {
        Command::Quit => "退出".to_string(),
        Command::Move { x, y } => format!("移动到 ({x}, {y})"),
        Command::Write(text) if text.is_empty() => "写入空文本".to_string(),
        Command::Write(text) => format!("写入: {text}"),
        Command::ChangeColor(0, 0, 0) => "改成黑色".to_string(),
        Command::ChangeColor(r, g, b) => format!("改成 rgb({r}, {g}, {b})"),
    }
}

pub fn take_until_none(values: Vec<Option<i32>>) -> Vec<i32> {
    let mut output = Vec::new();

    for value in values {
        // if let 适合只关心一个匹配分支的场景。
        if let Some(number) = value {
            output.push(number);
        } else {
            break;
        }
    }

    output
}

pub fn destructure_tuple(point: (i32, i32, i32)) -> i32 {
    let (x, _y, z) = point;
    x + z
}
```

## 测试

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn describes_commands() {
        assert_eq!(describe_command(Command::Quit), "退出");
        assert_eq!(
            describe_command(Command::Move { x: 3, y: 4 }),
            "移动到 (3, 4)"
        );
        assert_eq!(describe_command(Command::ChangeColor(0, 0, 0)), "改成黑色");
    }

    #[test]
    fn stops_at_none() {
        let values = vec![Some(1), Some(2), None, Some(3)];
        assert_eq!(take_until_none(values), vec![1, 2]);
    }
}
```

---

## 📘 TypeScript 对比

Rust 模式匹配 = 解构 + switch + 守卫 合体。

**Rust：**

```rust
match value {
    User { name, age: 30 } => ...  // 解构 + 条件
    (a, b) if a > b => ...          // 元组 + 守卫
    _ => ...                        // 通配
}
```

**TypeScript：**

```ts
// TS 需要分开写
if (value instanceof User && value.age === 30) { ... }
```

详细对照 → rust_vs_typescript.rs §5
