// 🔑 要点：泛型参数 vs impl Trait
// 这里把 add_ticket 的参数从 impl Into<Ticket> 改为泛型 T: Into<Ticket>

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

    // 改为泛型参数 T: Into<Ticket>
    pub fn add_ticket<T: Into<Ticket>>(&mut self, ticket: T) {
        self.tickets.push(ticket.into());
    }
}

impl From<TicketDraft> for Ticket {
    fn from(draft: TicketDraft) -> Self {
        Self { title: draft.title, description: draft.description, status: Status::ToDo }
    }
}

struct TicketDraft { pub title: TicketTitle, pub description: TicketDescription }

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn generic_add() {
        let mut store = TicketStore::new();
        store.add_ticket::<TicketDraft>(TicketDraft { title: ticket_title(), description: ticket_description() });
    }
}
