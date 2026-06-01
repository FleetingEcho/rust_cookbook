// 🔑 要点：Rust 的字符串字面量 &str 是不可变的 UTF-8 切片
// &'static str 表示在整个程序生命周期内有效的字符串引用

fn intro() -> &'static str {
    // TODO: 将 `__` 替换为正确的短语
    "I'm ready to build a calculator in Rust!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a calculator in Rust!");
    }
}
