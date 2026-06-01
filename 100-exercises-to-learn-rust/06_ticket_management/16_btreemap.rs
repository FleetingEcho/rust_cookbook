// 🔑 要点：BTreeMap 按键排序，适合需要有序遍历的场景
// IntoIterator for &TicketStore → 按 TicketId 顺序遍历

use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

type TicketTitle = String;
type TicketDescription = String;
fn ticket_title() -> TicketTitle { "A title".into() }
fn ticket_description() -> TicketDescription { "A description".into() }

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TicketId(u64);
#[derive(Clone)]
pub struct TicketStore { tickets: BTreeMap<TicketId, Ticket>, counter: u64 }
#[derive(Clone, Debug, PartialEq)]
pub struct Ticket { pub id: TicketId, pub title: TicketTitle, pub description: TicketDescription, pub status: Status }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TicketDraft { pub title: TicketTitle, pub description: TicketDescription }
#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum Status { ToDo, InProgress, Done }

impl TicketStore {
    pub fn new() -> Self { Self { tickets: BTreeMap::new(), counter: 0 } }
    pub fn add_ticket(&mut self, draft: TicketDraft) -> TicketId {
        let id = TicketId(self.counter); self.counter += 1;
        self.tickets.insert(id, Ticket { id, title: draft.title, description: draft.description, status: Status::ToDo });
        id
    }
    pub fn get(&self, id: TicketId) -> Option<&Ticket> { self.tickets.get(&id) }
    pub fn get_mut(&mut self, id: TicketId) -> Option<&mut Ticket> { self.tickets.get_mut(&id) }
}

impl Index<TicketId> for TicketStore { type Output = Ticket; fn index(&self, index: TicketId) -> &Self::Output { self.get(index).unwrap() } }
impl Index<&TicketId> for TicketStore { type Output = Ticket; fn index(&self, index: &TicketId) -> &Self::Output { &self[*index] } }
impl IndexMut<TicketId> for TicketStore { fn index_mut(&mut self, index: TicketId) -> &mut Self::Output { self.get_mut(index).unwrap() } }
impl IndexMut<&TicketId> for TicketStore { fn index_mut(&mut self, index: &TicketId) -> &mut Self::Output { &mut self[*index] } }

// IntoIterator for &TicketStore → BTreeMap 已按 key 排序
impl<'a> IntoIterator for &'a TicketStore {
    type Item = &'a Ticket;
    type IntoIter = std::collections::btree_map::Values<'a, TicketId, Ticket>;
    fn into_iter(self) -> Self::IntoIter { self.tickets.values() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn works() {
        let mut store = TicketStore::new();
        for _ in 0..5 {
            let id = store.add_ticket(TicketDraft { title: ticket_title(), description: ticket_description() });
            store[id].status = Status::InProgress;
        }
        let ids: Vec<TicketId> = (&store).into_iter().map(|t| t.id).collect();
        let mut sorted = ids.clone(); sorted.sort();
        assert_eq!(ids, sorted);
    }
}
