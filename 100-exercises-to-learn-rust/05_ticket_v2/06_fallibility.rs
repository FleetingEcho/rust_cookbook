// 🔑 要点：Result<T, E> 替代 panic，表示"可能失败"
// Ok(T) → 成功；Err(E) → 失败及错误信息
// 调用方可以通过 ? 或 match 处理错误

fn overly_long_description() -> String {
    /* ... */
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
    // 返回 Result 而非 panic
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
    fn title_cannot_be_empty() {
        let err = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err, "Title cannot be empty");
    }

    #[test]
    fn description_cannot_be_empty() {
        let err = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(err, "Description cannot be empty");
    }

    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let err = Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err, "Title cannot be longer than 50 bytes");
    }

    #[test]
    fn description_cannot_be_longer_than_500_chars() {
        let err = Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(err, "Description cannot be longer than 500 bytes");
    }
}
