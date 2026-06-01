// 🔑 要点：引用（指针）在 64 位系统上占用 8 字节
// 无论引用的是什么类型，引用本身的大小都是固定的
// &T、&mut T 和 &Ticket 在 64 位系统上都是 8 字节

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
    fn u16_ref_size() {
        // 64 位系统上，引用占用 8 字节
        assert_eq!(size_of::<&u16>(), 8);
    }

    #[test]
    fn u64_mut_ref_size() {
        // 可变引用也是 8 字节
        assert_eq!(size_of::<&mut u64>(), 8);
    }

    #[test]
    fn ticket_ref_size() {
        // 无论被引用类型多大，引用本身都是 8 字节
        assert_eq!(size_of::<&Ticket>(), 8);
    }
}
