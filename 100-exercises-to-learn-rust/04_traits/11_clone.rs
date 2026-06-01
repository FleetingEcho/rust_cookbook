// 🔑 要点：Clone trait 用于显式复制
// 调用 .clone() 会深度复制数据
// 原对象和克隆对象各自拥有独立的所有权

// 为 Ticket 和 Summary 添加 Clone derive
#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Clone)]
pub struct Summary {
    pub title: String,
    pub status: String,
}

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    // 先 clone ticket，然后将克隆传给 summary()
    // 原 ticket 仍然有效
    (ticket.clone(), ticket.summary())
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}
