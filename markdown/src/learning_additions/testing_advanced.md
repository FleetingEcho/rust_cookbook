# 高级测试

## 简介

Rust 内置测试框架支持多种测试形式，这里演示超出基础之外的用法：

- **文档测试（doc test）** — 写在 `///` 注释里，`cargo test` 会自动运行
- **`#[should_panic]`** — 期望 panic 的测试
- **返回 Result 的测试** — 直接用 `?` 传播错误，失败时显示错误信息
- **`#[ignore]`** — 标记慢测试，默认跳过
- **测试辅助函数** — 在测试模块里共享 setup/teardown 逻辑

## 常用 cargo test 命令

```
cargo test                          -- 运行所有测试
cargo test -p learning_notes        -- 只运行这个 crate
cargo test <name>                   -- 只运行名字包含 <name> 的测试
cargo test -- --nocapture           -- 显示 println! 输出
cargo test -- --ignored             -- 只运行 #[ignore] 的测试
cargo test -- --include-ignored     -- 运行全部（含 ignored）
```

## 文档测试

写在 `///` 里的 \`\`\`rust 代码块会被 `cargo test` 当成测试运行。优点：文档和测试永远保持同步，文档即测试。

```rust
/// 把字符串的首字母转为大写，其余不变。
///
/// # Examples
///
/// ```
/// use learning_notes::learning_additions::testing_advanced::capitalize;
/// assert_eq!(capitalize("hello"), "Hello");
/// assert_eq!(capitalize(""),      "");
/// assert_eq!(capitalize("rust"),  "Rust");
/// ```
pub fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None    => String::new(),
        Some(c) => c.to_uppercase().to_string() + chars.as_str(),
    }
}

/// 把两个数相除，除数为 0 时返回 None。
///
/// # Examples
///
/// ```
/// use learning_notes::learning_additions::testing_advanced::safe_div;
/// assert_eq!(safe_div(10, 2), Some(5));
/// assert_eq!(safe_div(7, 0),  None);
/// ```
pub fn safe_div(a: i32, b: i32) -> Option<i32> {
    if b == 0 { None } else { Some(a / b) }
}
```

## 普通测试模块

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // ── #[should_panic] ───────────────────────────────────────────────────────
    // 用于测试"这段代码必须 panic"。
    // expected 参数可选，指定 panic 消息中必须包含的子串。

    pub fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("除数不能为零");
        }
        a / b
    }

    #[test]
    #[should_panic(expected = "除数不能为零")]
    fn panics_on_zero_divisor() {
        divide(10, 0);
    }

    // ── 返回 Result 的测试 ────────────────────────────────────────────────────
    // 函数签名改为 -> Result<(), E>，可以直接用 ? 操作符，
    // 出错时测试失败并显示完整错误信息，比 unwrap() 更清晰。

    #[test]
    fn parse_integer_with_result() -> Result<(), std::num::ParseIntError> {
        let n: i32 = "42".parse()?; // ? 让错误自动传播，测试失败时打印错误
        assert_eq!(n, 42);
        Ok(())
    }

    #[test]
    fn json_parse_with_result() -> Result<(), serde_json::Error> {
        let val: serde_json::Value = serde_json::from_str(r#"{"ok":true}"#)?;
        assert_eq!(val["ok"], true);
        Ok(())
    }

    // ── #[ignore] ─────────────────────────────────────────────────────────────
    // 标记那些很慢、依赖外部服务、或暂时跳过的测试。
    // cargo test 默认不运行它们，cargo test -- --ignored 才会运行。

    #[test]
    #[ignore = "需要网络，CI 里跳过"]
    fn slow_network_test() {
        // 模拟一个需要很长时间或外部依赖的测试
        std::thread::sleep(std::time::Duration::from_secs(1));
    }

    // ── 测试辅助函数（setup / fixture）───────────────────────────────────────
    // 把重复的初始化逻辑提取到辅助函数，多个测试共享。

    fn make_sample_list() -> Vec<i32> {
        vec![3, 1, 4, 1, 5, 9, 2, 6]
    }

    #[test]
    fn max_of_sample() {
        let list = make_sample_list();
        assert_eq!(list.iter().max(), Some(&9));
    }

    #[test]
    fn min_of_sample() {
        let list = make_sample_list();
        assert_eq!(list.iter().min(), Some(&1));
    }

    // ── 多断言 + 自定义错误信息 ───────────────────────────────────────────────
    // assert_eq! 失败时自动显示两侧的值。
    // assert! 的第二参数是失败时的附加消息，支持格式化。

    #[test]
    fn capitalize_various_inputs() {
        let cases = [
            ("hello", "Hello"),
            ("rust",  "Rust"),
            ("",      ""),
            ("a",     "A"),
        ];
        for (input, expected) in cases {
            assert_eq!(
                capitalize(input),
                expected,
                "capitalize({input:?}) 返回值不对"
            );
        }
    }

    // ── 测试 panic 消息（不用 should_panic）──────────────────────────────────
    // 用 std::panic::catch_unwind 捕获 panic，可以进一步检查消息内容。

    #[test]
    fn catch_panic_message() {
        let result = std::panic::catch_unwind(|| divide(5, 0));
        assert!(result.is_err());
    }
}
```
