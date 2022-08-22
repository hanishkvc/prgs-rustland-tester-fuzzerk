//!
//! IO Bridge
//! 
//! HanishKVC, 2022
//!

use std::io;
use std::net;


pub enum IOBridge {
    None,
    Console(io::Stdout, io::Stdin),
    TcpClient(net::TcpStream),
}

impl IOBridge {

    pub fn new_console() -> IOBridge {
        Self::Console(io::stdout(), io::stdin())
    }

    pub fn new_tcpclient(addr: &str) -> IOBridge {
        let ts = net::TcpStream::connect(addr).expect("ERRR:FuzzerK:IOType:TcpClient:Connect");
        Self::TcpClient(ts)
    }

    pub fn new(ioaddr: &str) -> IOBridge {
        let ioaddr = ioaddr.to_lowercase();
        if ioaddr == "console" {
            return Self::new_console()
        }
        let ioa = ioaddr.split_once(':').expect("ERRR:FuzzerK:IOType:New:Setting up nw");
        if ioa.0 == "tcpclient" {
            return Self::new_tcpclient(ioa.1);
        }
        Self::None
    }

    pub fn write(&mut self, buf: Vec<u8>) -> Result<usize, String> {
        Ok(0)
    }

    pub fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }

}
