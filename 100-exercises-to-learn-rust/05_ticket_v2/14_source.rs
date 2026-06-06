// 🔑 要点：Error::source() 可以返回底层错误，形成错误链
// 模块分割：将代码拆分到多个文件中（这里内联为 mod）

mod status {
    use std::convert::TryFrom;

    #[derive(Debug, PartialEq, Clone)]
    pub enum Status {
        ToDo,
        InProgress,
        Done,
    }

    #[derive(Debug)]
    pub struct ParseStatusError {
        pub invalid_status: String,
    }

    impl std::fmt::Display for ParseStatusError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(
                f,
                "`{}` is not a valid status. Use one of: ToDo, InProgress, Done",
                self.invalid_status
            )
        }
    }
    impl std::error::Error for ParseStatusError {}

    impl TryFrom<String> for Status {
        type Error = ParseStatusError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            match value.to_lowercase().as_str() {
                "todo" => Ok(Status::ToDo),
                "inprogress" => Ok(Status::InProgress),
                "done" => Ok(Status::Done),
                _ => Err(ParseStatusError {
                    invalid_status: value,
                }),
            }
        }
    }
}

use status::Status;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum TicketNewError {
    TitleCannotBeEmpty,
    TitleTooLong,
    DescriptionCannotBeEmpty,
    DescriptionTooLong,
    InvalidStatus(Box<dyn Error + Send + Sync>), // 包含 source
}

impl fmt::Display for TicketNewError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TitleCannotBeEmpty => write!(f, "Title cannot be empty"),
            Self::TitleTooLong => write!(f, "Title cannot be longer than 50 bytes"),
            Self::DescriptionCannotBeEmpty => write!(f, "Description cannot be empty"),
            Self::DescriptionTooLong => write!(f, "Description cannot be longer than 500 bytes"),
            Self::InvalidStatus(_) => write!(f, "Invalid status"),
        }
    }
}

impl Error for TicketNewError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidStatus(e) => Some(e.as_ref()),
            _ => None,
        }
    }
}

fn valid_title() -> String {
    "A title".into()
}
fn valid_description() -> String {
    "A description".into()
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status_str: String,
    ) -> Result<Self, TicketNewError> {
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
        // 解析状态字符串
        let status =
            Status::try_from(status_str).map_err(|e| TicketNewError::InvalidStatus(Box::new(e)))?;
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
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert!(err.source().is_some());
    }
}
