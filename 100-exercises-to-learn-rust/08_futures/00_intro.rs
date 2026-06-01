// 🔑 要点：第八章学习 Rust 异步编程
// async/.await 语法、tokio 运行时、future、spawn_blocking

fn intro() -> &'static str {
    "I'm ready to learn about futures!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to learn about futures!");
    }
}
