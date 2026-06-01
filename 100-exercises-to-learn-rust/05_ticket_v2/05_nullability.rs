// 🔑 要点：Option<T> 代替 panic，表示"可能有值"
// Some(T) → 有值；None → 无值
// 比 panic 更安全，编译器强制调用方处理 None 情况

fn valid_title() -> String { "A title".into() }
fn valid_description() -> String { "A description".into() }

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Ticket {
        if title.is_empty() { panic!("Title cannot be empty"); }
        if title.len() > 50 { panic!("Title cannot be longer than 50 bytes"); }
        if description.is_empty() { panic!("Description cannot be empty"); }
        if description.len() > 500 { panic!("Description cannot be longer than 500 bytes"); }
        Ticket { title, description, status }
    }

    pub fn assigned_to(&self) -> Option<&String> {
        match &self.status {
            Status::InProgress { assigned_to } => Some(assigned_to),
            _ => None,    // 非 InProgress 返回 None，而非 panic
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_todo() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        assert!(ticket.assigned_to().is_none());
    }

    #[test]
    fn test_done() {
        let ticket = Ticket::new(valid_title(), valid_description(), Status::Done);
        assert!(ticket.assigned_to().is_none());
    }

    #[test]
    fn test_in_progress() {
        let ticket = Ticket::new(valid_title(), valid_description(),
            Status::InProgress { assigned_to: "Alice".to_string() });
        assert_eq!(ticket.assigned_to(), Some(&"Alice".to_string()));
    }
}
