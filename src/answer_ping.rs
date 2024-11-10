use std::{
    net::TcpStream,
    io::{Write, Read},
};

pub fn pong(mut stream: TcpStream) {
    stream.write_all(b"+PONG\r\n").expect("Could not write");
    loop {
        let mut content = String::with_capacity(512);
        let result = stream.read_to_string(&mut content);

        match result {
            Ok(n) => {
                println!("Got {n} bytes");
                if content.as_str() == "*1\r\n$4\r\nPING\r\n" {
                    stream.write_all(b"+PONG\r\n").expect("Couldn't write");
                }
            },
            Err(_e) => {},
        }
    }
}
