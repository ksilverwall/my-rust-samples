pub enum AcceptMessageType {
    GetMessages,
    SendMessage,
}

pub enum SendMessageType {
    RecordsLoaded,
    Updated,
}

impl AcceptMessageType {
    pub fn from_str(s: &str) -> AcceptMessageType {
        match s {
            "GET_MESSAGES" => AcceptMessageType::GetMessages,
            "SEND_MESSAGE" => AcceptMessageType::SendMessage,
            _ => panic!("unexpected message_type"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            AcceptMessageType::GetMessages => "GET_MESSAGES".to_string(),
            AcceptMessageType::SendMessage => "SEND_MESSAGE".to_string(),
        }
    }
}

impl SendMessageType {
    pub fn from_str(s: &str) -> SendMessageType {
        match s {
            "RECORDS_LOADED" => SendMessageType::RecordsLoaded,
            "UPDATED" => SendMessageType::Updated,
            _ => panic!("unexpected message_type"),
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            SendMessageType::RecordsLoaded => "RECORDS_LOADED".to_string(),
            SendMessageType::Updated => "UPDATED".to_string(),
        }
    }
}
