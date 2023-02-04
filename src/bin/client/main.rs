use std::{
    io::{BufRead, BufReader, Error, Write},
    net::{TcpStream, ToSocketAddrs},
};

const HOST: &str = "server";
const PORT: i32 = 10080;

fn main() -> Result<(), Error> {
    let socket_addrs = format!("{HOST}:{PORT}")
        .to_socket_addrs()
        .expect("Failed to get socket addr from hostname");

    for addr in socket_addrs {
        let mut sock = TcpStream::connect(addr).expect("Failed to connect");
        match sock.write_all("Hello, TCP\r\n".as_bytes()) {
            Ok(()) => println!("send test message success"),
            Err(v) => println!("send test message failed:{}", v),
        }
        match sock.write_all("World End\r\n".as_bytes()) {
            Ok(()) => println!("send test message success"),
            Err(v) => println!("send test message failed:{}", v),
        }

        let mut rcv_data = String::new();
        let mut reader = BufReader::new(sock.try_clone().unwrap());
        reader.read_line(&mut rcv_data).unwrap();
        print!("{rcv_data}");

        let mut rcv_data = String::new();
        reader.read_line(&mut rcv_data).unwrap();
        print!("{rcv_data}");
        break;
    }

    Ok(())
}
