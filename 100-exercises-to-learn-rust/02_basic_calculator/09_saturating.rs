// 🔑 要点：Rust 提供了安全的算术方法如 saturating_mul
// 饱和运算：超过最大值时停在最大值，而不是回绕
// 常用的三类方法：
// - wrapping_* : 回绕（回绕到最小值重新开始）
// - saturating_* : 饱和（停在最大值/最小值）
// - checked_* : 返回 Option（溢出时返回 None）

pub fn factorial(n: u32) -> u32 {
    let mut result: u32 = 1;
    for i in 1..=n {
        // 使用饱和乘法：结果超过 u32::MAX 时就停在 u32::MAX
        result = result.saturating_mul(i);
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn twentieth() {
        // 20! 在 u32 中饱和到最大值 4294967295
        assert_eq!(factorial(20), u32::MAX);
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
