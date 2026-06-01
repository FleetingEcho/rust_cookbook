// 🔑 要点：&String 和 &str 的区别
// &String 是对 String 的引用
// &str 是字符串切片，更通用（可以引用 String 的一部分或 &str 字面量）
// Rust 推荐在函数参数中使用 &str 而非 &String（Deref 强制转换）

fn valid_title() -> String { "A title".into() }
fn valid_description() -> String { "A description".into() }

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Ticket {
        if title.is_empty() { panic!("Title cannot be empty"); }
        if title.len() > 50 { panic!("Title cannot be longer than 50 bytes"); }
        if description.is_empty() { panic!("Description cannot be empty"); }
        if description.len() > 500 { panic!("Description cannot be longer than 500 bytes"); }
        if status != "To-Do" && status != "In Progress" && status != "Done" {
            panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
        }
        Ticket { title, description, status }
    }

    // 返回 &str 而不是 &String，更通用 💡
    pub fn title(&self) -> &str {
        // &String 通过 Deref 自动转换为 &str
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &str {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::{Any, TypeId};

    #[test]
    fn test_type() {
        let ticket = Ticket::new(valid_title(), valid_description(), "To-Do".to_string());
        // 验证返回类型确实是 &str 而不是 &String
        assert_eq!(TypeId::of::<str>(), ticket.title().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.description().type_id());
        assert_eq!(TypeId::of::<str>(), ticket.status().type_id());
    }
}
