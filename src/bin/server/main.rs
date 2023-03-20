mod ethereum;
mod event_handler;
mod message_sender;
mod receiver;
mod sender;
mod storage;

use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    error::Error,
    fs,
    io::{BufRead, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use ethereum::EthereumManager;
use event_handler::EventHandler;
use receiver::AcceptedMessage;
use storage::PostStorageManager;

struct DatabaseSettings {
    db_host: String,
    db_port: String,
}

struct EthereumSettings {
    node_url: String,
    abi_file: String,
    contract_address: String,
}

impl DatabaseSettings {
    fn get_url(&self) -> String {
        return format!(
            "postgresql://app:appPassword@{}:{}/lt",
            self.db_host, self.db_port
        );
    }
}

fn handle_clinet(eh: &EventHandler, socket: TcpStream) -> Result<(), Box<dyn Error>> {
    eh.connected(socket.try_clone()?);

    let mut reader = BufReader::new(socket.try_clone()?);
    let mut data = String::new();
    while let Ok(len) = reader.read_line(&mut data) {
        if len == 0 {
            thread::sleep(Duration::from_millis(10));
            continue;
        }

        println!("handled message: {data:?}");
        if let Err(e) = AcceptedMessage::from_str(&data)?.routing(&eh, &socket) {
            println!("error handled: {e:?}");
        }

        data.clear();
    }
    Ok(())
}

fn accept_requests(
    settings: &DatabaseSettings,
    eth: EthereumSettings,
) -> Result<(), Box<dyn Error>> {
    let database_url = settings.get_url();
    let pool_option = PgPoolOptions::new().max_connections(10);

    let pool = block_on(pool_option.connect(&database_url))?;

    let mh = EventHandler {
        post_storage_manager: PostStorageManager::new(pool),
        ethereum_manager: EthereumManager::create(
            &eth.node_url,
            &fs::read_to_string(&eth.abi_file)?,
            &eth.contract_address,
        )?,
        sockets: Arc::new(Mutex::new(vec![])),
    };

    let listener = TcpListener::bind("0.0.0.0:10080")?;
    loop {
        let (socket, addr) = listener.accept()?;
        println!("new client: {addr:?}");

        let mh_th = mh.clone();

        // FIXME: handle error
        thread::spawn(move || handle_clinet(&mh_th, socket).unwrap());
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let database = DatabaseSettings {
        db_host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
        db_port: env::var("DB_PORT").unwrap_or("5432".to_string()),
    };

    let ethereum = EthereumSettings {
        node_url: env::var("NODE_URL").unwrap_or("http://localhost:8545".to_string()),
        abi_file: env::var("ABI_FILE").unwrap(),
        contract_address: env::var("CONTRACT_ADDRESS").unwrap(),
    };

    accept_requests(&database, ethereum)?;

    println!("Terminated");
    Ok(())
}
