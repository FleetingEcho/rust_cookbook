// 🔑 要点：sync_channel 有界通道——容量有限的缓冲区
// try_send 在通道满时返回 TrySendError

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};

pub struct TicketStoreClient {
    sender: SyncSender<Command>,
}

enum Command {
    Insert {
        draft: String,
        response_channel: SyncSender<u64>,
    },
    Get {
        id: u64,
        response_channel: SyncSender<Option<String>>,
    },
}

impl TicketStoreClient {
    pub fn insert(&self, draft: String) -> Result<u64, String> {
        let (resp_s, resp_r) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: resp_s,
            })
            .map_err(|_| "Overloaded".to_string())?;
        Ok(resp_r.recv().unwrap())
    }
    pub fn get(&self, id: u64) -> Result<Option<String>, String> {
        let (resp_s, resp_r) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: resp_s,
            })
            .map_err(|_| "Overloaded".to_string())?;
        Ok(resp_r.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

fn server(receiver: Receiver<Command>) {
    let mut store = Vec::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                store.push(draft);
                let _ = response_channel.send(store.len() as u64 - 1);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let _ = response_channel.send(store.get(id as usize).cloned());
            }
            Err(_) => break,
        }
    }
}

#[test]
fn works() {
    let client = launch(5);
    let id = client.insert("hello".into()).unwrap();
    assert_eq!(client.get(id).unwrap(), Some("hello".into()));
}
