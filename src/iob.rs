//!
//! IO Bridge
//! 
//! HanishKVC, 2022
//!

use std::collections::HashMap;
use std::io;
use std::io::Read;
use std::io::Write;
use std::net;
use std::fs;
use std::time::Duration;
use boring::ssl;

use loggerk::{log_d, log_e};


pub enum IOBridge {
    None,
    Console(io::Stdout, io::Stdin),
    TcpClient(net::TcpStream),
    TlsClient(ssl::SslStream<net::TcpStream>),
    FileWriter(fs::File),
}

impl IOBridge {

    pub fn new_console() -> IOBridge {
        Self::Console(io::stdout(), io::stdin())
    }

    ///
    /// Supported IOArgs
    /// * read_timeout=millisecs
    ///
    pub fn new_tcpclient(addr: &str, ioargs: &HashMap<String, String>) -> IOBridge {
        let invalid = String::from("INVALID");
        let read_timeout = ioargs.get("read_timeout").or(Some(&invalid)).unwrap();

        let ts = net::TcpStream::connect(addr).expect("ERRR:FuzzerK:IOBridge:TcpClient:TcpStreamConnect");
        if *read_timeout != invalid {
            let timeout_millis = u64::from_str_radix(&read_timeout, 10).expect("ERRR:FuzzerK:IOBridge:TcpClient:New:ReadTimeout");
            let tomillis = Duration::from_millis(timeout_millis);
            ts.set_read_timeout(Some(tomillis)).expect("ERRR:FuzzerK:IOBridge:TcpClient:New:SetReadTimeout");
        }
        Self::TcpClient(ts)
    }

    ///
    /// Supported IOArgs
    /// * server_cert_check=yes/no
    /// * domain=the.domain.name
    /// * read_timeout=millisecs
    ///
    pub fn new_tlsclient(addr: &str, ioargs: &HashMap<String, String>) -> IOBridge {
        let msgtag = "FuzzerK:IOBridge:TlsClient";
        let yes = String::from("yes");
        let invalid = String::from("INVALID");

        let read_timeout = ioargs.get("read_timeout").or(Some(&invalid)).unwrap();
        let servercertcheck = ioargs.get("server_cert_check").or(Some(&yes)).unwrap();
        let domain = ioargs.get("domain").expect(&format!("ERRR:{}:domain missing", msgtag));

        let mut tlsconnbldr = ssl::SslConnector::builder(ssl::SslMethod::tls()).expect(&format!("ERRR:{}:SslConnectorBuilder", msgtag));
        if servercertcheck == "no" {
            tlsconnbldr.set_verify(ssl::SslVerifyMode::NONE);
        }
        let tlsconn = tlsconnbldr.build();
        let tcpstream = net::TcpStream::connect(addr).expect(&format!("ERRR:{}:TcpStreamConnect", msgtag));
        if *read_timeout != invalid {
            let timeout_millis = u64::from_str_radix(&read_timeout, 10).expect(&format!("ERRR:{}:New:ReadTimeout", msgtag));
            let tomillis = Duration::from_millis(timeout_millis);
            tcpstream.set_read_timeout(Some(tomillis)).expect(&format!("ERRR:{}:New:SetReadTimeout", msgtag));
        }
        let tlsstream = tlsconn.connect(domain, tcpstream).expect(&format!("ERRR:{}:SslConnectorConnect", msgtag));
        Self::TlsClient(tlsstream)
    }

    ///
    /// Supported IOArgs
    /// * append=yes/no (default: yes)
    /// * create=yes/no (default: no)
    ///   create will truncate any existing file
    ///
    pub fn new_filewriter(addr: &str, ioargs: &HashMap<String, String>) -> IOBridge {
        let msgtag = "FuzzerK:IOBridge:FileWriter:New:";
        let yes = String::from("yes");
        let maybe = String::from("maybe");

        let mut append = ioargs.get("append").or(Some(&maybe)).unwrap();
        let create = ioargs.get("create").or(Some(&maybe)).unwrap();

        if *append == maybe && *create == maybe {
            append = &yes;
        }

        let file: fs::File;
        if append == "yes" {
            file = fs::File::options().append(true).open(addr).expect(&format!("ERRR:{}:OpenAppend", msgtag));
        } else {
            if create == "yes" {
                file = fs::File::create(addr).expect(&format!("ERRR:{}:Create", msgtag));
            } else {
                panic!("ERRR:{}:Either append or create ioarg needs to be specified and inturn yes", msgtag);
            }
        }
        Self::FileWriter(file)
    }

