pub async fn parse_bytes(buf: &[u8]) -> Vec<&str> {
    let message = std::str::from_utf8(buf).expect("Couldn't get message");

    let split_message = message.split("\r\n");

    let filtered_message: Vec<&str> = split_message.into_iter().filter(|elem| elem.is_ascii()).collect();
    
    if filtered_message.len() > 1 {
        filtered_message
    } else {
        Vec::new()
    }
}
