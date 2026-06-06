// 🔑 要点：Send 和 Sync 是 Rust 并发的基础标记 trait
// Send: 类型可以跨线程转移所有权
// Sync: 类型可以跨线程共享引用 (&T: Send)
// 大多数类型自动实现了 Send + Sync

fn outro() -> &'static str {
    "I have a good understanding of Send and Sync!"
}
#[cfg(test)]
mod tests {
    use crate::outro;
    #[test]
    fn test_outro() {
        assert_eq!(outro(), "I have a good understanding of Send and Sync!");
    }
}
