use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Write},
    net::TcpStream,
};

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct PostedRecord {
    pub user_name: String,
    pub message: String,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct RecordsLoadedNotification {
    pub response_type: String,
    pub records: Vec<PostedRecord>,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct UpdatedNotification {
    pub response_type: String,
}

pub trait Sendable
where
    Self: Serialize,
{
    fn send(&self, s: &TcpStream) {
        BufWriter::new(s)
            .write(&(serde_json::to_string(self).unwrap() + "\r\n").as_bytes())
            .unwrap();
    }
}

impl Sendable for RecordsLoadedNotification {}
impl Sendable for UpdatedNotification {}
