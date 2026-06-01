// 🔑 要点：impl Trait 语法——返回迭代器而不暴露具体类型
// store.in_progress() 返回 impl Iterator<Item = &Ticket>

use std::convert::TryFrom;
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TicketTitle(String);
impl TryFrom<String> for TicketTitle { type Error = String; fn try_from(value: String) -> Result<Self, Self::Error> { Ok(Self(value)) } }
impl TryFrom<&str> for TicketTitle { type Error = String; fn try_from(value: &str) -> Result<Self, Self::Error> { Ok(Self(value.to_string())) } }
#[derive(Debug, PartialEq, Clone, Eq)]
pub struct TicketDescription(String);
impl TryFrom<String> for TicketDescription { type Error = String; fn try_from(value: String) -> Result<Self, Self::Error> { Ok(Self(value)) } }
impl TryFrom<&str> for TicketDescription { type Error = String; fn try_from(value: &str) -> Result<Self, Self::Error> { Ok(Self(value.to_string())) } }
pub fn ticket_title() -> TicketTitle { "A title".try_into().unwrap() }
pub fn ticket_description() -> TicketDescription { "A description".try_into().unwrap() }

#[derive(Clone)]
pub struct TicketStore { tickets: Vec<Ticket> }
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket { pub title: TicketTitle, pub description: TicketDescription, pub status: Status }
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status { ToDo, InProgress, Done }

impl TicketStore {
    pub fn new() -> Self { Self { tickets: Vec::new() } }
    pub fn add_ticket(&mut self, ticket: Ticket) { self.tickets.push(ticket); }

    // impl Trait 隐藏了具体迭代器类型
    pub fn in_progress(&self) -> impl Iterator<Item = &Ticket> {
        self.tickets.iter().filter(|t| t.status == Status::InProgress)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn in_progress() {
        let mut store = TicketStore::new();
        store.add_ticket(Ticket { title: ticket_title(), description: ticket_description(), status: Status::ToDo });
        let ip = Ticket { title: ticket_title(), description: ticket_description(), status: Status::InProgress };
        store.add_ticket(ip.clone());
        let tickets: Vec<&Ticket> = store.in_progress().collect();
        assert_eq!(tickets.len(), 1);
        assert_eq!(tickets[0], &ip);
    }
}
