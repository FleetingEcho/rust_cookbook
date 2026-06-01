// 🔑 要点：Rust 支持运算符重载，通过实现 std::ops 中的 trait
// PartialEq 是 == 和 != 运算符对应的 trait
// 实现后可以使用 == 直接比较 Ticket

struct Ticket {
    title: String,
    description: String,
    status: String,
}

// 为 Ticket 实现 PartialEq
impl PartialEq for Ticket {
    fn eq(&self, other: &Self) -> bool {
        // 比较所有字段是否相等
        self.title == other.title
            && self.description == other.description
            && self.status == other.status
    }
}

// 💡 也可以使用 #[derive(PartialEq)] 自动实现
// 这里手动实现是为了理解原理

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: "description".to_string(),
            status: "To-Do".to_string(),
        };
        let ticket2 = Ticket {
            title: "title".to_string(),
            description: "description".to_string(),
            status: "To-Do".to_string(),
        };
        assert!(ticket1 == ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: "description".to_string(),
            status: "To-Do".to_string(),
        };
        let ticket2 = Ticket {
            title: "title".to_string(),
            description: "description2".to_string(),
            status: "To-Do".to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_title_not_matching() {
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: "description".to_string(),
            status: "To-Do".to_string(),
        };
        let ticket2 = Ticket {
            title: "title2".to_string(),
            description: "description".to_string(),
            status: "To-Do".to_string(),
        };
        assert!(ticket1 != ticket2);
    }

    #[test]
    fn test_status_not_matching() {
        let ticket1 = Ticket {
            title: "title".to_string(),
            description: "description".to_string(),
            status: "status".to_string(),
        };
        let ticket2 = Ticket {
            title: "title".to_string(),
            description: "description".to_string(),
            status: "status2".to_string(),
        };
        assert!(ticket1 != ticket2);
    }
}
