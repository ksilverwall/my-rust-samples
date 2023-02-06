extern crate daemonize;

mod receiver;
mod sender;

use daemonize::Daemonize;
use futures::executor::block_on;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Error},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use receiver::ReceivedData;
use sender::Sendable;
use local_talk::interface::SendMessageType;

#[derive(sqlx::FromRow)]
struct NoRecord {}

fn send_messages(p: &Pool<Postgres>, socket: TcpStream) {
    let records = block_on(
        sqlx::query_as::<_, sender::PostedRecord>(
            "SELECT user_name, posted_at, message FROM main.records",
        )
        .fetch_all(p),
    )
    .unwrap();

    let v = sender::RecordsLoadedNotification {
        response_type: SendMessageType::RecordsLoaded.to_string(),
        records: records,
    };
    v.send(&socket);
}

fn accept_message(p: &Pool<Postgres>, ss: std::slice::Iter<TcpStream>, name: String, msg: String) {
    block_on(sqlx::query_as::<_, NoRecord>("INSERT INTO main.records (user_name, posted_at, message) VALUES ($1, CURRENT_TIMESTAMP, $2)").bind(name).bind(msg).fetch_optional(p)).unwrap();
    let v = sender::UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    };
    for s in ss {
        v.send(s);
    }
}

fn accept_requests() -> Result<(), Error> {
    let database_url = "postgresql://app:appPassword@database:5432/lt";

    let pool = block_on(
        PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url),
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
                let p = pool.clone();
                thread::spawn(move || loop {
                    let mut rcv_data = String::new();
                    match reader.read_line(&mut rcv_data) {
                        Ok(_) => match ReceivedData::from_str(&rcv_data) {
                            ReceivedData::GetMessages => {
                                send_messages(&p, socket.try_clone().unwrap())
                            }
                            ReceivedData::PostMessage(msg) => {
                                accept_message(&p, ss.lock().unwrap().iter(), addr.to_string(), msg)
                            }
                            ReceivedData::None => thread::sleep(Duration::from_millis(10)),
                        },
                        Err(e) => println!("no message: {e:?}"),
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

    match accept_requests() {
        Ok(_) => println!("Terminated"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
