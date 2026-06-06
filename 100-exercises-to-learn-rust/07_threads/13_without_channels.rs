// 🔑 要点：无需通道——直接通过 Arc<RwLock<Store>> 共享状态
// 线程直接获取锁来操作共享数据

use std::sync::{Arc, RwLock};

// 共享的 TicketStore
pub struct TicketStore {
    tickets: Vec<String>,
}

impl TicketStore {
    pub fn add_ticket(&mut self, draft: String) -> u64 {
        self.tickets.push(draft);
        self.tickets.len() as u64 - 1
    }
    pub fn get(&self, id: u64) -> Option<&String> {
        self.tickets.get(id as usize)
    }
}

#[test]
fn works() {
    use std::thread::spawn;
    let store = Arc::new(RwLock::new(TicketStore {
        tickets: Vec::new(),
    }));

    let store1 = store.clone();
    let t1 = spawn(move || store1.write().unwrap().add_ticket("hello".into()));

    let store2 = store.clone();
    let t2 = spawn(move || store2.write().unwrap().add_ticket("world".into()));

    let id1 = t1.join().unwrap();
    let id2 = t2.join().unwrap();

    let reader = store.read().unwrap();
    assert_eq!(*reader.get(id1).unwrap(), "hello");
    assert_eq!(*reader.get(id2).unwrap(), "world");
}
