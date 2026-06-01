// 🔑 要点：Drop trait 允许在值离开作用域时执行清理代码
// Rust 会自动在值不再使用时调用 drop
// 后续会深入讲解 Drop trait

fn outro() -> &'static str {
    "I have a basic understanding of destructors!"
}

#[cfg(test)]
mod tests {
    use crate::outro;

    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a basic understanding of destructors!");
    }
}
