// 🔑 要点：变量声明用 `let`，默认不可变
// Rust 的类型推理可以自动推断变量类型

/// 给定行程的起点、终点和耗时，计算平均速度
pub fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 声明一个 distance 变量，值为 end - start
    // 💡 Rust 可以自动推断 distance 的类型为 u32
    let distance = end - start;

    // 不要修改下面这行
    distance / time_elapsed
}

// 📝 注意：如果 end < start 会 panic（因为 u32 不能为负）
// 这是 Rust 的安全性设计：无符号整数溢出在 debug 模式下会 panic

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    fn case2() {
        assert_eq!(speed(10, 30, 10), 2);
    }

    #[test]
    fn case3() {
        // 整数除法会截断小数部分：20/10=2，而不是 2.1
        assert_eq!(speed(10, 31, 10), 2);
    }
}
