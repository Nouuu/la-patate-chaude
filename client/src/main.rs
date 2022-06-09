use std::{net::TcpStream, io::{Write, Read}};
use shared::Message;


const IP: &'static str = "127.0.0.1";
const PORT: u16 = 7878;

fn main() {
    let address = format!("{}:{}", IP, PORT);
    match TcpStream::connect(address) {
        Ok(stream) => {
            stream.set_nonblocking(true);
            let message = Message::Hello;
            send_message(&stream, message);
            let message = Message::Subscribe { name: "test".to_string() };
            send_message(&stream, message);
            receive_messages(&stream);
        },
        Err(_) => panic!("Could not connect to server {} on port {}", IP, PORT),
    }
}

fn send_message(mut stream: &TcpStream, message: Message) {
    if let Ok(message) = serde_json::to_string(&message) {
        let bytes_message = message.as_bytes();
        let message_size = bytes_message.len() as u32;
        let message_length_as_bytes = message_size.to_be_bytes();
        let result = stream.write(&[&message_length_as_bytes, bytes_message].concat());
        println!("result : {:?}, message: {}", result, message);
    }
}

fn receive_messages(mut stream: &TcpStream){
    loop {
        let mut v = Vec::<u8>::new();
        stream.read_to_end(&mut v);
        let str = String::from_utf8_lossy(&v);
        if str != "" {
            println!("{str:?}");
        }
    }
}
