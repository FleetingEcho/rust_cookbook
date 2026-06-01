// 🔑 要点：切片 &[T] 是对连续元素的引用
// 函数参数用 &[u32] 可以同时接受 Vec<u32> 和 [u32; N]

fn sum(values: &[u32]) -> u32 {
    let mut total = 0;
    for v in values {
        total += v;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn empty() { assert_eq!(sum(&[]), 0); }
    #[test] fn one_element() { assert_eq!(sum(&[1]), 1); }
    #[test] fn multiple_elements() { assert_eq!(sum(&[1, 2, 3, 4, 5]), 15); }
    #[test] fn array_slice() { assert_eq!(sum(&[1, 2, 3, 4, 5]), 15); }
}
