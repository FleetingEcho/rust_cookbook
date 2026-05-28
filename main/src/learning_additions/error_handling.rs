// Rust 的错误处理分两类：
// 1. Option<T>：表示“可能没有值”。
// 2. Result<T, E>：表示“可能失败，并且失败时有错误信息”。

pub fn parse_optional_number(input: &str) -> Option<i32> {
    input.trim().parse::<i32>().ok()
}

pub fn divide(left: i32, right: i32) -> Result<i32, String> {
    if right == 0 {
        return Err("除数不能为 0".to_string());
    }

    Ok(left / right)
}

pub fn option_and_result_flow(input: &str) -> Result<i32, String> {
    // ok_or_else 可以把 Option 转成 Result。
    let number = parse_optional_number(input).ok_or_else(|| "请输入有效整数".to_string())?;

    // ? 会在 Err 时提前返回，在 Ok 时取出里面的值。
    divide(100, number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_optional_number() {
        assert_eq!(parse_optional_number("42"), Some(42));
        assert_eq!(parse_optional_number("abc"), None);
    }

    #[test]
    fn returns_error_for_zero_division() {
        assert_eq!(divide(10, 2), Ok(5));
        assert!(divide(10, 0).is_err());
    }
}

// 📘 TypeScript 对比
// ====================
// Rust 用 `Result<T, E>` 返回错误，TS 用 `throw`。
//
// ```rust
// fn div(a: i32, b: i32) -> Result<i32, String> { ... }
// let x = div(10, 0)?;  // 错误自动传播
// ```
// ```ts
// function div(a: number, b: number): number {
//   if (b === 0) throw new Error("...");
//   return a / b;
// }
// ```
//
// ⚠️ `?` 运算符是 TS 没有但很实用的特性——遇到 Err 自动 return。
//
// 详细对照 → rust_vs_typescript.rs §12
