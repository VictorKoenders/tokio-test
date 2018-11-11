#[macro_use]
extern crate serde_derive;

use futures::{lazy, Future, Stream};
use std::net::SocketAddr;
use tokio::net::TcpListener;

mod client;
mod json_line_codec;
mod server;

pub use crate::client::Client;
pub use crate::json_line_codec::JsonLineCodec;
pub use crate::server::{Server, ServerHandle, ServerMessage, WriteHalf};

type Result<T> = std::result::Result<T, failure::Error>;

fn main() {
    tokio::run(lazy(|| {
        let addr: SocketAddr = "127.0.0.1:1234".parse().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();
        println!("TcpListener bound");

        let handle = Server::spawn();

        listener
            .incoming()
            .for_each(move |stream| {
                let peer_addr = stream.peer_addr().unwrap();
                if let Err(e) = Client::spawn(stream, handle.clone()) {
                    println!("Could not accept client {:?}: {:?}", peer_addr, e);
                }
                Ok(())
            })
            .map_err(|e| {
                println!("Could not accept incoming connections: {:?}", e);
            })
    }));
}
