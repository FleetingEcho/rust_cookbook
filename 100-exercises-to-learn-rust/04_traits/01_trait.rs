// 🔑 要点：trait 是 Rust 的"接口"——定义共享行为
// 可以为已有类型实现自己的 trait

// 定义 IsEven trait
trait IsEven {
    fn is_even(&self) -> bool;
}

// 为 u32 实现 IsEven
impl IsEven for u32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

// 为 i32 实现 IsEven
impl IsEven for i32 {
    fn is_even(&self) -> bool {
        self % 2 == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_is_even() {
        assert!(42u32.is_even());
        assert!(!43u32.is_even());
    }

    #[test]
    fn test_i32_is_even() {
        assert!(42i32.is_even());
        assert!(!43i32.is_even());
        assert!(0i32.is_even());
        assert!(!(-1i32).is_even());
    }
}
