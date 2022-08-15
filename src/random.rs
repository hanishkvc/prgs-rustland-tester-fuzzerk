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
    fn append_fuzzed(&mut self, _step: usize, buf: &mut Vec<u8>) {
        let curlen: usize = rand::random();
        let curlen = self.minlen + curlen % (self.maxlen-self.minlen+1);
        for _i in 0..curlen {
            buf.push(rand::random());
        }
    }
}
