// 🔑 要点：add_ticket 接收 TicketDraft，返回 TicketId
// get 根据 id 获取 Option<&Ticket>

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

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone)]
pub struct TicketStore { tickets: Vec<Ticket>, counter: u64 }

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket { pub id: TicketId, pub title: TicketTitle, pub description: TicketDescription, pub status: Status }

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft { pub title: TicketTitle, pub description: TicketDescription }

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status { ToDo, InProgress, Done }

impl TicketStore {
    pub fn new() -> Self { Self { tickets: Vec::new(), counter: 0 } }

    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        let ticket = Ticket { id, title: draft.title, description: draft.description, status: Status::ToDo };
        self.tickets.push(ticket);
        id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn works() {
        let mut store = TicketStore::new();
        let draft1 = TicketDraft { title: ticket_title(), description: ticket_description() };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(ticket1.status, Status::ToDo);
        let draft2 = TicketDraft { title: ticket_title(), description: ticket_description() };
        let id2 = store.add_ticket(draft2);
        assert_ne!(id1, id2);
    }
}
