pub trait Serialize {
    fn to_string(&self) -> String;
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
