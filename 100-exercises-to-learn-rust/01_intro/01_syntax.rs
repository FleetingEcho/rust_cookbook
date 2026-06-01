// 🔑 要点：Rust 是强类型语言，函数参数必须声明类型
// 编译器可以推理返回类型，但参数类型必须显式标注

fn compute(a: u32, b: u32) -> u32 {
    // 函数体不做修改，只修改签名
    a + b * 2
}

// Rust 的运算符优先级：b * 2 先计算，然后 a + 结果
// 所以 compute(1, 2) = 1 + (2 * 2) = 5

#[cfg(test)]
mod tests {
    use crate::compute;

    #[test]
    fn case() {
        assert_eq!(compute(1, 2), 5);
    }
}
