use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use crate::answer_ping::*;
use std::collections::HashMap;

pub async fn parse_bytes(mut stream: TcpStream) {
    let mut buf = [0; 512];
    let mut mp = Storage {
        storage: HashMap::new(),
    }; 
    tokio::spawn(async move {
        loop {
            let read_count = stream.read(&mut buf).await.unwrap();
            if read_count == 0 {
                break;
            }
            let message = std::str::from_utf8(&buf).expect("Couldn't get message");

            let lines: Vec<&str> = message.lines().collect();

            if lines.len() > 2 {
                match lines[2] {
                    "PING" => handle_ping(&mut stream).await,
                    "ECHO" if lines.len() > 3 => {
                        handle_echo(&mut stream, lines[3].to_string(), lines[4].to_string()).await;
                    },
                    "SET" => mp.handle_set(lines[4].to_string(), lines[5].to_string(), lines[6].to_string(), &mut stream).await,

                    "GET" => mp.handle_get(lines[4].to_string(), &mut stream).await,
                    c => panic!("Can't handle command {c}!"),
                };
            }
    }
    });
}
