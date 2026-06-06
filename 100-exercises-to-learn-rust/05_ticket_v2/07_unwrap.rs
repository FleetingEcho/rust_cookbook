// 🔑 要点：unwrap 和 expect 用于快速处理 Result
// 成功时提取 Ok 值，失败时 panic
// 这里标题无效时 panic，描述无效时用默认值

fn overly_long_description() -> String {
    "x".repeat(501)
}
fn overly_long_title() -> String {
    "A title that's definitely longer than what should be allowed in a development ticket".into()
}
fn valid_title() -> String {
    "A title".into()
}
fn valid_description() -> String {
    "A description".into()
}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    // 先检查标题是否有效——无效则直接 panic
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if title.len() > 50 {
        panic!("Title cannot be longer than 50 bytes");
    }

    // 描述无效时使用默认值
    let desc = if description.is_empty() || description.len() > 500 {
        "Description not provided".to_string()
    } else {
        description
    };

    // 使用 unwrap：我们已确保所有参数有效
    Ticket::new(title, desc, status).unwrap()
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(title: String, description: String, status: Status) -> Result<Ticket, String> {
        if title.is_empty() {
            return Err("Title cannot be empty".to_string());
        }
        if title.len() > 50 {
            return Err("Title cannot be longer than 50 bytes".to_string());
        }
        if description.is_empty() {
            return Err("Description cannot be empty".to_string());
        }
        if description.len() > 500 {
            return Err("Description cannot be longer than 500 bytes".to_string());
        }
        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
