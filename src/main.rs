#![allow(unused_imports)]
use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use redis_starter_rust::answer_ping::*;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("Couldn't bind to 127.0.0.1:6379");
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok(stream) => {
                pong(stream.0).await;
            }
            Err(e) => {
                println!("error: {}", e);
            }

        }
    }
}
