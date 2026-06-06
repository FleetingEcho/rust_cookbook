// 🔑 要点：tokio::spawn 并发执行多个异步任务
// JoinSet 管理多个并发任务并等待全部完成
// Requires: tokio (with "full" features), anyhow

use tokio::net::TcpListener;

// 同时监听两个端口，所有连接并发处理，数据原样返回
pub async fn echoes(first: TcpListener, second: TcpListener) -> Result<(), anyhow::Error> {
    // 用 tokio::select! 同时 accept 两个 listener
    loop {
        tokio::select! {
            r = first.accept() => {
                let (mut stream, _) = r?;
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    tokio::io::copy(&mut reader, &mut writer).await.unwrap();
                });
            }
            r = second.accept() => {
                let (mut stream, _) = r?;
                tokio::spawn(async move {
                    let (mut reader, mut writer) = stream.split();
                    tokio::io::copy(&mut reader, &mut writer).await.unwrap();
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
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
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
        tokio::spawn(echoes(first_listener, second_listener));

        let requests = vec!["hello", "world", "foo", "bar"];
        let mut join_set = JoinSet::new();

        for request in requests.clone() {
            for addr in [first_addr, second_addr] {
                join_set.spawn(async move {
                    let mut socket = tokio::net::TcpStream::connect(addr).await.unwrap();
                    let (mut reader, mut writer) = socket.split();
                    writer.write_all(request.as_bytes()).await.unwrap();
                    writer.shutdown().await.unwrap();
                    let mut buf = Vec::with_capacity(request.len());
                    reader.read_to_end(&mut buf).await.unwrap();
                    assert_eq!(&buf, request.as_bytes());
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
