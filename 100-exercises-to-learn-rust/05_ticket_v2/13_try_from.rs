// 🔑 要点：TryFrom 用于可能失败的转换
// 与 From 不同，TryFrom 返回 Result

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TryFrom<String> for Status {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        // 不区分大小写
        match value.to_lowercase().as_str() {
            "todo" => Ok(Status::ToDo),
            "inprogress" => Ok(Status::InProgress),
            "done" => Ok(Status::Done),
            _ => Err(format!("`{}` is not a valid status", value)),
        }
    }
}

impl TryFrom<&str> for Status {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Status::try_from(value.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_string() {
        assert_eq!(Status::try_from("ToDO".to_string()).unwrap(), Status::ToDo);
        assert_eq!(
            Status::try_from("inproGress".to_string()).unwrap(),
            Status::InProgress
        );
        assert_eq!(Status::try_from("Done".to_string()).unwrap(), Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        assert_eq!(Status::try_from("todo").unwrap(), Status::ToDo);
        assert_eq!(Status::try_from("inprogress").unwrap(), Status::InProgress);
        assert_eq!(Status::try_from("done").unwrap(), Status::Done);
    }
}
