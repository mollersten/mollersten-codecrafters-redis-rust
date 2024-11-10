use std::{
    net::TcpStream,
    io::{Write, Read},
};

pub fn pong(mut stream: TcpStream) {

    let mut content: String = String::with_capacity(512);
    let mut handle = stream.try_clone().unwrap().take(512);
    loop {
        let result = handle.read_to_string(&mut content);

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
