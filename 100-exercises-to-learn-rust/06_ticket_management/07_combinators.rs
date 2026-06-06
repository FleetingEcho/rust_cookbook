// 🔑 要点：迭代器适配器（combinators）如 filter
// store.to_dos() 用 filter 过滤出 ToDo 状态的票

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
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
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

    // 用 filter 迭代器适配器过滤
    pub fn to_dos(&self) -> Vec<&Ticket> {
        self.tickets
            .iter()
            .filter(|t| t.status == Status::ToDo)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn todos() {
        let mut store = TicketStore::new();
        let todo = Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::ToDo,
        };
        store.add_ticket(todo.clone());
        store.add_ticket(Ticket {
            title: ticket_title(),
            description: ticket_description(),
            status: Status::InProgress,
        });
        let todos: Vec<&Ticket> = store.to_dos();
        assert_eq!(todos.len(), 1);
        assert_eq!(todos[0], &todo);
    }
}
