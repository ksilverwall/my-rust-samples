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
use futures::executor::block_on;
use rand::rngs::OsRng;
use serde::Serialize;
use sha256::digest;

mod transaction;

use local_talk::interface::{AcceptMessageType, DeleteMessageDto, GetMessageDto, PostMessageDto};
use sqlx::postgres::PgPoolOptions;
use transaction::{TransactionData, TransactionDeleteData, TransactionPostData, Writer};

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

fn interactive_start(keypair: &Keypair) {
    let mut sock = TcpStream::connect(format!("{HOST}:{PORT}")).expect("Failed to connect");
    let database_url = "postgresql://app:appPassword@database:5432/lt";

    let pool = block_on(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url),
    )
    .unwrap();
 
    let writer = Writer {
        pool: pool,
    };

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
                let user_id = "user_01".to_string();
                let password = "sample".to_string();
                let message = phrase[1].to_string();
                let message_type = AcceptMessageType::SendMessage.to_str().to_string();

                let buf = message.clone();

                let v = TransactionPostData {
                    user_id: user_id.clone(),
                    request_type: message_type.clone(),
                    digest: digest(buf),
                };

                TransactionData::from_data(v, keypair).post(&writer);

                let m_req = PostMessageDto {
                    message_type: message_type.clone(),
                    user_id: user_id.clone(),
                    password: password.clone(),
                    message: message.clone(),
                };

                send_message(&m_req, &mut sock).unwrap();
            }
            "delete" => {
                let message_type = AcceptMessageType::DeleteMessage.to_str().to_string();
                let user_id = "user_01".to_string();
                let password = "sample".to_string();

                let v = TransactionDeleteData {
                    user_id: user_id.clone(),
                    request_type: message_type.clone(),
                };

                TransactionData::from_data(v, keypair).post(&writer);

                let m_req = DeleteMessageDto {
                    message_type: message_type.clone(),
                    user_id: user_id.clone(),
                    password: password.clone(),
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
