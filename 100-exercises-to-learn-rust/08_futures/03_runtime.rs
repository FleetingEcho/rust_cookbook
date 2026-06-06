// 🔑 要点：Arc 在线程间共享数据
// tokio::spawn 的闭包需要 'static 生命周期
// 共享数据用 Arc<Mutex<T>> 或 Arc<RwLock<T>>
// Requires: tokio (with "full" features), anyhow

// 实现 fixed_reply 函数，同时监听两个端口
// 收到连接后始终回复 reply 参数的 Display 表示
use std::fmt::Display;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;

pub async fn fixed_reply<T>(first: TcpListener, second: TcpListener, reply: T)
where
    T: Display + Send + Sync + 'static,
{
    let reply = Arc::new(reply);
    loop {
        tokio::select! {
            r = first.accept() => {
                let (mut stream, _) = r.unwrap();
                let reply = Arc::clone(&reply);
                tokio::spawn(async move {
                    let msg = format!("{}", reply);
                    stream.write_all(msg.as_bytes()).await.unwrap();
                });
            }
            r = second.accept() => {
                let (mut stream, _) = r.unwrap();
                let reply = Arc::clone(&reply);
                tokio::spawn(async move {
                    let msg = format!("{}", reply);
                    stream.write_all(msg.as_bytes()).await.unwrap();
                });
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use std::panic;
    use tokio::io::AsyncReadExt;
    use tokio::task::JoinSet;

    async fn bind_random() -> (TcpListener, SocketAddr) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        (listener, addr)
    }

    #[tokio::test]
    async fn test_echo() {
        let (first_listener, first_addr) = bind_random().await;
        let (second_listener, second_addr) = bind_random().await;
        let reply = "Yo";
        tokio::spawn(fixed_reply(first_listener, second_listener, reply));

        let mut join_set = JoinSet::new();

        for _ in 0..3 {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, _) = socket.split();
                    let mut buf = Vec::new();
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, reply.as_bytes());
                });
            }
        }

        while let Some(outcome) = join_set.join_next().await {
            if let Err(e) = outcome {
                if let Ok(reason) = e.try_into_panic() {
                    panic::resume_unwind(reason);
                }
            }
        }
    }
}
