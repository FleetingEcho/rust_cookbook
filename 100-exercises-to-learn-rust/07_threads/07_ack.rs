// 🔑 要点：使用响应通道（response channel）实现请求-响应模式
// 每个命令带一个 Sender 用于发送响应

use std::sync::mpsc::{channel, Receiver, Sender};

pub enum Command {
    Insert {
        draft: String,
        response_channel: Sender<u64>,
    },
    Get {
        id: u64,
        response_channel: Sender<Option<String>>,
    },
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = channel();
    std::thread::spawn(move || server(receiver));
    sender
}

fn server(receiver: Receiver<Command>) {
    let mut store: Vec<String> = Vec::new();
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
