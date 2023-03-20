use serde::{Deserialize, Serialize};
use std::error::Error;
use std::net::TcpStream;

use local_talk::interface::{AcceptMessageType, DeleteMessageDto, PostMessageDto};

use crate::event_handler::EventHandler;

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
}

impl AcceptedMessage {
    pub fn from_str(data: &str) -> Result<Self, Box<dyn Error>> {
        #[derive(Deserialize)]
        struct _H {
            message_type: String,
        }

        let message_type = serde_json::from_str::<_H>(data)?.message_type;

        let accepted = match AcceptMessageType::from_str(&message_type) {
            AcceptMessageType::GetMessages => AcceptedMessage::GetMessages,
            AcceptMessageType::SendMessage => {
                let rec = serde_json::from_str::<PostMessageDto>(data)?;
                AcceptedMessage::PostMessage(PostData {
                    user_id: rec.user_id,
                    password: rec.password,
                    message: rec.message,
                })
            }
            AcceptMessageType::DeleteMessage => {
                let rec = serde_json::from_str::<DeleteMessageDto>(data)?;
                AcceptedMessage::DeleteMessage(DeleteData {
                    user_id: rec.user_id,
                    password: rec.password,
                })
            }
        };
        Ok(accepted)
    }

    pub fn routing(&self, eh: &EventHandler, socket: &TcpStream) -> Result<(), Box<dyn Error>> {
        match self {
            AcceptedMessage::GetMessages => eh.handle_get_messages(socket),
            AcceptedMessage::PostMessage(msg) => eh.handle_post_message(&msg),
            AcceptedMessage::DeleteMessage(msg) => eh.handle_delete_message(&msg),
        }
    }
}
