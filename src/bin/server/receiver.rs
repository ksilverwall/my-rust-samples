use serde::{Deserialize, Serialize};

use local_talk::interface::{AcceptMessageType, DeleteMessageDto, PostMessageDto};

#[derive(Serialize, Deserialize)]
pub struct PostData {
    pub user_id: String,
    pub password: String,
    pub message: String,
}

pub struct DeleteData {
    pub user_id: String,
    pub password: String,
}

pub enum AcceptedMessage {
    PostMessage(PostData),
    GetMessages,
    DeleteMessage(DeleteData),
    None,
}

impl AcceptedMessage {
    pub fn from_str(data: &String) -> AcceptedMessage {
        if data.len() == 0 {
            return AcceptedMessage::None;
        }

        println!("handled message: {data:?}");
        let value = serde_json::from_str::<_Received>(data).unwrap();

        match AcceptMessageType::from_str(&&value.message_type.to_string()) {
            AcceptMessageType::GetMessages => AcceptedMessage::GetMessages,
            AcceptMessageType::SendMessage => match serde_json::from_str::<PostMessageDto>(data) {
                Ok(rec) => AcceptedMessage::PostMessage(PostData {
                    user_id: rec.user_id,
                    password: rec.password,
                    message: rec.message,
                }),
                Err(err) => {
                    println!("Failed to parse send message");
                    panic!("{err}");
                }
            },
            AcceptMessageType::DeleteMessage => {
                match serde_json::from_str::<DeleteMessageDto>(data) {
                    Ok(rec) => AcceptedMessage::DeleteMessage(DeleteData {
                        user_id: rec.user_id,
                        password: rec.password,
                    }),
                    Err(err) => {
                        println!("Failed to parse delete message");
                        panic!("{err}");
                    }
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct _Received {
    message_type: String,
}
