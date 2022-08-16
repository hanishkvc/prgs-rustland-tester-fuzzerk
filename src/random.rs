//!
//! RandomFuzzer: Generate some random data
//!
//! HanishKVC, 2022
//!

use rand;

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
