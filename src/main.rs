extern crate daemonize;

use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    net::TcpListener,
    thread,
    time::Duration,
};

use daemonize::Daemonize;

fn accept_requests() -> Result<(), Error> {
    let listener = TcpListener::bind("127.0.0.1:10080")?;
    let jh = thread::spawn(move || loop {
        match listener.accept() {
            Ok((socket, addr)) => {
                println!("new client: {addr:?}");
                let mut reader = BufReader::new(socket);
                thread::spawn(move || loop {
                    let mut rcv_data = String::new();
                    match reader.read_line(&mut rcv_data) {
                        Ok(_) => {
                            if rcv_data.len() == 0 {
                                thread::sleep(Duration::from_millis(10))
                            } else {
                                println!("handled message: {rcv_data:?}");
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

    match accept_requests() {
        Ok(_) => println!("Terminated"),
        Err(e) => eprintln!("Error, {}", e),
    }
}
