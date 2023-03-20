use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    receiver::{DeleteData, PostData},
    storage::PostStorageManager,
    message_sender::{send_loaded, broadcast_updated},
};

pub struct EventHandler {
    pub post_storage_manager: PostStorageManager,
    pub sockets: Arc<Mutex<Vec<TcpStream>>>,
}

impl EventHandler {
    pub fn connected(&self, socket: TcpStream) {
        self.sockets.lock().unwrap().push(socket);
    }
    pub fn handle_get_messages(&self, socket: &TcpStream) -> Result<(), String> {
        let loaded = self.post_storage_manager.load();
        send_loaded(socket, loaded);
        Ok(())
    }
    pub fn handle_post_message(&self, data: &PostData) -> Result<(), String> {
        self.post_storage_manager
            .push(&data.user_id, &data.password, &data.message)?;
        broadcast_updated(self.sockets.clone());
        Ok(())
    }
    pub fn handle_delete_message(&self, data: &DeleteData) -> Result<(), String> {
        self.post_storage_manager
            .delete(&data.user_id, &data.password);
        broadcast_updated(self.sockets.clone());
        Ok(())
    }
}
