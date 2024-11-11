use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn handle_ping(stream: &mut TcpStream) {
    stream.write_all(b"+PONG\r\n").await.unwrap();
}

pub async fn handle_echo(stream: &mut TcpStream, msg: String) {
    stream.write_all(msg.as_bytes()).await.unwrap();
}
