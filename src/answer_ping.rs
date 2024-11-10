use std::{
    io::{Read, Write}, net::TcpStream, thread
};


pub fn pong(mut stream: TcpStream) {
    loop {
        let mut buf = [0; 512];
        let buf_count = stream.read(&mut buf).expect("Couldn't read");


        if buf_count == 0 {
            break;
        }

        let mut cont = String::with_capacity(512);
        for byte in buf.bytes() {
            cont.push_str(&byte.unwrap().to_string());
            if cont.contains("PING") {
                let mut stream2 = stream.try_clone().unwrap();
                thread::spawn(move||{
                    stream2.write_all(b"PONG\r\n").expect("Error writing...");
                });
                cont = String::with_capacity(512);
            }
        }


        //stream.write_all(b"+PONG\r\n").expect("Error writing...");
    }
}
