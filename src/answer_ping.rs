use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::collections::HashMap;

pub struct Storage {
    pub storage: HashMap<String, String>,
}

impl Storage {
    pub async fn handle_get(&mut self, key: String, stream: &mut TcpStream) {
        let value = match self.storage.get(&key) {
            Some(val) => val,
            None => &"$-1\r\n".to_string(),
        };
        stream.write_all(value.as_bytes()).await.unwrap();
    }

    pub async fn handle_set(&mut self, key: String, number: String, value: String, stream: &mut TcpStream) {
        self.storage.insert(key, format!("{number}\r\n{value}\r\n"));
        stream.write_all(b"+OK\r\n").await.unwrap();
    }
}

pub async fn handle_ping(stream: &mut TcpStream) {
    stream.write_all(b"+PONG\r\n").await.unwrap();
}

pub async fn handle_echo(stream: &mut TcpStream, precursor: String, msg: String) {
    let message: String = format!("{precursor}\r\n{msg}\r\n");
    stream.write_all(message.as_bytes()).await.unwrap();
}


