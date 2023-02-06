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

impl ReceivedData {
    pub fn from_str(data: &String) -> ReceivedData {
        if data.len() == 0 {
            return ReceivedData::None;
        }

        println!("handled message: {data:?}");
        let value = serde_json::from_str::<Received>(data).unwrap();

        match value.request_type.as_str() {
            "GET_MESSAGES" => ReceivedData::GetMessages,
            "SEND_MESSAGE" => {
                let value = serde_json::from_str::<ReceivedMessage>(data).unwrap();
                ReceivedData::PostMessage(value.message)
            }
            _ => panic!("unexpected message_type"),
        }
    }
}
