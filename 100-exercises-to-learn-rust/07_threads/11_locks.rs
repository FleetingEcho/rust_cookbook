// 🔑 要点：Mutex 互斥锁——保证同一时间只有一个线程能访问数据
// Arc<Mutex<T>> 组合：线程安全的多所有者 + 互斥访问

use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::{Arc, Mutex};

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
        response_channel: SyncSender<Option<Arc<Mutex<String>>>>,
    },
}

impl TicketStoreClient {
    pub fn insert(&self, draft: String) -> Result<u64, ()> {
        let (s, r) = sync_channel(1);
        self.sender
            .try_send(Command::Insert {
                draft,
                response_channel: s,
            })
            .map_err(|_| ())?;
        Ok(r.recv().unwrap())
    }
    pub fn get(&self, id: u64) -> Result<Option<Arc<Mutex<String>>>, ()> {
        let (s, r) = sync_channel(1);
        self.sender
            .try_send(Command::Get {
                id,
                response_channel: s,
            })
            .map_err(|_| ())?;
        Ok(r.recv().unwrap())
    }
}

pub fn launch(capacity: usize) -> TicketStoreClient {
    let (sender, receiver) = sync_channel(capacity);
    std::thread::spawn(move || server(receiver));
    TicketStoreClient { sender }
}

fn server(receiver: Receiver<Command>) {
    let mut store: Vec<Arc<Mutex<String>>> = Vec::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                store.push(Arc::new(Mutex::new(draft)));
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
    let ticket = client.get(id).unwrap().unwrap();
    assert_eq!(*ticket.lock().unwrap(), "hello");
}
