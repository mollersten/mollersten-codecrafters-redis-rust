use tokio::net::TcpStream;
use tokio::io::AsyncReadExt;
use crate::answer_ping::*;
use crate::config::*;

pub async fn parse_string(val: String) -> String {
    format!("${}\r\n{}\r\n", val.len(), val)
}

pub async fn parse_array(vals: Vec<String>) -> String {
    let length = vals.len();
    let mut ret_val = format!("*{length}\r\n");
    for val in &vals {
        ret_val.push_str(&parse_string(val.to_owned()).await);
    }

    ret_val
}

pub async fn parse_bytes(mut stream: TcpStream, dir: String, dbfilename: String) {
    let mut buf = [0; 512];
    let mut mp = Storage::new().await;
    let mut cg = Config::new().await;
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

                    "GET" =>  mp.handle_get(lines[4].to_string(), &mut stream).await,
                    "CONFIG" => cg.handle_config_get(lines[6].to_string(), &mut stream, dir.to_owned(), dbfilename.to_owned()).await, 
                    c => panic!("Can't handle command {c}!"),
                };
            }
    }
    });
}
