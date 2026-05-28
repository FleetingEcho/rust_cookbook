use async_std::fs;
#[cfg(test)]
use async_std::io::Result as AsyncResult;
use async_std::io::{Read, Write};
use async_std::net::TcpListener;
use async_std::prelude::*;
use async_std::task;
use futures::stream::StreamExt;
#[cfg(test)]
use futures::task::{Context, Poll};
use std::marker::Unpin;
use std::time::Duration;
#[cfg(test)]
use std::{cmp::min, pin::Pin};

pub async fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:8888").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            task::spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: impl Read + Write + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(5)).await;
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(format!("src/{filename}"))
        .await
        .unwrap();

    let response = format!(
        "{status_line}\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{contents}",
        contents.len()
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}

// ===== 测试 =====
// MockTcpStream 通过实现 async_std::io::Read + Write，
// 让 handle_connection 可以在不绑定真实端口的情况下被测试。
// cursor 字段确保 poll_read 每次推进读取位置，模拟真实流的行为。

#[cfg(test)]
struct MockTcpStream {
    read_data: Vec<u8>,
    write_data: Vec<u8>,
    cursor: usize,
}

#[cfg(test)]
impl async_std::io::Read for MockTcpStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context,
        buf: &mut [u8],
    ) -> Poll<AsyncResult<usize>> {
        let remaining = self.read_data.len().saturating_sub(self.cursor);
        let size = min(remaining, buf.len());
        if size == 0 {
            return Poll::Ready(Ok(0));
        }
        buf[..size].copy_from_slice(&self.read_data[self.cursor..self.cursor + size]);
        self.cursor += size;
        Poll::Ready(Ok(size))
    }
}

#[cfg(test)]
impl async_std::io::Write for MockTcpStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        _: &mut Context,
        buf: &[u8],
    ) -> Poll<AsyncResult<usize>> {
        self.write_data.extend_from_slice(buf);
        Poll::Ready(Ok(buf.len()))
    }

    fn poll_flush(self: Pin<&mut Self>, _: &mut Context) -> Poll<AsyncResult<()>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _: &mut Context) -> Poll<AsyncResult<()>> {
        Poll::Ready(Ok(()))
    }
}

#[cfg(test)]
impl Unpin for MockTcpStream {}

#[async_std::test]
async fn test_handle_connection() {
    let input_bytes = b"GET / HTTP/1.1\r\n";
    let mut contents = vec![0u8; 1024];
    contents[..input_bytes.len()].clone_from_slice(input_bytes);
    let mut stream = MockTcpStream {
        read_data: contents,
        write_data: Vec::new(),
        cursor: 0,
    };

    handle_connection(&mut stream).await;

    let expected_contents = async_std::fs::read_to_string("src/hello.html")
        .await
        .unwrap();
    let expected_response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
        expected_contents.len(),
        expected_contents
    );
    assert_eq!(stream.write_data, expected_response.as_bytes());
}
