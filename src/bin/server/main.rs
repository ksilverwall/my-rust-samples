mod ethereum;
mod event_handler;
mod message_sender;
mod receiver;
mod sender;
mod settings;
mod storage;

use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;
use std::{
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
use settings::Settings;
use storage::PostStorageManager;

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

fn accept_requests(settings: &Settings) -> Result<(), Box<dyn Error>> {
    let database_url = settings.database.get_url();
    let pool_option = PgPoolOptions::new().max_connections(10);

    let pool = block_on(pool_option.connect(&database_url))?;

    let mh = EventHandler {
        post_storage_manager: PostStorageManager::new(pool),
        ethereum_manager: EthereumManager::create(
            &settings.ethereum.node_url,
            &fs::read_to_string(&settings.ethereum.abi_file)?,
            &settings.ethereum.contract_address,
        ).map_err(|e| format!("create ethereum manager: {e}"))?,
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
    let settings = Settings::load()?;

    if let Err(e) = accept_requests(&settings) {
        eprintln!("{e:?}");
    };

    println!("Terminated");
    Ok(())
}
