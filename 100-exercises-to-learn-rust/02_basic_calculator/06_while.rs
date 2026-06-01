// 🔑 要点：while 循环是 Rust 最基本的循环结构
// 用 while 重写阶乘，避免递归的栈溢出风险

// 使用 while 循环实现阶乘
pub fn factorial(n: u32) -> u32 {
    let mut result = 1;    // 用 mut 声明可变变量
    let mut i = 1;         // 计数器

    // while 循环：当条件为真时持续执行
    while i <= n {
        result *= i;
        i += 1;            // 注意：Rust 没有 i++ 运算符
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);   // 循环不会执行，result 保持为 1
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
