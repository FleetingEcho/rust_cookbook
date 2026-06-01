// 欢迎来到 Rust 学习之旅！
// 本文件是一个 Rust 练习题的模板，你将通过修改代码来学习 Rust。
fn greeting() -> &'static str {
    // TODO: 将返回值改为 "I'm ready to learn Rust!"
    // 💡 提示：Rust 中字符串字面量的类型是 &'static str
    "I'm ready to learn Rust!"
}

// ⚠️ 以下为测试代码，请勿修改！
// 运行 `cargo test` 或 `rustc --test` 来执行测试
#[cfg(test)]
mod tests {
    use crate::greeting;

    #[test]
    fn test_welcome() {
        // assert_eq! 是 Rust 中最常用的测试宏之一
        // 如果两个值不相等，测试会失败并打印它们
        assert_eq!(greeting(), "I'm ready to learn Rust!");
    }
}
