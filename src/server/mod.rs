use crate::Client;
use futures::sync::mpsc::channel;
use futures::{Future, Stream};
use std::collections::HashMap;
use std::net::SocketAddr;

mod handle;
mod message;

pub use self::handle::ServerHandle;
pub use self::message::{ClientMessage, ServerMessage, WriteHalf};

pub struct Server {
    pub clients: HashMap<SocketAddr, Client>,
}

impl Server {
    pub fn spawn() -> ServerHandle {
        let (sender, receiver) = channel(1024);
        let mut server = Server {
            clients: HashMap::new(),
        };
        tokio::spawn(
            receiver
                .map_err(|e| {
                    println!("Server crashed: {:?}", e);
                })
                .for_each(move |msg| {
                    match msg {
                        ServerMessage::ClientConnected {
                            peer_addr,
                            write_half,
                        } => {
                            print!("Client connected: {:?}", peer_addr);
                            server
                                .clients
                                .insert(peer_addr, Client::new(peer_addr, write_half));
                            print!(" (connected client count is {})", server.clients.len());
                            println!();
                        }
                        ServerMessage::ClientMessage { peer_addr, message } => {
                            print!("Client send {:?}", message);
                            if let Some(_client) = server.clients.get_mut(&peer_addr) {
                                print!(" (client was connected)");
                            } else {
                                print!(" (client was not connected)");
                            }
                            println!();
                        }
                        ServerMessage::ClientDisconnected { peer_addr } => {
                            print!("Client disconnected: {:?}", peer_addr);
                            let _removed_client = server.clients.remove(&peer_addr);
                            print!(" (connected client count is {})", server.clients.len());
                            println!();
                        }
                    }
                    Ok(())
                })
                .map(|_| {
                    println!("Server finished");
                }),
        );
        ServerHandle::new(sender)
    }
}
