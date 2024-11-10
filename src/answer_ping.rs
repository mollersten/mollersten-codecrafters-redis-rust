use std::{
    net::TcpStream,
    io::{Write, Read},
};

pub fn pong(mut stream: TcpStream) {

    loop {
        let mut content: String = String::new();
        let result = stream.read_to_string(&mut content);

        match result {
            Ok(n) => {
                println!("Got {n} bytes");
                if content.as_str() == "PING" {
                    stream.write_all(b"+PONG\r\n").expect("Couldn't write");
                }
            },
            Err(_e) => {},
        }
    }
}
