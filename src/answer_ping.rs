use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use std::collections::HashMap;
use std::time::{Instant, Duration};

struct MapItem {
    value: String,
    expiration_time: Option<String>,
    time_set: Option<Instant>,
}

pub struct Storage {
    storage: HashMap<String, MapItem>,
}

impl Storage {

    pub async fn new() -> Self {
        Self {
            storage: HashMap::new(),
        }
    }

    pub async fn handle_get(&mut self, key: String, stream: &mut TcpStream) {
        let item = match self.storage.get(&key) {
            Some(val) => {
                match &val.expiration_time {
                    Some(_) => {
                        if Self::valid_get(val).await {
                            val
                        } else {
                            &MapItem { value: "$-1\r\n".to_string(), expiration_time: None, time_set: None }
                        }
                    },
                    None => val
                }
            }
            None => &MapItem { value: "$-1\r\n".to_string(), expiration_time: None, time_set: None },
        };
        stream.write_all(item.value.as_bytes()).await.unwrap();
    }

    pub async fn handle_set(&mut self, key: String, number: String, value: String, stream: &mut TcpStream, px: Option<String>) {
        self.storage.insert(key, MapItem { 
            value: format!("{number}\r\n{value}\r\n"), 
            expiration_time: px.clone(),
            time_set: if px.is_some() {
                Some(Instant::now())
            } else {
                None
            }
        });
        stream.write_all(b"+OK\r\n").await.unwrap();
    }

    async fn valid_get(item: &MapItem) -> bool {
        let expiration_time = Duration::from_millis(item.expiration_time.clone().unwrap().parse()
            .expect("Must have been no PX."));

        item.time_set.expect("No time set...").elapsed() < expiration_time   
    }
}

pub async fn handle_ping(stream: &mut TcpStream) {
    stream.write_all(b"+PONG\r\n").await.unwrap();
}

pub async fn handle_echo(stream: &mut TcpStream, precursor: String, msg: String) {
    let message: String = format!("{precursor}\r\n{msg}\r\n");
    stream.write_all(message.as_bytes()).await.unwrap();
}


