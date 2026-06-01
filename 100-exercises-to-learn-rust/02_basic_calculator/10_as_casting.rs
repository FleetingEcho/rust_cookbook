// 🔑 要点：as 关键字用于类型转换（casting）
// Rust 的类型转换必须显式进行，不会隐式转换
// 注意：大范围类型到小范围类型可能会截断

#[cfg(test)]
mod tests {

    #[test]
    fn u16_to_u32() {
        // u16 → u32：安全提升，不会丢失数据
        let v: u32 = 47u16 as u32;
        assert_eq!(47u16 as u32, v);
    }

    #[test]
    fn u8_to_i8() {
        // u8 → i8：可能丢失数据！u8 的 255 在 i8 中是 -1
        // 编译器会警告 overflowing_literals
        #[allow(overflowing_literals)]
        let x = { 255 as i8 };

        // y 应该等于 x，即 -1 的 i8 表示
        // 💡 -1i8 作为 u8 的位模式是 255（补码表示）
        let y: i8 = -1;

        assert_eq!(x, y);
    }

    #[test]
    fn bool_to_u8() {
        // bool → 整数：true = 1, false = 0
        let v: u8 = 1;
        assert_eq!(true as u8, v);
    }
}
