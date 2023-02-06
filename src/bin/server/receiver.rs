use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Received {
    request_type: String,
}

#[derive(Serialize, Deserialize)]
struct ReceivedMessage {
    message: String,
}

pub enum ReceivedData {
    PostMessage(String),
    GetMessages,
    None,
}

use local_talk::interface::AcceptMessageType;

impl ReceivedData {
    pub fn from_str(data: &String) -> ReceivedData {
        if data.len() == 0 {
            return ReceivedData::None;
        }

        println!("handled message: {data:?}");
        let value = serde_json::from_str::<Received>(data).unwrap();

        match AcceptMessageType::from_str(&value.request_type.to_string()) {
            AcceptMessageType::GetMessages => ReceivedData::GetMessages,
            AcceptMessageType::SendMessage => {
                let value = serde_json::from_str::<ReceivedMessage>(data).unwrap();
                ReceivedData::PostMessage(value.message)
            }
        }
    }
}
