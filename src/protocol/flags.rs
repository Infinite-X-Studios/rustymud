use super::Telnet;

#[non_exhaustive]
pub struct Server {}

#[allow(dead_code)]
impl Server {
    pub const FLAGS: [u8; 4] = [Telnet::GMCP, Telnet::MCCP2, Telnet::MCCP3, Telnet::MSDP];
}
