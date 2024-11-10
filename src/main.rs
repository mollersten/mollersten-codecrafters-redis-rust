#![allow(unused_imports)]
use std::{
    io::Write,
    net::TcpListener,
};
use redis_starter_rust::answer_ping::*;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

     let listener = TcpListener::bind("127.0.0.1:6379").expect("Couldn't bind to 127.0.0.1:6379");

     for stream in listener.incoming() {
         match stream {
             Ok(stream) => {
                 pong(stream);
             }
             Err(e) => {
                 println!("error: {}", e);
             }
         }
     }
}
