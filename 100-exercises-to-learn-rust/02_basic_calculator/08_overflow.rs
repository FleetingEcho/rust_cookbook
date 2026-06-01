// 🔑 要点：整数溢出——使用 wrapping_mul 显式回绕
// Rust 在 debug 模式下默认检查溢出（panic）
// 使用 wrapping_* 方法可以显式选择回绕行为

pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // wrapping_mul 在溢出时回绕（wrap around）
        result = result.wrapping_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! = 2432902008176640000，远超 u32 的最大值 4294967295
        // 开启 overflow-checks 时这里会 panic
        // 关闭后则回绕（wrapping）到 2_192_834_560
        assert_eq!(factorial(20), 2_192_834_560);
    }

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
