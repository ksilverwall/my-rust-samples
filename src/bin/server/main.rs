extern crate daemonize;

use daemonize::Daemonize;
use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Error, Write},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(sqlx::FromRow)]
struct NoRecord {}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
struct PostedRecord {
    user_name: String,
    message: String,
}

#[derive(Serialize, Deserialize)]
struct Revieved {
    request_type: String,
}

fn send_messages(p: &Pool<Postgres>, socket: TcpStream) {
    let records = block_on(
        sqlx::query_as::<_, PostedRecord>("SELECT user_name, posted_at, message FROM main.records")
            .fetch_all(p),
    )
    .unwrap();

    let mut w = BufWriter::new(socket);
    for r in records {
        w.write((serde_json::to_string(&r).unwrap() + "\r\r").as_bytes())
            .unwrap();
    }
}

fn accept_message(
    p: &Pool<Postgres>,
    ss: std::slice::Iter<TcpStream>,
    name: String,
    rcv_data: String,
) {
    for s in ss {
        BufWriter::new(s).write(rcv_data.as_bytes()).unwrap();
    }
    block_on(sqlx::query_as::<_, NoRecord>("INSERT INTO main.records (user_name, posted_at, message) VALUES ($1, CURRENT_TIMESTAMP, $2)").bind(name).bind(rcv_data).fetch_optional(p)).unwrap();
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
                        Ok(_) => {
                            if rcv_data.len() == 0 {
                                thread::sleep(Duration::from_millis(10))
                            } else {
                                println!("handled message: {rcv_data:?}");
                                let value = serde_json::from_str::<Revieved>(&rcv_data).unwrap();

                                if value.request_type == "SEND_MESSAGE" {
                                    accept_message(
                                        &p,
                                        ss.lock().unwrap().iter(),
                                        addr.to_string(),
                                        rcv_data,
                                    );
                                }
                                if value.request_type == "GET_MESSAGES" {
                                    send_messages(&p, socket.try_clone().unwrap());
                                }
                            }
                        }
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
