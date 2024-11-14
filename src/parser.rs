use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use crate::answer_ping::*;

pub async fn parse_bytes(mut stream: TcpStream) {
    let mut buf = [0; 512];
    let mut mp = Storage::new().await;
    tokio::spawn(async move {
        loop {
            
            let read_count = stream.read(&mut buf).await.unwrap();
            if read_count == 0 {
                break;
            }
            let message = std::str::from_utf8(&buf).expect("Couldn't get message");

            let lines: Vec<&str> = message.lines().collect();

            if lines.len() > 2 {
                match lines[2] {
                    "PING" => handle_ping(&mut stream).await,
                    "ECHO" if lines.len() > 3 => {
                        handle_echo(&mut stream, lines[3].to_string(), lines[4].to_string()).await;
                       
                    },
                    "SET" => {
                        let str_px: Option<String> = lines.get(8).map(|str| str.to_string());
                        let mut px: Option<String> = None;
                        if str_px.is_some() && str_px.unwrap().eq("px") {
                            px = lines.get(10).map(|x| x.to_string());
                        }
                        mp.handle_set(lines[4].to_string(), 
                            lines[5].to_string(), 
                            lines[6].to_string(), &mut stream, px).await;
                    },

                    "GET" => mp.handle_get(lines[4].to_string(), &mut stream).await,
                    c => panic!("Can't handle command {c}!"),
                };
            }
    }
    });
}
