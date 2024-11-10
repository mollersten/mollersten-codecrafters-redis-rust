use std::{
    net::TcpStream,
    io::Write,
};

pub fn pong(mut stream: TcpStream) {
    stream.write_all(b"+PONG\r\n").expect("Couldn't write");
}
