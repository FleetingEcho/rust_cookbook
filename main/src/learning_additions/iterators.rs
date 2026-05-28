// 迭代器是 Rust 中处理集合的常用方式。
// 常见方法：map 转换、filter 过滤、fold 汇总、collect 收集。

pub fn square_even_numbers(numbers: &[i32]) -> Vec<i32> {
    numbers
        .iter()
        .copied()
        .filter(|number| number % 2 == 0)
        .map(|number| number * number)
        .collect()
}

pub fn sum_with_fold(numbers: &[i32]) -> i32 {
    numbers.iter().fold(0, |total, number| total + number)
}

pub fn find_first_long_word<'a>(words: &'a [&str], min_len: usize) -> Option<&'a str> {
    words.iter().copied().find(|word| word.len() >= min_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn squares_even_numbers() {
        assert_eq!(square_even_numbers(&[1, 2, 3, 4]), vec![4, 16]);
    }

    #[test]
    fn finds_long_word() {
        let words = ["go", "rust", "typescript"];
        assert_eq!(find_first_long_word(&words, 4), Some("rust"));
    }
}

// 📘 TypeScript 对比
// ====================
// Rust 和 TS 的迭代器风格很接近：
//
// ```rust
// words.iter()
//     .filter(|w| w.len() > 4)
//     .map(|w| w.to_uppercase())
//     .collect::<Vec<_>>();
// ```
// ```ts
// words
//     .filter(w => w.length > 4)
//     .map(w => w.toUpperCase());
// ```
//
// | 差异 | Rust | TypeScript |
// |------|------|-----------|
// | 执行时机 | 惰性（collect 才执行） | 即时执行 |
// | 中间分配 | 零开销（编译期优化） | 每次 map/filter 创建新数组 |
// | 三种迭代 | iter/iter_mut/into_iter | 只有一种 |
//
// 详细对照 → rust_vs_typescript.rs §14
