// 🔑 要点：Update 命令——修改已有票证
// TicketPatch 可以部分更新字段

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

#[derive(Clone)]
pub struct TicketStoreClient { sender: SyncSender<Command> }

#[derive(Clone, Debug)]
pub struct TicketPatch { pub id: u64, pub status: Option<String> }

enum Command {
    Insert { draft: String, response_channel: SyncSender<u64> },
    Get { id: u64, response_channel: SyncSender<Option<String>> },
    Update { patch: TicketPatch, response_channel: SyncSender<()> },
}

impl TicketStoreClient {
    pub fn insert(&self, draft: String) -> Result<u64, ()> {
        let (s, r) = sync_channel(1);
        self.sender.try_send(Command::Insert { draft, response_channel: s }).map_err(|_| ())?;
        Ok(r.recv().unwrap())
    }
    pub fn get(&self, id: u64) -> Result<Option<String>, ()> {
        let (s, r) = sync_channel(1);
        self.sender.try_send(Command::Get { id, response_channel: s }).map_err(|_| ())?;
        Ok(r.recv().unwrap())
    }
    pub fn update(&self, patch: TicketPatch) -> Result<(), ()> {
        let (s, r) = sync_channel(1);
        self.sender.try_send(Command::Update { patch, response_channel: s }).map_err(|_| ())?;
        r.recv().map_err(|_| ())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

fn server(receiver: Receiver<Command>) {
    let mut store: Vec<String> = Vec::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert { draft, response_channel }) => {
                store.push(draft);
                let _ = response_channel.send(store.len() as u64 - 1);
            }
            Ok(Command::Get { id, response_channel }) => {
                let _ = response_channel.send(store.get(id as usize).cloned());
            }
            Ok(Command::Update { patch, response_channel }) => {
                if let Some(t) = store.get_mut(patch.id as usize) {
                    if patch.status.is_some() { /* update logic */ }
                }
                let _ = response_channel.send(());
            }
            Err(_) => break,
        }
    }
}

#[test] fn works() {
    let client = launch(5);
    let id = client.insert("test".into()).unwrap();
    client.update(TicketPatch { id, status: Some("Done".into()) }).unwrap();
}
