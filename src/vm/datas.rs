//!
//! Helpers wrt data used by VM
//! HanishKVC, 2022
//!

use std::time;

use rand::Rng;

use crate::datautils;


#[derive(Debug)]
pub enum VDataType {
    Unknown,
    Integer,
    String,
    Buffer,
    Special,
}


#[derive(Debug, Clone)]
pub enum Variant {
    IntValue(isize),
    StrValue(String),
    BufValue(Vec<u8>),
    XRandomBytes(usize),
    XTimeStamp,
}

impl Variant {

    pub fn get_type(&self) -> VDataType {
        match self {
            Variant::IntValue(_) => VDataType::Integer,
            Variant::StrValue(_) => VDataType::String,
            Variant::BufValue(_) => VDataType::Buffer,
            Variant::XRandomBytes(_) => VDataType::Special,
            Variant::XTimeStamp => VDataType::Special,
        }
    }

    ///
    /// * Int -> Int
    /// * String -> Try interpret the string as a textual literal value of a integer
    /// * Buf -> Try interpret the buf as the underlying raw byte values of a integer
    /// * XTimeStamp -> milliseconds from UnixEpoch truncated
    /// * XRandomBytes -> a randomly generated Int (limited to min(Int size,requested bytes))
    ///
    pub fn get_isize(&self) -> Result<isize, String> {
        match self {
            Self::IntValue(ival) => {
                return Ok(*ival);
            },
            Self::StrValue(sval) => {
                let ival = datautils::intvalue(sval);
                if ival.is_ok() {
                    return Ok(ival.unwrap());
                }
                return Err(format!("Variant:GetISize:StrValue:[{}]:Conversion failed:{}", sval, ival.unwrap_err()));
            },
            Self::BufValue(bval) => {
                let bsval = bval.as_slice().try_into();
                if bsval.is_ok() {
                    let bval = bsval.unwrap();
                    return Ok(isize::from_ne_bytes(bval));
                }
                return Err(format!("Variant:GetISize:BufValue:[{:?}]:Adapting buf for int failed? Wrong number of bytes or?:{}", bval, bsval.unwrap_err()));
            },
            Self::XTimeStamp => {
                let ts = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
                let uts = ts.as_millis();
                return Ok(uts as isize);
            },
            Self::XRandomBytes(bytelen) => {
                let mut rng = rand::thread_rng();
                let mut vdata: Vec<u8> = Vec::new();
                let mut ibytes = (isize::BITS/8) as usize;
                let irem;
                if ibytes > *bytelen {
                    irem = ibytes - *bytelen;
                    ibytes = *bytelen;
                } else {
                    return Err(format!("WARN:Variant:GetISize:XRandomBytes:Specified length {}, greater than int length {}", *bytelen, ibytes));
                }
                for _i in 0..ibytes {
                    vdata.push(rng.gen_range(0..=255)); // rusty 0..256
                }
                for _i in 0..irem {
                    vdata.push(0);
                }
                return Ok(isize::from_ne_bytes(vdata.as_slice().try_into().unwrap()));
            }
        }
    }

    ///
    /// Return a positive interger value, this is built upon get_isize
    /// If the underlying value is negative, then it will panic
    ///
    #[allow(dead_code)]
    fn get_usize(&self) -> Result<usize, String> {
        let ival = self.get_isize();
        match ival {
            Ok(ival) => {
                if ival < 0 {
                    panic!("Variant:GetUSize: Negative int value not supported here")
                }
                return Ok(ival as usize);
            }
            Err(msg) => return Err(format!("Variant:GetUSize:{}", msg)),
        }
    }

    ///
    /// * Returns Int values as equivalent string literal form
    /// * Returns String as is
    /// * Returns Buf8 data as a hex string
    /// * XTimeStamp returns current System time converted to milliseconds since UNIX Epoch, as a string
    /// * XRandomBytes returns random generated bytes converted to string using utf8_lossy
    ///
    pub fn get_string(&self) -> String {
        match self {
            Self::IntValue(ival) => {
                return ival.to_string();
            },
            Self::StrValue(sval) => {
                return sval.to_string();
            },
            Self::BufValue(bval) => {
                return datautils::hex_from_vu8(bval);
            },
            Self::XTimeStamp => {
                let ts = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
                let uts = ts.as_millis();
                return uts.to_string();
            },
            Self::XRandomBytes(bytelen) => {
                let mut rng = rand::thread_rng();
                let mut vdata: Vec<u8> = Vec::new();
                for _i in 0..*bytelen {
                    vdata.push(rng.gen_range(0..=255)); // rusty 0..256
                }
                return String::from_utf8_lossy(&vdata).to_string();
            }
         }
    }

    ///
    /// * returns int values as underlying byte values based vector in the native endianess format
    /// * Returns String as the underlying byte values based vector
    /// * Returns Buf8 data as is (rather a cloned buf)
    /// * XTimeStamp -> milliseconds from UnixEpoch, as the underlying byte values of the int
    /// * XRandomBytes returns random generated bytes
    ///
    /// TODO:ThinkAgain: Should I return a fixed endian format like network byte order (BigEndian) or little endian
    /// rather than native byte order (If testing between systems having different endianess, it could help)
    pub fn get_bufvu8(&self) -> Vec<u8> {
        match self {
            Self::IntValue(ival) => {
                return ival.to_ne_bytes().to_vec();
            },
            Self::StrValue(sval) => {
                return Vec::from(sval.to_string());
            },
            Self::BufValue(bval) => {
                return bval.clone();
            },
            Self::XTimeStamp => {
                let ts = time::SystemTime::now().duration_since(time::UNIX_EPOCH).unwrap();
                let uts = ts.as_millis();
                return uts.to_ne_bytes().to_vec();
            },
            Self::XRandomBytes(bytelen) => {
                let mut rng = rand::thread_rng();
                let mut vdata: Vec<u8> = Vec::new();
                for _i in 0..*bytelen {
                    vdata.push(rng.gen_range(0..=255)); // rusty 0..256
                }
                return vdata;
            }
         }
    }

    pub fn get_bufvu8_mut(&mut self) -> Option<&mut Vec<u8>> {
        if let Self::BufValue(thebuf) = self {
            return Some(thebuf.as_mut());
        }
        return None;
    }

}