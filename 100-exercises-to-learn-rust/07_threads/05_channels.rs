// 🔑 要点：mpsc 通道——多生产者单消费者
// Sender 发送，Receiver 接收
// 服务器线程持续处理命令

use std::sync::mpsc::{Receiver, Sender};

// 简化类型
type Ticket = String;
type TicketDraft = String;

pub enum Command {
    Insert(TicketDraft),
}

pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

pub fn server(receiver: Receiver<Command>) {
    loop {
        match receiver.recv() {
            Ok(_cmd) => { /* 处理命令 */ }
            Err(_) => break,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{launch, Command};
    #[test]
    fn a_thread_is_spawned() {
        let sender = launch();
        sender
            .send(Command::Insert("test".into()))
            .expect("Channel closed!");
    }
    #[test]
    fn ready() {
        let move_forward = true;
        assert!(move_forward);
    }
}
