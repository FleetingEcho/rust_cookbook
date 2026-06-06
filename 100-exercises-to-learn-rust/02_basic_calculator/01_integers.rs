// 🔑 要点：Rust 的整数类型有不同的大小：u8/u16/u32/u64
// 不同大小的整数类型不能直接进行算术运算，需要类型转换
// 编译器会报错：`cannot add u32 to u8`

fn compute(a: u32, b: u32) -> u32 {
    // 🐛 修改前：`let multiplier: u8 = 4;` → 不能将 u8 与 u32 相加
    // ✅ 改为 u32 或使用 as 转换：`multiplier as u32`
    let multiplier: u32 = 4; // 保持与 a, b 相同的类型
    a + b * multiplier
}

// 🔍 compute(1, 2) = 1 + (2 * 4) = 9

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 9);
    }
}
