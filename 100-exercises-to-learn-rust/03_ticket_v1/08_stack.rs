// 🔑 要点：Rust 的基本类型大小（栈上）
// 使用 std::mem::size_of 可以查看类型占用的字节数
// u16 = 2 字节, i32 = 4 字节, bool = 1 字节

#[cfg(test)]
mod tests {
    use std::mem::size_of;

    #[test]
    fn u16_size() {
        // u16 是无符号 16 位整数 → 2 字节
        assert_eq!(size_of::<u16>(), 2);
    }

    #[test]
    fn i32_size() {
        // i32 是有符号 32 位整数 → 4 字节
        assert_eq!(size_of::<i32>(), 4);
    }

    #[test]
    fn bool_size() {
        // bool 是布尔类型 → 1 字节
        assert_eq!(size_of::<bool>(), 1);
    }
}
