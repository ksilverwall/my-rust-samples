use std::{
    io::{BufRead, BufReader, Error, Write},
    net::TcpStream,
};

const HOST: &str = "server";
const PORT: i32 = 10080;

use serde::Serialize;

#[derive(Serialize)]
struct GetMessageRequest {
    request_type: String,
}

#[derive(Serialize)]
struct SendMessageRequest {
    request_type: String,
    message: String,
}

fn main() -> Result<(), Error> {
    let mut sock = TcpStream::connect(format!("{HOST}:{PORT}")).expect("Failed to connect");

    {
        let m_req = GetMessageRequest {
            request_type: "GET_MESSAGE".to_string(),
        };

        let content = serde_json::to_string(&m_req).unwrap();
        match sock.write_all((content + "\r\n").as_bytes()) {
            Ok(()) => println!("send GET MESSAGE request"),
            Err(v) => println!("send test message failed:{}", v),
        }
    }

    //
    // Send Messages
    //
    let messages = ["Hello Tcp".to_string(), "World End".to_string()];
    for m in messages {
        let m_req = SendMessageRequest {
            request_type: "SEND_MESSAGE".to_string(),
            message: m,
        };

        let content = serde_json::to_string(&m_req).unwrap();
        match sock.write_all((content + "\r\n").as_bytes()) {
            Ok(()) => println!("send test message success"),
            Err(v) => println!("send test message failed:{}", v),
        }
    }

    //
    // Receive Messages
    //
    let mut reader = BufReader::new(sock.try_clone().unwrap());
    for _ in 0..3 {
        let mut rcv_data = String::new();
        reader.read_line(&mut rcv_data).unwrap();
        print!("{rcv_data}");
    }

    Ok(())
}
