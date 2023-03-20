use std::net::TcpStream;

use crate::sender::{PostedRecord, RecordsLoadedNotification, Sendable, UpdatedNotification};
use local_talk::interface::SendMessageType;

pub fn send_loaded(socket: &TcpStream, records: Vec<PostedRecord>) {
    let v = RecordsLoadedNotification {
        response_type: SendMessageType::RecordsLoaded.to_string(),
        records: records,
    };
    v.send(&socket);
}

pub fn broadcast_updated(sockets: std::slice::Iter<TcpStream>) {
    let v = UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    };
    for s in sockets {
        v.send(s);
    }
}
