// 🔑 要点：客户端封装——隐藏通道细节
// TicketStoreClient 封装了 insert/get 的通信逻辑

use std::sync::mpsc::{channel, Receiver, Sender};

pub struct TicketStoreClient {
    sender: Sender<Command>,
}

enum Command {
    Insert {
        draft: String,
        response_channel: Sender<u64>,
    },
    Get {
        id: u64,
        response_channel: Sender<Option<String>>,
    },
}

impl TicketStoreClient {
    pub fn insert(&self, draft: String) -> u64 {
        let (resp_s, resp_r) = channel();
        self.sender
            .send(Command::Insert {
                draft,
                response_channel: resp_s,
            })
            .unwrap();
        resp_r.recv().unwrap()
    }
    pub fn get(&self, id: u64) -> Option<String> {
        let (resp_s, resp_r) = channel();
        self.sender
            .send(Command::Get {
                id,
                response_channel: resp_s,
            })
            .unwrap();
        resp_r.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = channel();
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
fn client_works() {
    let client = launch();
    let id = client.insert("hello".into());
    assert_eq!(client.get(id), Some("hello".into()));
}
