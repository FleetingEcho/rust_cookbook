// 🔑 要点：Rust 的所有权规则
// - 每个值同时只能有一个所有者
// - 方法可以获取 self 所有权、借用 &self 或可变借用 &mut self
// - 借用（&self）不转移所有权，调用后原变量仍可用

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

    // 🐛 原代码：pub fn title(self) -> String { self.title }
    // 这行获取了 self 的所有权，调用后 ticket 被消耗掉
    // ✅ 改为 &self，不获取所有权，只借用一个引用
    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn description(&self) -> &String {
        &self.description
    }

    pub fn status(&self) -> &String {
        &self.status
    }
}

#[cfg(test)]
mod tests {
    use super::Ticket;

    #[test]
    fn works() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());
        // 因为 getter 用 &self，我们可以连续调用多次
        // ticket 的所有权没有被转移，仍然有效
        assert_eq!(ticket.title(), "A title");
        assert_eq!(ticket.description(), "A description");
        assert_eq!(ticket.status(), "To-Do");
    }
}
