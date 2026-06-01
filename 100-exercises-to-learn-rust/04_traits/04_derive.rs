// 🔑 要点：derive 宏可以自动实现标准 trait
// #[derive(Debug)] 可以让类型支持 {:?} 格式化输出
// assert_eq! 要求两个参数都实现 Debug（失败时打印它们）

// 添加 Debug derive：
#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: String,
}

// 💡 derive(Debug) 会自动生成 Debug trait 的实现代码
// 💡 derive(PartialEq) 会逐字段比较

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
        assert_eq!(ticket1, ticket2);
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
        assert_ne!(ticket1, ticket2);
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
        assert_ne!(ticket1, ticket2);
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
        assert_ne!(ticket1, ticket2);
    }
}
