use super::ServerMessage;
use futures::sync::mpsc::Sender;
use futures::{Future, Sink};

#[derive(Clone)]
pub struct ServerHandle(Sender<ServerMessage>);

impl ServerHandle {
    pub(crate) fn new(sender: Sender<ServerMessage>) -> Self {
        ServerHandle(sender)
    }

    pub fn send(&self, message: ServerMessage) {
        tokio::spawn(self.0.clone().send(message).map(|_| ()).map_err(|e| {
            println!("Could not send message to server: {:?}", e);
        }));
    }
}
