// 🔑 要点：str 的 trim() 方法可以去除首尾空白
// trim() 是在 str 上定义的，不是 String
// 通过 Deref 强制转换，String 自动获取 str 的方法

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        // 返回去除首尾空白后的标题
        // 💡 trim() 是 str 的方法，返回 &str
        self.title.trim()
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
