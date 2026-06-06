// 🔑 要点：thiserror 是常用的错误处理库
// 通过 #[derive(Error)] 和 #[error("...")] 自动实现 Error + Display
// 这里用手动实现代替（独立文件无需外部依赖）

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

enum TicketNewError {
    TitleCannotBeEmpty,
    TitleTooLong,
    DescriptionCannotBeEmpty,
    DescriptionTooLong,
}

impl fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TicketNewError::TitleCannotBeEmpty => write!(f, "Title cannot be empty"),
            TicketNewError::TitleTooLong => write!(f, "Title cannot be longer than 50 bytes"),
            TicketNewError::DescriptionCannotBeEmpty => write!(f, "Description cannot be empty"),
            TicketNewError::DescriptionTooLong => {
                write!(f, "Description cannot be longer than 500 bytes")
            }
        }
    }
}

impl fmt::Debug for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for TicketNewError {}

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
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
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
        assert_eq!(err.to_string(), "Title cannot be empty");
    }
    #[test]
    fn description_cannot_be_empty() {
        let err = Ticket::new(valid_title(), "".into(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Description cannot be empty");
    }
    #[test]
    fn title_cannot_be_longer_than_fifty_chars() {
        let err = Ticket::new(overly_long_title(), valid_description(), Status::ToDo).unwrap_err();
        assert_eq!(err.to_string(), "Title cannot be longer than 50 bytes");
    }
    #[test]
    fn description_cannot_be_too_long() {
        let err = Ticket::new(valid_title(), overly_long_description(), Status::ToDo).unwrap_err();
        assert_eq!(
            err.to_string(),
            "Description cannot be longer than 500 bytes"
        );
    }
}
