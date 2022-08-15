//!
//! FixedFuzzer: Cycle through the list of provided data
//!
//! HanishKVC, 2022
//!

pub struct FixedStringsFuzzer {
    list: Vec<String>,
    curi: isize,
}

impl FixedStringsFuzzer {
    pub fn new(fixed_list: Vec<String>) -> FixedStringsFuzzer {
        FixedStringsFuzzer {
            list: fixed_list,
            curi: -1,
        }
    }
}

impl super::Fuzz for FixedStringsFuzzer {
    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        self.curi = (step % self.list.len()) as isize;
        let tosend = self.list[step].clone();
        for b in tosend.bytes() {
            buf.push(b)
        }
    }
}

