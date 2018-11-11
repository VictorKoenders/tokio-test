use crate::JsonLineCodec;
use futures::stream::SplitSink;
use std::net::SocketAddr;
use tokio::codec::Framed;
use tokio::net::TcpStream;

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {}

pub type WriteHalf = SplitSink<Framed<TcpStream, JsonLineCodec<ClientMessage>>>;

pub enum ServerMessage {
    ClientConnected {
        peer_addr: SocketAddr,
        write_half: WriteHalf,
    },
    ClientMessage {
        peer_addr: SocketAddr,
        message: ClientMessage,
    },
    ClientDisconnected {
        peer_addr: SocketAddr,
    },
}
