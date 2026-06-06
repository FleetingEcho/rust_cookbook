// 🔑 要点：std::error::Error trait — Rust 错误处理的核心 trait
// 需要同时实现 Debug 和 Display
// Error 可以通过 source() 提供错误链

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

use std::fmt;

#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

// 实现 Display
impl fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketNewError::TitleError(msg) => write!(f, "{}", msg),
            TicketNewError::DescriptionError(msg) => write!(f, "{}", msg),
        }
    }
}

// 实现 std::error::Error
impl std::error::Error for TicketNewError {}

fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    if title.is_empty() {
        panic!("Title cannot be empty");
    }
    if title.len() > 50 {
        panic!("Title cannot be longer than 50 bytes");
    }
    let desc = if description.is_empty() || description.len() > 500 {
        "Description not provided".to_string()
    } else {
        description
    };
    Ticket::new(title, desc, status).unwrap()
}

#[derive(Debug, PartialEq, Clone)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError("Title cannot be empty".into()));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError(
                "Title cannot be longer than 50 bytes".into(),
            ));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be empty".into(),
            ));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError(
                "Description cannot be longer than 500 bytes".into(),
            ));
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

    #[test]
    fn display_is_correctly_implemented() {
        let err = Ticket::new("".into(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(format!("{}", err), "Title cannot be empty");
    }

    // 💡 TicketNewError 实现了 std::error::Error（编译器已保证）
}
