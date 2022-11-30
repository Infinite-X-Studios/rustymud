use tokio::io::{self, Interest};
use tokio::net::{TcpListener, TcpStream};

mod protocol;
use protocol::Telnet;

async fn handle_client(socket: TcpStream) -> std::io::Result<()> {
    println!("Connection attempt received.");
    socket
        .ready(Interest::READABLE | Interest::WRITABLE)
        .await?;
    println!("Connection Established: {}", socket.peer_addr()?);

    let msg = [Telnet::IAC, Telnet::GA];
    loop {
        match socket.try_write(&msg) {
            Ok(_) => break,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e);
            }
        }
    }

    loop {
        let mut buf = [0u8; 4028];
        match socket.try_read(&mut buf[..]) {
            // Connection closed
            Ok(0) => {
                println!("Connection Closed: {}", socket.local_addr()?);
                return Ok(());
            }
            // Bytes recieved. n = number of bytes recieved.
            Ok(n) => {
                if buf[0] == Telnet::IAC {
                    println!("Interpret As Command Recieved!");

                    let mut command_string: String = String::new();

                    for bit in buf[..n].iter() {
                        if *bit == Telnet::DO { /* TODO: Handle DO(s) */ };

                        command_string.push_str(&Telnet::from_u8(*bit));
                    }

                    println!("{}", command_string);
                } else if let Ok(s) = std::str::from_utf8(&buf[..n]) {
                    // This is good, print it out!
                    println!("Buffer: {}", s);
                } else {
                    // Ignore the input
                    // Send an error message to the sender

                    socket.writable().await?;

                    socket.try_write(
                        "Invalid encoding detected. UTF8 encoding expected.".as_bytes(),
                    )?;
                }
            }
            // If error WouldBlock is returned, data is not yet ready to read, continue until it is.
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            // Error happened, pass it back to handle
            Err(e) => return Err(e),
        };
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    // accept connections and process them serially
    loop {
        match listener.accept().await {
            Ok((socket, _addr)) => {
                tokio::spawn(async move {
                    match handle_client(socket).await {
                        Ok(_) => {
                            println!("Client Disconnected.");
                            Ok(())
                        }
                        Err(e) => Err(e),
                    }
                });
            }
            Err(e) => {
                println!("Client unable to get connected: {:?}", e);
                return Err(e);
            }
        }
    }
}