    ///
    /// The ioaddr passed could be one of the following
    /// * none
    /// * console
    /// * tcpclient:addr:port
    /// * tlsclient:addr:port
    /// * filewriter:path/to/file
    ///
    /// NOTE: Address could be ip address or domain name
    ///
    pub fn new(ioaddr: &str, ioargs: &HashMap<String, String>) -> IOBridge {
        let ioaddr = ioaddr.to_lowercase();
        if ioaddr == "none" {
            return Self::None;
        }
        if ioaddr == "console" {
            return Self::new_console()
        }
        let ioa = ioaddr.split_once(':').expect("ERRR:FuzzerK:IOBridge:New:Setting up nw");
        if ioa.0 == "tcpclient" {
            return Self::new_tcpclient(ioa.1, ioargs);
        }
        if ioa.0 == "tlsclient" {
            return Self::new_tlsclient(ioa.1, ioargs);
        }
        if ioa.0 == "filewriter" {
            return Self::new_filewriter(ioa.1, ioargs);
        }
        Self::None
    }

    pub fn write(&mut self, buf: &Vec<u8>) -> Result<usize, String> {
        match self {
            Self::None => todo!("ERRR:FuzzerK:IOBridge:Write:None:Why me???"),
            Self::Console(so, _si ) => {
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
            Self::FileWriter(file) => {
                let gotr = file.write_all(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Write:FileWriter:{}", gotr.unwrap_err()))
                }
                return Ok(buf.len());
            }
        }
        //Ok(0)
    }

    pub fn flush(&mut self) -> Result<(), String> {
        match self {
            Self::None => todo!("ERRR:FuzzerK:IOBridge:Flush:None:Why me???"),
            Self::Console(so, _si ) => {
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
            Self::FileWriter(file) => {
                let gotr = file.flush();
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Flush:FileWriter:{}", gotr.unwrap_err()))
                }
                return Ok(());
            },
        }
        //Ok(())
    }

    pub fn read(&mut self, buf: &mut Vec<u8>) -> Result<usize, String> {
        match self {
            Self::None => todo!("ERRR:FuzzerK:IOBridge:Read:None:Why me???"),
            Self::Console(_so, si ) => {
                let mut si = si.lock();
                //let gotr = si.read_to_end(buf);
                let gotr = si.read(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Read:Console:{}", gotr.unwrap_err()))
                }
                return Ok(gotr.unwrap());
            },
            Self::TcpClient(ts) => {
                let gotr = ts.read(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Read:TcpClient:{}", gotr.unwrap_err()))
                }
                return Ok(gotr.unwrap());
            },
            Self::TlsClient(ss) => {
                let gotr = ss.read(buf);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Read:TlsClient:{}", gotr.unwrap_err()))
                }
                return Ok(gotr.unwrap());
            },
            Self::FileWriter(_file) => {
                panic!("ERRR:FuzzerK:IOBridge:Read:FileWriter:Not supported");
            }
        }
        //Ok(0)
    }

    pub fn close(&mut self) -> Result<(), String> {
        match self {
            Self::TcpClient(ts) => {
                let gotr = ts.shutdown(net::Shutdown::Both);
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Close:TcpClient:{}", gotr.unwrap_err()))
                }
                return Ok(());
            },
            Self::TlsClient(ss) => {
                let gotr = ss.shutdown();
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Close:TlsClient:S1:{}", gotr.unwrap_err()))
                }
                if *gotr.as_ref().unwrap() == ssl::ShutdownResult::Sent {
                    log_d("DBUG:FuzzerK:IOBridge:Close:TlsClient:S1:GotSent");
                } else {
                    log_e(&format!("ERRR:FuzzerK:IOBridge:Close:TlsClient:S1:NotSent???:{:?}", gotr.unwrap()));
                }
                // Rather keeping it simple, ignoring any additional data that might be there to read etc
                return Ok(());
            },
            Self::FileWriter(file) => {
                let gotr = file.sync_all();
                if gotr.is_err() {
                    return Err(format!("ERRR:FuzzerK:IOBridge:Close:FileWriter:SyncAll:{}", gotr.unwrap_err()))
                }
                drop(file);
                return Ok(());
            }
            _ => {},
        }
        Ok(())
    }

}
