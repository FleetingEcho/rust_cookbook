/*
Queue 队列
┌─────────────────────────┐
│ first_half: [1,2,3,4,5] │
│ second_half: [6,7,8,9,10]│
└─────────────────────────┘
           │
           ↓
     send_tx() 函数
           │
    ┌──────┴──────┐
    ↓             ↓
 线程1          线程2
处理前半        处理后一半
[1,2,3,4,5]   [6,7,8,9,10]
    │             │
    └──────┬──────┘
           ↓
      tx.send() 发送
           ↓
     通道 (Channel)
           ↓
      rx.recv() 接收
           ↓
    received 向量
    [1,2,3,4,5,6,7,8,9,10]

*/
use std::{sync::mpsc, thread, time::Duration};

struct Queue {
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Self {
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

fn send_tx(q: Queue, tx: mpsc::Sender<u32>) {
    // TODO: We want to send `tx` to both threads. But currently, it is moved
    // into the first thread. How could you solve this problem?

    let tx2 = tx.clone();

    thread::spawn(move || {
        for val in q.first_half {
            println!("Sending {val:?}");
            tx.send(val).unwrap(); // 把数字扔进通道
            thread::sleep(Duration::from_millis(250));
        }
    });

    thread::spawn(move || {
        for val in q.second_half {
            println!("Sending {val:?}");
            tx2.send(val).unwrap(); // 把数字扔进通道
            thread::sleep(Duration::from_millis(250));
        }
    });
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn threads3() {
        let (tx, rx) = mpsc::channel();
        let queue = Queue::new();

        send_tx(queue, tx);

        let mut received = Vec::with_capacity(10);
        for value in rx {
            // rx 是迭代器，会自动等待新数据
            // 当所有 Sender 都销毁时，循环结束
            received.push(value);
        }

        received.sort();
        assert_eq!(received, [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    }
}
