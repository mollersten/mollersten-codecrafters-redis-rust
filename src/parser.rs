use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use crate::answer_ping::*;

pub async fn parse_bytes(mut stream: TcpStream) {
    let mut buf = [0; 512];
    tokio::spawn(async move {
        loop {
            let read_count = stream.read(&mut buf).await.unwrap();
            if read_count == 0 {
                break;
            }
            let message = std::str::from_utf8(&buf).expect("Couldn't get message");

            let lines: Vec<&str> = message.lines().collect();

            if lines.len() > 1 {
                match lines[1] {
                    "PING" => handle_ping(&mut stream).await,
                    "ECHO" if lines.len() > 2 => {
                        handle_echo(&mut stream, lines[2].to_string()).await;
                    },
                    _ => break,
                };
            }
    }
    });
}
