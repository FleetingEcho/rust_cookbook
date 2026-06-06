// 🔑 要点：HashMap<K, V> 基于哈希表的键值对集合
// TicketId 需要实现 Hash + Eq 才能作为 HashMap 的键

use std::collections::HashMap;
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

// 使用简化内联类型
type TicketTitle = String;
type TicketDescription = String;
fn ticket_title() -> TicketTitle {
    "A title".into()
}
fn ticket_description() -> TicketDescription {
    "A description".into()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TicketId(u64);
#[derive(Clone)]
pub struct TicketStore {
    tickets: HashMap<TicketId, Ticket>,
    counter: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: HashMap::new(),
            counter: 0,
        }
    }
    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = TicketId(self.counter);
        self.counter += 1;
        self.tickets.insert(
            id,
            Ticket {
                id,
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            },
        );
        id
    }
    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.get(&id)
    }
    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> {
        self.tickets.get_mut(&id)
    }
}

impl Index<TicketId> for TicketStore {
    type Output = Ticket;
    fn index(&self, index: TicketId) -> &Self::Output {
        self.get(index).unwrap()
    }
}
impl Index<&TicketId> for TicketStore {
    type Output = Ticket;
    fn index(&self, index: &TicketId) -> &Self::Output {
        &self[*index]
    }
}
impl IndexMut<TicketId> for TicketStore {
    fn index_mut(&mut self, index: TicketId) -> &mut Self::Output {
        self.get_mut(index).unwrap()
    }
}
impl IndexMut<&TicketId> for TicketStore {
    fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output {
        &mut self[*index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn works() {
        let mut store = TicketStore::new();
        let id = store.add_ticket(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
        let ticket = &mut store[id];
        ticket.status = Status::InProgress;
        assert_eq!(store[id].status, Status::InProgress);
    }
}
