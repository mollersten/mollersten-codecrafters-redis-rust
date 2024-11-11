use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::collections::HashMap;

pub struct HMap {
    pub hm: HashMap<String, String>
}

impl HMap {
    pub async fn handle_get(&mut self) {

    }

    pub async fn handle_set(&mut self, key: String, value: String, stream: &mut TcpStream) {
        self.hm.insert(key, value);
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


