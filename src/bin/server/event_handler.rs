use std::{
    error::Error,
    net::TcpStream,
    sync::{Arc, Mutex},
};

use crate::{
    message_sender::{broadcast_updated, send_loaded},
    receiver::{DeleteData, PostData},
    storage::PostStorageManager,
    ethereum::EthereumManager,
};

pub struct EventHandler {
    pub post_storage_manager: PostStorageManager,
    pub ethereum_manager: EthereumManager,
    pub sockets: Arc<Mutex<Vec<TcpStream>>>,
}

impl EventHandler {
    pub fn clone(&self) -> Self {
        return Self{
            post_storage_manager: self.post_storage_manager.clone(),
            ethereum_manager: self.ethereum_manager.clone(),
            sockets: Arc::clone(&self.sockets),
        }
    }
    pub fn connected(&self, socket: TcpStream) {
        self.sockets.lock().unwrap().push(socket);
    }
    pub fn get_messages(&self, socket: &TcpStream) -> Result<(), Box<dyn Error>> {
        let loaded = self.post_storage_manager.load();
        send_loaded(socket, loaded);
        Ok(())
    }
    pub fn post_message(&self, data: &PostData) -> Result<(), Box<dyn Error>> {
        self.post_storage_manager
            .push(&data.user_id, &data.password, &data.message)?;
        broadcast_updated(self.sockets.lock().unwrap().iter());
        Ok(())
    }
    pub fn delete_message(&self, data: &DeleteData) -> Result<(), Box<dyn Error>> {
        self.post_storage_manager
            .delete(&data.user_id, &data.password);
        broadcast_updated(self.sockets.lock().unwrap().iter());
        Ok(())
    }
}
