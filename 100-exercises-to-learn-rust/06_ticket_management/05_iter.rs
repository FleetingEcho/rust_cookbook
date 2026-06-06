// 🔑 要点：iter() 方法返回 &T 的迭代器
// Vec 的 iter() 返回 std::slice::Iter<T>

use std::convert::TryFrom;

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TicketTitle(String);
impl TryFrom<String> for TicketTitle {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
impl TryFrom<&str> for TicketTitle {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.to_string()))
    }
}

#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TicketDescription(String);
impl TryFrom<String> for TicketDescription {
    type Error = String;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value))
    }
}
impl TryFrom<&str> for TicketDescription {
    type Error = String;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self(value.to_string()))
    }
}

pub fn ticket_title() -> TicketTitle {
    "A title".try_into().unwrap()
}
pub fn ticket_description() -> TicketDescription {
    "A description".try_into().unwrap()
}

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ticket {
    title: TicketTitle,
    description: TicketDescription,
    status: Status,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
        }
    }
    pub fn add_ticket(&mut self, ticket: Ticket) {
        self.tickets.push(ticket);
    }

    // 添加 iter 方法：委托给 Vec::iter()
    pub fn iter(&self) -> std::slice::Iter<'_, Ticket> {
        self.tickets.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_ticket() {
        let mut store = TicketStore::new();
        store.add_ticket(Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        });
        store.add_ticket(Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        });
        let tickets: Vec<&Ticket> = store.iter().collect();
        let tickets2: Vec<&Ticket> = store.iter().collect();
        assert_eq!(tickets, tickets2);
    }
}
