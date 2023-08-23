use std::{collections::HashMap, env, io::BufWriter, io::Write, net::TcpStream};

#[tokio::test]
async fn test_http_sample() {
    let result = reqwest::get("https://www.example.com").await;
    assert!(result.is_ok());
    let res = result.unwrap();
    assert_eq!(res.status(), 200);
}

#[tokio::test]
async fn test_integration() {
    let host = env::var("HOST").unwrap_or("localhost".to_string());
    let port = env::var("PORT").unwrap_or("10080".to_string());

    let result = TcpStream::connect(format!("{host}:{port}"));
    assert!(result.is_ok());
    let socket = result.unwrap();

    let mut writer = BufWriter::new(socket.try_clone().unwrap());
    let mut m_req = HashMap::new();
    m_req.insert("message_type", "GET_MESSAGES");

    let result = writer.write_all((serde_json::to_string(&m_req).unwrap() + "\r\n").as_bytes());
    assert!(result.is_ok());
}

fn main() {}
