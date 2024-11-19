#![allow(unused_imports)]
use std::env;

use tokio::net::TcpListener;
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use redis_starter_rust::answer_ping::*;
use redis_starter_rust::parser::*;

#[tokio::main]
async fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();
    let mut dir = String::from("/tmp");
    let mut dbfilename = String::from("dump.rdb");

    for (i, arg) in args.iter().enumerate() {
        match arg.as_str() {
            "--dir" if i + 1 < args.len() => dir = args[i+1].clone(),
            "--dbfilename" if i + 1 < args.len() => dbfilename = args[i+1].clone(),
            _ => continue,
        }
    }


    let listener = TcpListener::bind("127.0.0.1:6379").await.expect("Couldn't bind to 127.0.0.1:6379");
    loop {
        let stream = listener.accept().await;
        match stream {
            Ok(stream) => {
                parse_bytes(stream.0, dir.clone(), dbfilename.clone()).await;
            }
            Err(e) => {
                println!("error: {}", e);
            }

        }
    }
}
