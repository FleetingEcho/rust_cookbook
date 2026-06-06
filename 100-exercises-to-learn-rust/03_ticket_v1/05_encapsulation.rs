// 🔑 要点：通过 pub 方法暴露内部数据（getter）
// 这是封装（encapsulation）的基础模式

pub mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }
            Ticket {
                title,
                description,
                status,
            }
        }

        // TODO: 添加三个 pub 方法作为 getter
        // 💡 返回 &String 而非 String，避免所有权转移
        pub fn title(&self) -> &String {
            &self.title
        }

        pub fn description(&self) -> &String {
            &self.description
        }

        pub fn status(&self) -> &String {
            &self.status
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ticket::Ticket;

    #[test]
    fn description() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.description(), "A description");
    }

    #[test]
    fn title() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.title(), "A title");
    }

    #[test]
    fn status() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        assert_eq!(ticket.status(), "To-Do");
    }
}
