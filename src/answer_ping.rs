use std::{
    net::TcpStream,
    io::{Write, Read},
};

pub fn pong(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 512];
        let buf_count = stream.read(&mut buf).expect("Couldn't read");

        if buf_count == 0 {
            break;
        }
        stream.write_all(b"+PONG\r\n").expect("Couldn't write");
    }
}
