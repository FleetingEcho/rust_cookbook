fn compute(a: u32, b: u32) -> u32 {
    let multiplier: u32 = 4 as u32; // 保持与 a, b 相同的类型
    a + b * multiplier
}

// 🔍 compute(1, 2) = 1 + (2 * 4) = 9

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
        println!("Success");
    }
}
