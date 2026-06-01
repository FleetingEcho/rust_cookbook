// 🔑 要点：Rust 的 if/else 是**表达式**，可以返回值
// 不需要括号包裹条件，但必须用花括号包裹分支

/// 如果 n 是偶数返回 12，
/// 如果 n 能被 3 整除返回 13，
/// 否则返回 17
fn magic_number(n: u32) -> u32 {
    // 💡 使用 if/else if/else 实现多重条件判断
    // 注意：条件和 if 之间不需要括号
    if n % 2 == 0 {
        12        // 偶数优先：6 既是偶数也是 3 的倍数，返回 12
    } else if n % 3 == 0 {
        13
    } else {
        17
    }
}

#[cfg(test)]
mod tests {
    use crate::magic_number;

    #[test]
    fn one() {
        assert_eq!(magic_number(1), 17);
    }

    #[test]
    fn two() {
        assert_eq!(magic_number(2), 12);
    }

    #[test]
    fn six() {
        // 6 既是偶数也是 3 的倍数，但偶数判断优先 → 12
        assert_eq!(magic_number(6), 12);
    }

    #[test]
    fn nine() {
        assert_eq!(magic_number(9), 13);
    }

    #[test]
    fn high() {
        assert_eq!(magic_number(233), 17);
    }
}
