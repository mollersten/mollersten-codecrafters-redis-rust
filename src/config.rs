use tokio::{io::AsyncWriteExt, net::TcpStream};
use crate::parser::parse_array;

pub struct Config; 


impl Config {
    pub async fn new() -> Self {
        Self {
        }
    }
    pub async fn handle_config_get(&mut self, key: String, stream: &mut TcpStream, dir: String, dbfilename: String) {
        let val = match key.as_str() {
            "dir" => parse_array(vec![key, dir]).await,
            "dbfilename" => parse_array(vec![key, dbfilename]).await,
            _ => "Neither dir nor dbfilename".to_owned(),
        };
        stream.write_all(val.as_bytes()).await.unwrap();
    }

}
