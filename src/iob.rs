//!
//! IO Bridge
//! 
//! HanishKVC, 2022
//!

use std::io;
use std::io::Write;
use std::net;
use boring::ssl;


pub enum IOBridge {
    None,
    Console(io::Stdout, io::Stdin),
    TcpClient(net::TcpStream),
    TlsClient(ssl::SslStream<net::TcpStream>),
}

impl IOBridge {

    pub fn new_console() -> IOBridge {
        Self::Console(io::stdout(), io::stdin())
    }

    pub fn new_tcpclient(addr: &str) -> IOBridge {
        let ts = net::TcpStream::connect(addr).expect("ERRR:FuzzerK:IOBridge:TcpClient:TcpStreamConnect");
        Self::TcpClient(ts)
    }

    pub fn new_tlsclient(addr: &str) -> IOBridge {
        let msgtag = "FuzzerK:IOBridge:TlsClient";
        let (taddr, tdomain) = addr.split_once(",").expect(&format!("ERRR:{}:Extract Addr and Domain", msgtag));
        let tlsconn = ssl::SslConnector::builder(ssl::SslMethod::tls()).expect(&format!("ERRR:{}:SslConnectorBuilder", msgtag)).build();
        let tcpstream = net::TcpStream::connect(taddr).expect(&format!("ERRR:{}:TcpStreamConnect", msgtag));
        let tlsstream = tlsconn.connect(tdomain, tcpstream).expect(&format!("ERRR:{}:SslConnectorConnect", msgtag));
        Self::TlsClient(tlsstream)
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
        if ioa.0 == "tlsclient" {
            return Self::new_tlsclient(ioa.1);
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
            },
            Self::TcpClient(ts) => {
                let gotr = ts.write_all(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Write:TcpClient:{}", gotr.unwrap_err()))
                }
                return Ok(buf.len());
            },
            Self::TlsClient(ss) => {
                let gotr = ss.write_all(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Write:TlsClient:{}", gotr.unwrap_err()))
                }
                return Ok(buf.len());
            },
        }
        //Ok(0)
    }

    pub fn flush(&mut self) -> Result<(), String> {
        match self {
            Self::None => todo!("ERRR:FuzzerK:IOBridge:Flush:None:Why me???"),
            Self::Console(so, si ) => {
                let mut so = so.lock();
                let gotr = so.flush();
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Flush:Console:{}", gotr.unwrap_err()))
                }
                return Ok(());
            },
            Self::TcpClient(ts) => {
                let gotr = ts.flush();
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Flush:TcpClient:{}", gotr.unwrap_err()))
                }
                return Ok(());
            },
            Self::TlsClient(ss) => {
                let gotr = ss.flush();
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Flush:TlsClient:{}", gotr.unwrap_err()))
                }
                return Ok(());
            },
        }
        //Ok(())
    }

}
