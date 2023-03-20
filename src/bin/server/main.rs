extern crate daemonize;

mod receiver;
mod sender;
mod storage;

use daemonize::Daemonize;
use futures::executor::block_on;
use sqlx::postgres::PgPoolOptions;
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use local_talk::interface::SendMessageType;
use receiver::{AcceptedMessage, DeleteData, PostData};
use sender::Sendable;
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

fn handle_get_messages(da: &PostStorageManager, socket: &TcpStream) -> Result<(), String> {
    let v = sender::RecordsLoadedNotification {
        response_type: SendMessageType::RecordsLoaded.to_string(),
        records: da.load(),
    };
    v.send(&socket.try_clone().unwrap());
    Ok(())
}

fn handle_post_message(
    da: &PostStorageManager,
    data: &PostData,
    on_updated: &dyn Fn(sender::UpdatedNotification) -> (),
) -> Result<(), String> {
    da.push(&data.user_id, &data.password, &data.message)?;
    on_updated(sender::UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    });
    Ok(())
}

fn handle_delete_message(
    da: &PostStorageManager,
    socket: &TcpStream,
    data: &DeleteData,
) -> Result<(), String> {
    da.delete(&data.user_id, &data.password);
    let v = sender::UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    };
    v.send(&socket.try_clone().unwrap());
    Ok(())
}

fn handle_none() -> Result<(), String> {
    thread::sleep(Duration::from_millis(10));
    Ok(())
}

fn handle_message(
    post_storage_manager: &PostStorageManager,
    socket: &TcpStream,
    data: &String,
    on_updated: &dyn Fn(sender::UpdatedNotification) -> (),
) -> Result<(), String> {
    match AcceptedMessage::from_str(data) {
        AcceptedMessage::GetMessages => handle_get_messages(post_storage_manager, socket),
        AcceptedMessage::PostMessage(msg) => {
            handle_post_message(post_storage_manager, &msg, &on_updated)
        }
        AcceptedMessage::DeleteMessage(msg) => {
            handle_delete_message(post_storage_manager, socket, &msg)
        }
        AcceptedMessage::None => handle_none(),
    }
}

fn handle_clinet(
    post_storage_manager: &PostStorageManager,
    sockets: &Arc<Mutex<Vec<TcpStream>>>,
    socket: TcpStream,
) {
    let mut reader = BufReader::new(socket.try_clone().unwrap());
    let mut data = String::new();
    while let Ok(len) = reader.read_line(&mut data) {
        if len == 0 {
            break;
        }

        let on_updated = |v: sender::UpdatedNotification| {
            for s in sockets.lock().unwrap().iter() {
                v.send(s);
            }
        };

        if let Err(e) = handle_message(&post_storage_manager, &socket, &data, &on_updated) {
            println!("error handled: {}", e);
        }

        data.clear();
    }
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

        let sockets_clone = Arc::clone(&sockets);
        let post_storage_manager = PostStorageManager::new(pool.clone());

        Arc::clone(&sockets)
            .lock()
            .unwrap()
            .push(socket.try_clone()?);

        thread::spawn(move || {
            handle_clinet(
                &post_storage_manager,
                &sockets_clone,
                socket.try_clone().unwrap(),
            )
        });
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let is_deamon = args.len() >= 2 && args[1] == "-d";

    if is_deamon {
        let stdout = File::create("/tmp/daemon.out").unwrap();
        let stderr = File::create("/tmp/daemon.err").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/test.pid")
            .working_directory("/tmp")
            .stdout(stdout)
            .stderr(stderr)
            .privileged_action(|| "Executed before drop privileges");

        match daemonize.start() {
            Ok(_) => println!("Success, daemonized"),
            Err(e) => eprintln!("Error, {}", e),
        }
    }

    let settings = DatabaseSettings {
        db_host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
        db_port: env::var("DB_PORT").unwrap_or("5432".to_string()),
    };

    match accept_requests(settings) {
        Ok(_) => println!("Terminated"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
