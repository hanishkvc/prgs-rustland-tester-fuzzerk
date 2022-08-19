//!
//! FixedFuzzer: Cycle through the list of provided data
//!
//! HanishKVC, 2022
//!

///
/// Loop through a predefined list of strings, in given sequence
///
pub struct LoopFixedStringsFuzzer {
    list: Vec<String>,
    curi: usize,
}

impl LoopFixedStringsFuzzer {
    pub fn new(fixed_list: Vec<String>) -> LoopFixedStringsFuzzer {
        LoopFixedStringsFuzzer {
            list: fixed_list,
            curi: 0,
        }
    }
}

impl super::Fuzz for LoopFixedStringsFuzzer {
    fn append_fuzzed_immut(&self, step: usize, buf: &mut Vec<u8>) {
        if self.list.len() == 0 {
            eprintln!("ERRR:FixedStringsFuzzer:AppendFuzzed:Step {}: Empty list to work with", step);
            return;
        }
        let curi = step % self.list.len();
        let tosend = self.list[curi].clone();
        for b in tosend.bytes() {
            buf.push(b)
        }
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        self.curi = step % self.list.len();
        self.append_fuzzed_immut(step, buf);
    }
}

impl super::FromStringVec<LoopFixedStringsFuzzer> for LoopFixedStringsFuzzer {
    fn from_sv(sv: &mut Vec<String>) -> LoopFixedStringsFuzzer {
        todo!()
    }
}

///
/// Randomly select from predefined list of strings
///
pub struct RandomFixedStringsFuzzer {
    list: Vec<String>,
}

impl RandomFixedStringsFuzzer {
    pub fn new(fixed_list: Vec<String>) -> RandomFixedStringsFuzzer {
        RandomFixedStringsFuzzer {
            list: fixed_list,
        }
    }
}

impl super::Fuzz for RandomFixedStringsFuzzer {
    fn append_fuzzed_immut(&self, step: usize, buf: &mut Vec<u8>) {
        if self.list.len() == 0 {
            eprintln!("ERRR:FixedStringsFuzzer:AppendFuzzed:Step {}: Empty list to work with", step);
            return;
        }
        let curi = rand::random::<usize>() % self.list.len();
        let tosend = self.list[curi].clone();
        for b in tosend.bytes() {
            buf.push(b)
        }
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        return self.append_fuzzed_immut(step, buf);
    }
}
