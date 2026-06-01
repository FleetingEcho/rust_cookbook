// 🔑 要点：panic! 宏用于不可恢复的错误，会立即终止程序
// 在测试中可以用 #[should_panic] 来验证 panic 行为

/// 给定行程的起点、终点和耗时，计算平均速度
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    // TODO: 如果 time_elapsed 为 0，用自定义消息 panic
    // 💡 panic! 宏支持格式化字符串
    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!");
    }

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    // #[should_panic] 可以验证函数按预期 panic
    // expected 参数检查 panic 消息是否包含指定文本
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
