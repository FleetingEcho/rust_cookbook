// 🔑 要点：for 循环是 Rust 最常用的循环，配合范围表达式使用
// `1..=n` 是包含 n 的范围（右闭），`1..n` 是不包含 n 的范围（右开）

// 使用 for 循环实现阶乘——最地道的 Rust 风格
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;

    // `1..=n` 生成从 1 到 n（包含 n）的迭代器
    // 相比 while 循环更安全（不会忘记更新计数器）
    for i in 1..=n {
        result *= i;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1); // 空范围：循环体不会执行
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
