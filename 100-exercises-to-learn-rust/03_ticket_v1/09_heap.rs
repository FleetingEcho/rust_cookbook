// 🔑 要点：String 在栈上只有 24 字节（3 个 usize）
// 栈上存储：指针(8B) + 长度(8B) + 容量(8B) = 24B
// 实际字符串数据存在堆上
// Ticket 包含 3 个 String，所以 24 * 3 = 72 字节

#[allow(dead_code)]
pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        // String 栈大小 = 24 字节 (ptr + len + capacity)
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // Ticket 包含 3 个 String，共 3 × 24 = 72 字节
        // struct 的大小是所有字段大小之和（可能还有对齐填充）
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
