// 🔑 要点：递归函数需要显式声明返回类型
// 递归必须有终止条件（base case），否则会栈溢出

// 定义阶乘函数，使用**递归**实现
// 阶乘：n! = n * (n-1) * ... * 1，其中 0! = 1
fn factorial(n: u32) -> u32 {
    // 💡 base case: 0! = 1
    if n == 0 {
        1
    } else {
        // 递归调用：n! = n * (n-1)!
        n * factorial(n - 1)
    }
}

// ⚠️ 递归可能会栈溢出！Rust 没有尾递归优化保证
// 后续练习会用循环改写

#[cfg(test)]
mod tests {
    use crate::factorial;

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
