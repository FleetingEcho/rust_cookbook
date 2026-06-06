// 🔑 要点：枚举变体可以携带数据
// InProgress { assigned_to: String } 是一个带命名字段的变体
// match 时可以通过模式匹配提取这些数据

fn valid_title() -> String {
    "A title".into()
}
fn valid_description() -> String {
    "A description".into()
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String }, // 携带指派人的名字
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
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
        Ticket {
            title,
            description,
            status,
        }
    }

    pub fn assigned_to(&self) -> &str {
        // 只有 InProgress 状态才有指派人
        match &self.status {
            Status::InProgress { assigned_to } => assigned_to,
            _ => panic!("Only `In-Progress` tickets can be assigned to someone"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_todo() {
        Ticket::new(valid_title(), valid_description(), Status::ToDo).assigned_to();
    }

    #[test]
    #[should_panic(expected = "Only `In-Progress` tickets can be assigned to someone")]
    fn test_done() {
        Ticket::new(valid_title(), valid_description(), Status::Done).assigned_to();
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(
            valid_title(),
            valid_description(),
            Status::InProgress {
                assigned_to: "Alice".to_string(),
            },
        );
        assert_eq!(ticket.assigned_to(), "Alice");
    }
}
