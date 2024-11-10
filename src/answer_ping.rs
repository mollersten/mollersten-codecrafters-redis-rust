use std::{
    net::TcpStream,
    io::{Write, Read},
};

pub fn pong(mut stream: TcpStream) {
    //stream.write_all(b"+PONG\r\n").expect("Couldn't write");
    let mut content: String = String::new();

    stream.read_to_string(&mut content).expect("Couldn't read message");

    dbg!(&content);
    if content == "PING" {
        stream.write_all(b"+PONG\r\n").expect("Couldn't write");
    }
}
