//!
//! RandomFuzzer: Generate some random data
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;
use rand;



///
/// Generate a buffer of random bytes, of size within the specified limits of min and max len.
///
#[derive(Debug)]
pub struct RandomRandomFuzzer {
    minlen: usize,
    maxlen: usize,
}

impl RandomRandomFuzzer {
    pub fn new(minlen: usize, maxlen: usize) -> RandomRandomFuzzer {
        RandomRandomFuzzer {
            minlen,
            maxlen,
        }
    }
}

impl super::Fuzz for RandomRandomFuzzer {
    fn append_fuzzed_immut(&self, _step: usize, buf: &mut Vec<u8>) {
        let curlen: usize = rand::random();
        let curlen = self.minlen + curlen % (self.maxlen-self.minlen+1);
        for _i in 0..curlen {
            buf.push(rand::random());
        }
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        return self.append_fuzzed_immut(step, buf);
    }
}

impl crate::cfgfiles::FromVecStrings for RandomRandomFuzzer {

    fn get_name() -> String {
        return "RandomRandomFuzzer".to_string();
    }

    ///
    /// The config file should contain below, for this
    /// ### FuzzerType:RandomRandomFuzzer:InstanceName
    /// * this requires the below keys
    ///   * minlen
    ///   * maxlen
    ///
    fn from_vs(vs: &mut VecDeque<String>) -> RandomRandomFuzzer {
        let l = vs.pop_front();
        if l.is_none() {
            panic!("ERRR:RandomRandomFuzzer:FromStringVec:Got empty vector");
        }
        let _l = l.unwrap(); // This should identify this particular type of Fuzzer and a runtime instance name
        let spacesprefix = Self::get_spacesprefix(vs);
        let minlen = Self::get_value(vs, "minlen", spacesprefix).expect("ERRR:RandomRandomFuzzer:GetMinLen:");
        let maxlen = Self::get_value(vs, "maxlen", spacesprefix).expect("ERRR:RandomRandomFuzzer:GetMaxLen:");
        let minlen = String::from_utf8(minlen).unwrap();
        let maxlen = String::from_utf8(maxlen).unwrap();
        let minlen = usize::from_str_radix(minlen.as_str(), 10).expect(&format!("ERRR:RandomRandomFuzzer:MinLen issue:{}", minlen));
        let maxlen = usize::from_str_radix(maxlen.as_str(), 10).expect(&format!("ERRR:RandomRandomFuzzer:MaxLen issue:{}", maxlen));
        RandomRandomFuzzer::new(minlen, maxlen)
    }
}


///
/// Generate a random buffer of bytes, which containts byte values from the specified set,
/// of size within the specified limits of min and max len.
///
#[derive(Debug)]
pub struct RandomFixedFuzzer {
    minlen: usize,
    maxlen: usize,
    charset: Vec<u8>,
}

impl RandomFixedFuzzer {
    pub fn new(minlen: usize, maxlen: usize, charset: Vec<u8>) -> RandomFixedFuzzer {
        RandomFixedFuzzer {
            minlen,
            maxlen,
            charset,
        }
    }

    ///
    /// Created a instance, setup to generate printable ascii chars
    ///
    pub fn new_printables(minlen: usize, maxlen: usize) -> RandomFixedFuzzer {
        //let charset = vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u',];
        let mut charset = Vec::new();
        for i in 32..127 {
            charset.push(i);
        }
        Self::new(minlen, maxlen, charset)
    }
}

impl super::Fuzz for RandomFixedFuzzer {
    fn append_fuzzed_immut(&self, _step: usize, buf: &mut Vec<u8>) {
        let curlen: usize = rand::random();
        let curlen = self.minlen + curlen % (self.maxlen-self.minlen+1);
        for _i in 0..curlen {
            let char = self.charset[rand::random::<usize>()%self.charset.len()];
            buf.push(char);
        }
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        return self.append_fuzzed_immut(step, buf);
    }
}

impl crate::cfgfiles::FromVecStrings for RandomFixedFuzzer {

    fn get_name() -> String {
        return "RandomFixedFuzzer".to_string();
    }

    ///
    /// In the cfgfile use either
    /// ### FuzzerType:RandomFixedFuzzer:InstanceName
    /// * this requieres the following keys
    ///   * minlen
    ///   * maxlen
    ///   * charset
    /// * If the charset can contain any binary value, one needs to define this has a hexstring (with 0x prefix)
    /// ### FuzzerType:RandomFixedFuzzerPrintables:InstanceName
    /// * this requires only the below keys
    ///   * minlen
    ///   * maxlen
    ///
    fn from_vs(vs: &mut VecDeque<String>) -> RandomFixedFuzzer {
        let l = vs.pop_front();
        if l.is_none() {
            panic!("ERRR:RandomFixedFuzzer:FromStringVec:Got empty vector");
        }
        let l = l.unwrap(); // This should identify this particular type of Fuzzer and a runtime instance name
        let la: Vec<&str> = l.split(":").collect();
        let spacesprefix = Self::get_spacesprefix(vs);
        let minlen = Self::get_value(vs, "minlen", spacesprefix).expect("ERRR:RandomFixedFuzzer:GetMinLen:");
        let maxlen = Self::get_value(vs, "maxlen", spacesprefix).expect("ERRR:RandomFixedFuzzer:GetMaxLen:");
        let minlen = String::from_utf8(minlen).unwrap();
        let maxlen = String::from_utf8(maxlen).unwrap();
        let minlen = usize::from_str_radix(minlen.trim(), 10).expect(&format!("ERRR:RandomFixedFuzzer:MinLen issue:{}", minlen));
        let maxlen = usize::from_str_radix(maxlen.trim(), 10).expect(&format!("ERRR:RandomFixedFuzzer:MaxLen issue:{}", maxlen));
        if la[1] == "RandomFixedFuzzerPrintables" {
            return RandomFixedFuzzer::new_printables(minlen, maxlen);
        }
        let charset = Self::get_value(vs, "charset", spacesprefix).expect("ERRR:RandomFixedFuzzer:GetCharSet:");
        RandomFixedFuzzer::new(minlen, maxlen, charset)
    }

}

