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
use storage::PostStrageManager;

struct DatabaseSettings {
    db_host: String,
    db_port: String,
}

impl DatabaseSettings {
    fn get_url(&self) -> String {
        return format!("postgresql://app:appPassword@{}:{}/lt", self.db_host, self.db_port);
    }
}

fn send_messages(da: &PostStrageManager, socket: TcpStream) {
    let v = sender::RecordsLoadedNotification {
        response_type: SendMessageType::RecordsLoaded.to_string(),
        records: da.load(),
    };
    v.send(&socket);
}

fn accept_message(da: &PostStrageManager, ss: std::slice::Iter<TcpStream>, data: PostData) -> Result<(), String> {
    da.push(&data.user_id, &data.password, &data.message)?;
    let v = sender::UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    };
    for s in ss {
        v.send(s);
    }
    Ok(())
}

fn delete_message(da: &PostStrageManager, data: DeleteData) {
    da.delete(&data.user_id, &data.password);
    sender::UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    };
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
    let jh = thread::spawn(move || loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("new client: {addr:?}");
                let mut reader = BufReader::new(socket.try_clone().unwrap());
                Arc::clone(&sockets)
                    .lock()
                    .unwrap()
                    .push(socket.try_clone().unwrap());

                let ss = Arc::clone(&sockets);
                let da = PostStrageManager::new(pool.clone());

                thread::spawn(move || loop {
                    let mut rcv_data = String::new();
                    if let Err(e) = match reader.read_line(&mut rcv_data) {
                        Ok(_) => match AcceptedMessage::from_str(&rcv_data) {
                            AcceptedMessage::GetMessages => {
                                send_messages(&da, socket.try_clone().unwrap());
                                Ok(())
                            }
                            AcceptedMessage::PostMessage(msg) => {
                                accept_message(&da, ss.lock().unwrap().iter(), msg)
                            }
                            AcceptedMessage::DeleteMessage(msg) => {
                                delete_message(&da, msg);
                                Ok(())
                            },
                            AcceptedMessage::None => {
                                thread::sleep(Duration::from_millis(10));
                                Ok(())
                            },
                        },
                        Err(e) => Err(format!("no message: {e:?}")),
                    } {
                        print!("error handled: {e}")
                    }
                });
            }
            Err(e) => println!("couldn't get client: {e:?}"),
        }
    });
    if let Err(_) = jh.join() {
        println!("failed to join thread");
    }
    Ok(())
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

    let settings = DatabaseSettings{
        db_host: env::var("DB_HOST").unwrap_or("localhost".to_string()),
        db_port: env::var("DB_PORT").unwrap_or("5432".to_string()),      
    };

    match accept_requests(settings) {
        Ok(_) => println!("Terminated"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
