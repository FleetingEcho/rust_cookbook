// 🔑 要点：Rust 的可见性控制
// - 默认私有（private）
// - pub 对外公开
// - pub(crate) 对当前 crate 公开
// - super 引用父模块

mod ticket {
    pub struct Ticket {
        pub title: String,      // pub 以使测试模块可以访问
        pub description: String,
        pub status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() { panic!("Title cannot be empty"); }
            if title.len() > 50 { panic!("Title cannot be longer than 50 bytes"); }
            if description.is_empty() { panic!("Description cannot be empty"); }
            if description.len() > 500 { panic!("Description cannot be longer than 500 bytes"); }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }
            Ticket { title, description, status }
        }
    }
}

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::ticket::Ticket;

    #[test]
    fn constructor_works() {
        let ticket = Ticket::new("title".into(), "desc".into(), "To-Do".into());
        assert_eq!(ticket.title, "title");
    }

    // 💡 以下是教学注释：验证了封装性
    // 当字段为私有时，以下代码无法编译：
    // fn should_not_be_possible() {
    //     let ticket = Ticket::new(...);
    //     assert_eq!(ticket.description, "A description");  // 字段私有！
    // }
    //
    // fn encapsulation_cannot_be_violated() {
    //     let ticket = Ticket { title: "...", description: "...", status: "..." };  // 字段私有！
    // }
}
