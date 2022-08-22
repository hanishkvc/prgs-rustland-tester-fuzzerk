//!
//! IO Bridge
//! 
//! HanishKVC, 2022
//!

use std::io;
use std::io::Write;
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
        let ts = net::TcpStream::connect(addr).expect("ERRR:FuzzerK:IOBridge:TcpClient:Connect");
        Self::TcpClient(ts)
    }

    pub fn new(ioaddr: &str) -> IOBridge {
        let ioaddr = ioaddr.to_lowercase();
        if ioaddr == "none" {
            return Self::None;
        }
        if ioaddr == "console" {
            return Self::new_console()
        }
        let ioa = ioaddr.split_once(':').expect("ERRR:FuzzerK:IOBridge:New:Setting up nw");
        if ioa.0 == "tcpclient" {
            return Self::new_tcpclient(ioa.1);
        }
        Self::None
    }

    pub fn write(&mut self, buf: &Vec<u8>) -> Result<usize, String> {
        match self {
            Self::None => todo!("ERRR:FuzzerK:IOBridge:Write:None:Why me???"),
            Self::Console(so, si ) => {
                let mut so = so.lock();
                let gotr = so.write_all(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Write:Console:{}", gotr.unwrap_err()))
                }
                return Ok(buf.len());
            }
            Self::TcpClient(ts) => {
                let gotr = ts.write_all(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Write:TcpClient:{}", gotr.unwrap_err()))
                }
                return Ok(buf.len());
            }
        }
        //Ok(0)
    }

    pub fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }

}
