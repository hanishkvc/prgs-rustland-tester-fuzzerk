//!
//! FixedFuzzer: Cycle through the list of provided data
//!
//! HanishKVC, 2022
//!

pub struct FixedStringsFuzzer {
    list: Vec<String>,
    curi: usize,
}

impl FixedStringsFuzzer {
    pub fn new(fixed_list: Vec<String>) -> FixedStringsFuzzer {
        FixedStringsFuzzer {
            list: fixed_list,
            curi: 0,
        }
    }
}

impl super::Fuzz for FixedStringsFuzzer {
    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        if self.list.len() == 0 {
            eprintln!("ERRR:FixedStringsFuzzer:AppendFuzzed:Step {}: Empty list to work with", step);
            return;
        }
        self.curi = step % self.list.len();
        let tosend = self.list[self.curi].clone();
        for b in tosend.bytes() {
            buf.push(b)
        }
    }
}
