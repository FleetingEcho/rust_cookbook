// 🔑 要点：第七章学习 Rust 并发编程
// 涉及线程创建、通道（mpsc）、互斥锁（Mutex）、读写锁（RwLock）
// Send 和 Sync 是 Rust 并发安全的核心标记 trait

fn intro() -> &'static str {
    "I'm ready to build a concurrent ticket management system!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(
            intro(),
            "I'm ready to build a concurrent ticket management system!"
        );
    }
}
