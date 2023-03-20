mod event_handler;
mod message_sender;
mod receiver;
mod sender;
mod storage;

use event_handler::EventHandler;
use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    io::{BufRead, BufReader, Error},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use receiver::AcceptedMessage;
use storage::PostStorageManager;

struct DatabaseSettings {
    db_host: String,
    db_port: String,
}

impl DatabaseSettings {
    fn get_url(&self) -> String {
        return format!(
            "postgresql://app:appPassword@{}:{}/lt",
            self.db_host, self.db_port
        );
    }
}

fn handle_message(mh: &EventHandler, socket: &TcpStream, data: &String) -> Result<(), String> {
    match AcceptedMessage::from_str(data) {
        AcceptedMessage::GetMessages => mh.handle_get_messages(socket),
        AcceptedMessage::PostMessage(msg) => mh.handle_post_message(&msg),
        AcceptedMessage::DeleteMessage(msg) => mh.handle_delete_message(&msg),
        AcceptedMessage::None => {
            thread::sleep(Duration::from_millis(10));
            Ok(())
        }
    }
}

fn handle_clinet(eh: &EventHandler, socket: TcpStream) -> Result<(), String> {
    eh.connected(socket.try_clone().unwrap());

    let mut reader = BufReader::new(socket.try_clone().unwrap());
    let mut data = String::new();
    while let Ok(len) = reader.read_line(&mut data) {
        if len == 0 {
            break;
        }

        if let Err(e) = handle_message(&eh, &socket, &data) {
            println!("error handled: {}", e);
        }

        data.clear();
    }
    Ok(())
}

fn accept_requests(settings: DatabaseSettings) -> Result<(), Error> {
    let database_url = settings.get_url();
    let pool = block_on(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(&database_url),
    )
    .unwrap();
    let sockets: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(vec![]));
    let listener = TcpListener::bind("0.0.0.0:10080")?;

    loop {
        let (socket, addr) = listener.accept()?;
        println!("new client: {addr:?}");

        // FIXME: Move to out of loop
        let mh = EventHandler {
            post_storage_manager: PostStorageManager::new(pool.clone()),
            sockets: Arc::clone(&sockets),
        };

        thread::spawn(move || handle_clinet(&mh, socket));
    }
}

fn main() {
    let settings = DatabaseSettings {
        db_host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
        db_port: env::var("DB_PORT").unwrap_or("5432".to_string()),
    };

    match accept_requests(settings) {
        Ok(_) => println!("Terminated"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
