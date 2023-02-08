use std::{
    io::{stdin, BufRead, BufReader, BufWriter, Error, Write},
    net::TcpStream,
    thread,
};

const HOST: &str = "server";
const PORT: i32 = 10080;

use serde::Serialize;

use local_talk::interface::{AcceptMessageType, GetMessageDto, PostMessageDto};

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

fn interactive_start() {
    let mut sock = TcpStream::connect(format!("{HOST}:{PORT}")).expect("Failed to connect");

    //
    // Receive Messages
    //
    let mut reader = BufReader::new(sock.try_clone().unwrap());
    thread::spawn(move || loop {
        let s = accept_message(&mut reader);
        println!("msg: {s}");
    });

    {
        let m_req = GetMessageDto {
            message_type: AcceptMessageType::GetMessages.to_string(),
        };

        send_message(&m_req, &mut sock).unwrap();
    }

    //
    // Send Messages
    //
    let i = stdin();
    loop {
        let mut buf = String::new();
        println!("input send message:");
        i.read_line(&mut buf).unwrap();

        let m_req = PostMessageDto {
            message_type: AcceptMessageType::SendMessage.to_string(),
            user_id: "user_01".to_string(),
            password: "sample".to_string(),
            message: buf.to_string(),
        };

        send_message(&m_req, &mut sock).unwrap();
    }
}

fn main() {
    interactive_start();
}
