// 🔑 要点：enum 枚举类型——可以取多个不同值之一
// 这里用 Status 枚举替代字符串表示状态
// derive 要求所有字段也实现相应的 trait

fn valid_title() -> String { "A title".into() }
fn valid_description() -> String { "A description".into() }

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,   // 改为 Status 类型
}

#[derive(Debug, PartialEq)]
enum Status {
    ToDo,
    InProgress,
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

    pub fn title(&self) -> &String { &self.title }
    pub fn description(&self) -> &String { &self.description }
    pub fn status(&self) -> &Status { &self.status }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        let ticket1 = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        let ticket2 = Ticket::new(valid_title(), valid_description(), Status::ToDo);
        assert_eq!(ticket1, ticket2);
    }

    #[test]
    fn test_description_not_matching() {
        let ticket1 = Ticket { title: valid_title(), description: "description".into(), status: Status::ToDo };
        let ticket2 = Ticket { title: valid_title(), description: "description2".into(), status: Status::ToDo };
        assert_ne!(ticket1, ticket2);
    }
}
