#[non_exhaustive]
pub struct Telnet {}

#[allow(dead_code)]
impl Telnet {
    // Telnet Commands
    pub const SE: u8 = 240; // Subnegotiation End
    pub const NOP: u8 = 241; // No Operation
    pub const DM: u8 = 242; // Data Mark: The data stream portion of a Synch.
                            // This should always be accompanied by a TCP Urgent notification.
    pub const BRK: u8 = 243; // Break
    pub const IP: u8 = 244; // Interrupt Process
    pub const AO: u8 = 245; // Abort Output
    pub const AYT: u8 = 246; // Are You Still There?
    pub const EC: u8 = 247; // Erase Character
    pub const EL: u8 = 248; // Erase Line
    pub const GA: u8 = 249; // Go Ahead
    pub const SB: u8 = 250; // Subnegotiation Start (options follow)
    pub const WILL: u8 = 251; // WILL (option)
    pub const WONT: u8 = 252; // WON'T (option)
    pub const DO: u8 = 253; // DO (option)
    pub const DONT: u8 = 254; // DON'T (option)
    pub const IAC: u8 = 255; // Interpret as Command

    // Command Options
    pub const BINARY: u8 = 0; // Interpret as 8-bit binary transmission.
    pub const ECHO: u8 = 1; // Echo the data received back to the transmitter.
    pub const SUPPRESS_GO_AHEAD: u8 = 3; // Start suppress go ahead.
    pub const STATUS: u8 = 5; // Request the status of TELNET
    pub const TIMING_MARK: u8 = 6; // Define the timing marks.
    pub const LINE_WIDTH: u8 = 8; // Specifies the line width.
    pub const PAGE_SIZE: u8 = 9; // Specifies the number of lines in a page.
    pub const TERMINAL_TYPE: u8 = 24; // Sets the terminal type.
    pub const TERMINAL_SPEED: u8 = 32; // Sets the terminal speed.
    pub const LINE_MODE: u8 = 34; // Changes to the line mode.

    // MUD related protocol definitions
    pub const GMCP: u8 = 201; // GMCP
    pub const MCCP2: u8 = 86; // MCCP2
    pub const MCCP3: u8 = 87; // MCCP3
    pub const MSDP: u8 = 69; // MSDP
}
