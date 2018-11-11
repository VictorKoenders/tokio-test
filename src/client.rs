use crate::{JsonLineCodec, Result, ServerHandle, ServerMessage, WriteHalf};
use futures::{Future, Stream};
use std::net::SocketAddr;
use tokio::codec::Decoder;
use tokio::net::TcpStream;

pub struct Client {
    pub peer_addr: SocketAddr,
    pub write_half: WriteHalf,
}

impl Client {
    pub fn new(peer_addr: SocketAddr, write_half: WriteHalf) -> Self {
        Client {
            peer_addr,
            write_half,
        }
    }

    pub fn spawn(stream: TcpStream, server_handle: ServerHandle) -> Result<()> {
        let peer_addr = stream.peer_addr().unwrap();
        let codec = JsonLineCodec::default();
        let (write_half, read_half) = codec.framed(stream).split();
        let error_server_handle = server_handle.clone();
        let end_server_handle = server_handle.clone();

        server_handle.send(ServerMessage::ClientConnected {
            peer_addr,
            write_half,
        });

        tokio::spawn(
            read_half
                .map_err(move |e| {
                    println!("Could not read from client: {:?}", e);
                    error_server_handle.send(ServerMessage::ClientDisconnected { peer_addr });
                })
                .for_each(move |line| {
                    server_handle.send(ServerMessage::ClientMessage {
                        peer_addr,
                        message: line,
                    });
                    Ok(())
                })
                .map(move |_| {
                    end_server_handle.send(ServerMessage::ClientDisconnected { peer_addr });
                }),
        );
        Ok(())
    }
}
