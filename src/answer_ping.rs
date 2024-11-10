use std::{
    io::{Read, Write}, net::TcpStream, thread
};


pub fn pong(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 512];
        let buf_count = stream.read(&mut buf).expect("Couldn't read");

        let mut stream2 = stream.try_clone().unwrap();

        if buf_count == 0 {
            break;
        }

        thread::spawn(move||{
            stream2.write_all(b"PONG\r\n").expect("Error writing...");
        });

        stream.write_all(b"+PONG\r\n").expect("Error writing...");
    }
}
