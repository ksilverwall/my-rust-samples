use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{sender, sender::Sendable};

use local_talk::interface::SendMessageType;

pub fn send_loaded(socket: &TcpStream, records: Vec<sender::PostedRecord>) {
    let v = sender::RecordsLoadedNotification {
        response_type: SendMessageType::RecordsLoaded.to_string(),
        records: records,
    };
    v.send(&socket.try_clone().unwrap());
}

pub fn broadcast_updated(sockets: Arc<Mutex<Vec<TcpStream>>>) {
    let v = sender::UpdatedNotification {
        response_type: SendMessageType::Updated.to_string(),
    };
    for s in sockets.lock().unwrap().iter() {
        v.send(s);
    }
}
