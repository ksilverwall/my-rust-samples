use ed25519_dalek::{Keypair, Signer};
use sha256::digest;
use sqlx::{Pool, Postgres};

pub trait Serialize {
    fn to_string(&self) -> String;
}

pub struct Writer {
    pub pool: Pool<Postgres>,
}

impl Writer {
    fn write<T: serde::Serialize>(&self, _data: T) {
        // TODO: Implement here
    }
}

#[derive(serde::Serialize)]
pub struct TransactionPostData {
    pub user_id: String,
    pub request_type: String,
    pub digest: String,
}

impl Serialize for TransactionPostData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(serde::Serialize)]
pub struct TransactionDeleteData {
    pub user_id: String,
    pub request_type: String,
}

impl Serialize for TransactionDeleteData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[derive(serde::Serialize)]
pub struct TransactionData {
    data: String,
    signiture: String,
}

impl TransactionData {
    pub fn post(&self, writer: &Writer) {
        writer.write(self)
    }
    pub fn from_data<T: Serialize>(d: T, keypair: &Keypair) -> TransactionData {
        let data = d.to_string();
        let signiture = keypair.sign(digest(data.clone()).as_bytes());
        TransactionData {
            data: data,
            signiture: signiture.to_string(),
        }
    }
}

impl Serialize for TransactionData {
    fn to_string(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
