use simple_logger::SimpleLogger;
use tokio::{
    io::{self, Interest},
    net::{TcpListener, TcpStream},
};

mod protocol;
mod session;
use protocol::{flags, Telnet};

async fn handle_client(socket: TcpStream) -> std::io::Result<()> {
    log::info!("Connection attempt received.");
    socket
        .ready(Interest::READABLE | Interest::WRITABLE)
        .await?;
    log::info!("Connection Established: {}", socket.peer_addr()?);

    // Send a Go Ahead signal (This seems to fix some issues with telnet on Windows)
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
                log::debug!("Connection Closed: {}", socket.local_addr()?);
                return Ok(());
            }
            // Bytes recieved. n = number of bytes recieved.
            Ok(mut n) => {
                for u in buf[n - 2..n].iter().rev() {
                    if u == &b"\r"[0] || u == &b"\n"[0] {
                        n -= 1;
                    } else {
                        // Break out as soon as there are no more \n or \r bytes
                        break;
                    }
                }
                if buf[0] == Telnet::IAC {
                    log::debug!("Interpret As Command Recieved!");

                    let mut command_string: String = String::new();

                    for (i, bit) in buf[..n].iter().enumerate() {
                        if *bit == Telnet::DO {
                            /* TODO: Handle DO(s) */
                            log::debug!("IAC DO command recieved.");
                            if i < n {
                                if buf[i + 1] == Telnet::TIMING_MARK {
                                    log::debug!("Sending IAC WILL TIMING_MARK");
                                    socket.try_write(&[
                                        Telnet::IAC,
                                        Telnet::WILL,
                                        Telnet::TIMING_MARK,
                                    ])?;
                                }
                            } else {
                                log::warn!(
                                    "There must have been an error. Nothing recieved after IAC DO!"
                                );
                            }
                        } else if *bit == Telnet::WILL {
                            /* TODO: Handle WILL(s) */
                            log::debug!("IAC WILL command recieved.");
                        } else if *bit == Telnet::DONT {
                            /* TODO: Handle DONT(s) */
                            log::debug!("IAC DONT command recieved.");
                        } else if *bit == Telnet::WONT {
                            /* TODO: Handle WONT(s) */
                            log::debug!("IAC WONT command recieved.");
                        } else if *bit == Telnet::IP {
                            log::debug!("IAC IP command recieved.");
                            // Dropout to close the connection.
                            return Ok(());
                        }

                        command_string.push_str(&Telnet::from_u8(*bit));
                    }

                    log::debug!("{}", command_string);
                } else if let Ok(s) = std::str::from_utf8(&buf[..n]) {
                    // This is good, print it out!
                    log::info!("Buffer: {}", s);
                } else {
                    // Wait until the socket is writable.
                    socket.writable().await?;

                    // Try to send the message
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
    match SimpleLogger::new().init() {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    };

    let listener = match TcpListener::bind("127.0.0.1:8080").await {
        Ok(l) => l,
        Err(e) => {
            log::error!("Error binding port: {}", e);
            return Err(e);
        }
    };

    // accept connections and process them sequentially
    loop {
        match listener.accept().await {
            Ok((socket, _addr)) => {
                tokio::spawn(async move {
                    match handle_client(socket).await {
                        Ok(_) => {
                            log::info!("Client Disconnected.");
                            Ok(())
                        }
                        Err(e) => {
                            log::error!("Handle Client: {}", e);
                            Err(e)
                        }
                    }
                });
            }
            Err(e) => {
                // TODO: We should have a log file where we log all of these errors.
                log::error!("Client unable to connect: {:?}", e);
            }
        }
    }
}
