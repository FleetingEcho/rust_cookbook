// 🔑 要点：IndexMut trait 支持可变索引 store[id] = ...

use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

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
        let id = TicketId(self.counter); self.counter += 1;
        self.tickets.push(Ticket { id, title: draft.title, description: draft.description, status: Status::ToDo });
        id
    }
    pub fn get(&self, id: TicketId) -> Option<&Ticket> { self.tickets.iter().find(|t| t.id == id) }
    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> { self.tickets.iter_mut().find(|t| t.id == id) }
}

impl Index<TicketId> for TicketStore { type Output = Ticket; fn index(&self, index: TicketId) -> &Self::Output { self.get(index).unwrap() } }
impl Index<&TicketId> for TicketStore { type Output = Ticket; fn index(&self, index: &TicketId) -> &Self::Output { &self[*index] } }
impl IndexMut<TicketId> for TicketStore { fn index_mut(&mut self, index: TicketId) -> &mut Self::Output { self.get_mut(index).unwrap() } }
impl IndexMut<&TicketId> for TicketStore { fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output { &mut self[*index] } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn works() {
        let mut store = TicketStore::new();
        let id = store.add_ticket(TicketDraft { title: ticket_title(), description: ticket_description() });
        let ticket = &mut store[id];
        ticket.status = Status::InProgress;
        assert_eq!(store[id].status, Status::InProgress);
    }
}
