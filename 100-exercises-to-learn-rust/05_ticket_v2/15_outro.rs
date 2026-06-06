// 🔑 要点：最终结构——每个字段有自己的验证类型
// 使用模块组织代码 + 重导出（re-export）模式
// 字段不再需要私有，因为类型本身保证了有效性

mod description {
    use std::convert::TryFrom;
    #[derive(Debug, PartialEq, Clone)]
    pub struct TicketDescription(String);
    #[derive(Debug)]
    pub enum TicketDescriptionError {
        Empty,
        TooLong,
    }

    impl std::fmt::Display for TicketDescriptionError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "The description cannot be empty"),
                Self::TooLong => write!(f, "The description cannot be longer than 500 bytes"),
            }
        }
    }
    impl std::error::Error for TicketDescriptionError {}

    impl TryFrom<String> for TicketDescription {
        type Error = TicketDescriptionError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.is_empty() {
                return Err(TicketDescriptionError::Empty);
            }
            if value.len() > 500 {
                return Err(TicketDescriptionError::TooLong);
            }
            Ok(Self(value))
        }
    }
    impl TryFrom<&str> for TicketDescription {
        type Error = TicketDescriptionError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.to_string().try_into()
        }
    }
}

mod title {
    use std::convert::TryFrom;
    #[derive(Debug, PartialEq, Clone)]
    pub struct TicketTitle(String);
    #[derive(Debug)]
    pub enum TicketTitleError {
        Empty,
        TooLong,
    }

    impl std::fmt::Display for TicketTitleError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Empty => write!(f, "The title cannot be empty"),
                Self::TooLong => write!(f, "The title cannot be longer than 50 bytes"),
            }
        }
    }
    impl std::error::Error for TicketTitleError {}

    impl TryFrom<String> for TicketTitle {
        type Error = TicketTitleError;
        fn try_from(value: String) -> Result<Self, Self::Error> {
            if value.is_empty() {
                return Err(TicketTitleError::Empty);
            }
            if value.len() > 50 {
                return Err(TicketTitleError::TooLong);
            }
            Ok(Self(value))
        }
    }
    impl TryFrom<&str> for TicketTitle {
        type Error = TicketTitleError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.to_string().try_into()
        }
    }
}

mod status_mod {
    use std::convert::TryFrom;
    #[derive(Debug, PartialEq, Clone)]
    pub enum Status {
        ToDo,
        InProgress,
        Done,
    }
    #[derive(Debug)]
    pub struct ParseStatusError {
        invalid_status: String,
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
    impl TryFrom<&str> for Status {
        type Error = ParseStatusError;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            value.to_string().try_into()
        }
    }
}

pub use description::TicketDescription;
pub use status_mod::Status;
pub use title::TicketTitle;

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
