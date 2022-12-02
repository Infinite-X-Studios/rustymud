use tokio::net::TcpStream;

pub struct Session {
    socket: TcpStream,
}
