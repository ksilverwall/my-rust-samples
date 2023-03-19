extern crate ed25519_dalek;
extern crate rand;

use local_talk::interface::{AcceptMessageType, DeleteMessageDto, GetMessageDto, PostMessageDto};
use serde::Serialize;
use std::{
    io::{stdin, BufRead, BufReader, BufWriter, Error, Write},
    net::TcpStream,
    thread, env,
};

mod transaction;

fn send_message<T: Serialize>(m_req: &T, socket: &mut TcpStream) -> Result<(), Error> {
    let mut writer = BufWriter::new(socket.try_clone().unwrap());
    let content = serde_json::to_string(m_req).unwrap();
    writer.write_all((content + "\r\n").as_bytes())
}

fn accept_message(reader: &mut BufReader<TcpStream>) -> String {
    let mut rcv_data = String::new();
    reader.read_line(&mut rcv_data).unwrap();
    rcv_data
}

fn interactive_start(host: &str, port: i32) {
    println!("starting...");

    let mut sock = TcpStream::connect(format!("{host}:{port}")).expect("Failed to connect");

    //
    // Receive Messages
    //
    let mut reader = BufReader::new(sock.try_clone().unwrap());
    thread::spawn(move || loop {
        let s = accept_message(&mut reader);
        if s.len() == 0 {
            println!("msg length is zero");
            println!("NOTE: on connecion closed, accept_message returns zero length eternal");
            break;
        }
        println!("msg: {s}");
    });

    //
    // Send Messages
    //
    let i = stdin();
    println!("done!!");
    println!("input your message");

    enum Command {
        Unknown(String),
        Get,
        Post,
        Delete,
    }

    impl Command {
        fn parse(value: &str) -> Command {
            match value {
                "get" => Command::Get,
                "post" => Command::Post,
                "delete" => Command::Delete,
                _ => Command::Unknown(value.to_string()),
            }
        }
    }
    loop {
        let mut buf = String::new();
        i.read_line(&mut buf).unwrap();

        let phrase: Vec<&str> = buf.trim().split(" ").collect();

        match Command::parse(phrase[0]) {
            Command::Get => {
                let m_req = GetMessageDto {
                    message_type: AcceptMessageType::GetMessages.to_str().to_string(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            Command::Post => {
                if phrase.len() < 2 {
                    println!("need any message");
                    continue;
                }
                let user_id = "user_01".to_string();
                let password = "sample".to_string();
                let message = phrase[1].to_string();
                let message_type = AcceptMessageType::SendMessage.to_str().to_string();

                let m_req = PostMessageDto {
                    message_type: message_type.clone(),
                    user_id: user_id.clone(),
                    password: password.clone(),
                    message: message.clone(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            Command::Delete => {
                let message_type = AcceptMessageType::DeleteMessage.to_str().to_string();
                let user_id = "user_01".to_string();
                let password = "sample".to_string();

                let m_req = DeleteMessageDto {
                    message_type: message_type.clone(),
                    user_id: user_id.clone(),
                    password: password.clone(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            Command::Unknown(s) => println!("command '{s}' not implemented, "),
        }
    }
}

fn main() {
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port = env::var("PORT").unwrap_or("10080".to_string());

    interactive_start(&host, port.parse().unwrap());
}