///
/// Generate a random purterbance on a given buffer of bytes (buf8).
/// Its values are manipulated based on the following settings
/// randcount, startoffset, endoffset, startval, endval
///
#[derive(Debug)]
pub struct Buf8RandomizeFuzzer {
    /// the buffer of bytes to operate on
    buf8: Vec<u8>,
    /// number of bytes to manipulate
    randcount: isize,
    /// location within buffer from where purterbances should be created
    startoffset: isize,
    /// location within buffer till which purterbances should be created
    endoffset: isize,
    /// starting of the binary byte value range, wrt what values can be randomly used
    startval: isize,
    /// ending of the binary byte value range (inclusive), wrt what values can be randomly used
    endval: isize,
}

impl Buf8RandomizeFuzzer {

    ///
    /// Create a instance
    ///
    pub fn new(buf8: Vec<u8>, mut randcount: isize, mut startoffset: isize, mut endoffset: isize, mut startval:isize, mut endval:isize) -> Buf8RandomizeFuzzer {
        let buflen = buf8.len() as isize;
        if randcount < 0 {
            randcount = ((rand::random::<usize>() % buf8.len()) + 1) as isize;
        }
        if startoffset < 0 {
            startoffset = 0;
        } else if startoffset >= buflen {
            startoffset = buflen-1;
        }
        if endoffset < 0 {
            endoffset = buflen;
        } else if endoffset > buflen {
            endoffset = buflen;
        }
        if startval < 0 {
            startval = 0;
        } else if startval > 255 {
            startval = 255;
        }
        if endval < 0 {
            endval = 255;
        } else if endval > 255 {
            endval = 255;
        }
        Buf8RandomizeFuzzer {
            buf8,
            randcount,
            startoffset,
            endoffset,
            startval,
            endval
        }
    }

}

impl super::Fuzz for Buf8RandomizeFuzzer {
    fn append_fuzzed_immut(&self, _step: usize, buf: &mut Vec<u8>) {
        let valuerange: usize = (self.endval - self.startval + 1) as usize;
        let offsetrange: usize = (self.endoffset - self.startoffset) as usize;
        let mut inb = self.buf8.clone();
        for _i in 0..self.randcount {
            let char = (self.startval as usize + (rand::random::<usize>() % valuerange)) as u8;
            let ipos = (self.startoffset as usize + (rand::random::<usize>() % offsetrange)) as usize;
            inb[ipos] = char;
        }
        buf.append(&mut inb)
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        return self.append_fuzzed_immut(step, buf);
    }
}

impl crate::cfgfiles::FromVecStrings for Buf8RandomizeFuzzer {

    fn get_name() -> String {
        return "Buf8RandomizeFuzzer".to_string();
    }

    ///
    /// In the cfgfile use
    /// ### FuzzerType:Buf8RandomizeFuzzer:InstanceName
    /// * this requieres the following keys
    ///   * buf8: a textual or hex string
    ///   * randcount: -1 (decide randomly) | 0 | +ve integer
    ///   * startoffset: -1 (0) | 0 | +ve integer < buf8.len()
    ///   * endoffset: -1 (buf8.len()) | 0 | +ve integer <= buf8.len()
    ///   * startval: -1 (0) | 0 | +ve integer <= 255
    ///   * endval: -1 (255) | 0 | +ve integer <= 255
    ///
    fn from_vs(vs: &mut VecDeque<String>) -> Buf8RandomizeFuzzer {
        let l = vs.pop_front();
        if l.is_none() {
            panic!("ERRR:Buf8RandomizeFuzzer:FromStringVec:Got empty vector");
        }
        let l = l.unwrap(); // This should identify this particular type of Fuzzer and a runtime instance name
        let la: Vec<&str> = l.split(":").collect();
        if la[1] != "Buf8RandomizeFuzzer" {
            panic!("ERRR:Buf8RandomizeFuzzer:FromStringVec:Mismatch wrt Fuzzer Type????");
        }
        let spacesprefix = Self::get_spacesprefix(vs);
        let buf8 = Self::get_value(vs, "buf8", spacesprefix).expect("ERRR:Buf8RandomizeFuzzer:GetBuf8:");
        let randcount = Self::get_ivalue(vs, "randcount", spacesprefix).expect("ERRR:Buf8RandomizeFuzzer:GetRandCount:");
        let startoffset = Self::get_ivalue(vs, "startoffset", spacesprefix).expect("ERRR:Buf8RandomizeFuzzer:GetStartOffset:");
        let endoffset = Self::get_ivalue(vs, "endoffset", spacesprefix).expect("ERRR:Buf8RandomizeFuzzer:GetEndOffset:");
        let startval = Self::get_ivalue(vs, "startval", spacesprefix).expect("ERRR:Buf8RandomizeFuzzer:GetStartVal:");
        let endval = Self::get_ivalue(vs, "endval", spacesprefix).expect("ERRR:Buf8RandomizeFuzzer:GetEndVal:");
        Buf8RandomizeFuzzer::new(buf8, randcount, startoffset, endoffset, startval, endval)
    }

}
