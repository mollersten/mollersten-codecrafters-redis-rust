use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use crate::parser::*;
pub async fn pong(mut stream: TcpStream) {
        tokio::spawn(async move {
            let mut buf = [0; 512];
            loop {
                let buf_count = stream.read(&mut buf).await.expect("Error reading...");
                if buf_count == 0 {
                    break;
                }
                let message = parse_bytes(&buf).await;

                match message[0] {
                    "PING" => stream.write_all(b"+PONG\r\n").await.expect("Error writing..."),
                    "ECHO" if message.len() > 1 => {

                    }, 
                    _ => {},
                };
            }
        });
}
