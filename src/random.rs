//!
//! RandomFuzzer: Generate some random data
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;
use rand;

use crate::datautils;


///
/// Generate a buffer of random bytes, of size within the specified limits of min and max len.
///
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

    fn from_vs(vs: &mut VecDeque<String>) -> RandomRandomFuzzer {
        let l = vs.pop_front();
        if l.is_none() {
            panic!("ERRR:RandomRandomFuzzer:FromStringVec:Got empty vector");
        }
        let _l = l.unwrap(); // This should identify this particular type of Fuzzer and a runtime instance name
        let spacesprefix = Self::get_spacesprefix(vs);
        let minlen = Self::get_value(vs, "minlen", spacesprefix).expect("ERRR:RandomRandomFuzzer:GetMinLen:");
        let maxlen = Self::get_value(vs, "maxlen", spacesprefix).expect("ERRR:RandomRandomFuzzer:GetMaxLen:");
        let minlen = usize::from_str_radix(minlen.trim(), 10).expect(&format!("ERRR:RandomRandomFuzzer:MinLen issue:{}", minlen));
        let maxlen = usize::from_str_radix(maxlen.trim(), 10).expect(&format!("ERRR:RandomRandomFuzzer:MaxLen issue:{}", maxlen));
        RandomRandomFuzzer::new(minlen, maxlen)
    }
}


///
/// Generate a random buffer of bytes, which containts byte values from the specified set,
/// of size within the specified limits of min and max len.
///
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
        let minlen = usize::from_str_radix(minlen.trim(), 10).expect(&format!("ERRR:RandomFixedFuzzer:MinLen issue:{}", minlen));
        let maxlen = usize::from_str_radix(maxlen.trim(), 10).expect(&format!("ERRR:RandomFixedFuzzer:MaxLen issue:{}", maxlen));
        if la[1] == "RandomFixedFuzzerPrintables" {
            return RandomFixedFuzzer::new_printables(minlen, maxlen);
        }
        let charset = Self::get_value(vs, "charset", spacesprefix).expect("ERRR:RandomFixedFuzzer:GetCharSet:");
        let charset = datautils::vu8_from_hex(charset.trim()).expect("ERRR:RandomFixedFuzzer:GetCharSet:");
        RandomFixedFuzzer::new(minlen, maxlen, charset)
    }

}
