extern crate ed25519_dalek;
extern crate rand;

use std::{
    fs::File,
    io::{stdin, BufRead, BufReader, BufWriter, Error, Read, Write},
    net::TcpStream,
    thread,
};

const HOST: &str = "server";
const PORT: i32 = 10080;

use ed25519_dalek::{Keypair, SignatureError};
use rand::rngs::OsRng;
use serde::Serialize;

use local_talk::interface::{AcceptMessageType, DeleteMessageDto, GetMessageDto, PostMessageDto};

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

fn interactive_start(_keypair: &Keypair) {
    let mut sock = TcpStream::connect(format!("{HOST}:{PORT}")).expect("Failed to connect");

    //
    // Receive Messages
    //
    let mut reader = BufReader::new(sock.try_clone().unwrap());
    thread::spawn(move || loop {
        let s = accept_message(&mut reader);
        println!("msg: {s}");
    });

    //
    // Send Messages
    //
    let i = stdin();
    loop {
        let mut buf = String::new();
        i.read_line(&mut buf).unwrap();

        let phrase: Vec<&str> = buf.trim().split(" ").collect();

        match phrase[0] {
            "get" => {
                let m_req = GetMessageDto {
                    message_type: AcceptMessageType::GetMessages.to_str().to_string(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            "post" => {
                let m_req = PostMessageDto {
                    message_type: AcceptMessageType::SendMessage.to_str().to_string(),
                    user_id: "user_01".to_string(),
                    password: "sample".to_string(),
                    message: phrase[1].to_string(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            "delete" => {
                let m_req = DeleteMessageDto {
                    message_type: AcceptMessageType::DeleteMessage.to_str().to_string(),
                    user_id: "user_01".to_string(),
                    password: "sample".to_string(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            _ => println!("not implemented"),
        }
    }
}

const FILE_PATH: &str = "/app/local/key-pair";

fn write_key_pair(keypair: &Keypair) {
    File::create(FILE_PATH)
        .unwrap()
        .write_all(&keypair.to_bytes())
        .unwrap();
}

fn load_key_pair() -> Result<Keypair, SignatureError> {
    let mut buffer = Vec::<u8>::new();
    File::open(FILE_PATH)
        .unwrap()
        .read_to_end(&mut buffer)
        .unwrap();

    Keypair::from_bytes(&buffer)
}

fn main() {
    let key_pair = match load_key_pair() {
        Ok(key) => key,
        Err(_) => {
            let mut csprng = OsRng {};
            let keypair: Keypair = Keypair::generate(&mut csprng);

            write_key_pair(&keypair);
            keypair
        }
    };

    interactive_start(&key_pair);
}
