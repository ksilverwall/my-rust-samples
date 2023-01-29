use std::{io::{Error, Write}, net::TcpStream};

fn main() -> Result<(), Error> {
    let mut sock = TcpStream::connect("127.0.0.1:10080").expect("Failed to connect");
    match sock.write_all("Hello, TCP\r\n".as_bytes()) {
        Ok(()) => println!("send test message success"),
        Err(v) => println!("send test message failed:{}",v),
    }
    match sock.write_all("World End\r\n".as_bytes()) {
        Ok(()) => println!("send test message success"),
        Err(v) => println!("send test message failed:{}",v),
    }

    Ok(())
}
