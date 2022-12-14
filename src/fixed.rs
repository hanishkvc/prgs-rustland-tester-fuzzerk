//!
//! FixedFuzzer: Cycle through the list of provided data
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;

use loggerk::log_e;

///
/// Loop through a predefined list of strings, in given sequence
///
#[derive(Debug)]
pub struct LoopFixedStringsFuzzer {
    list: Vec<Vec<u8>>,
    curi: usize,
}

impl LoopFixedStringsFuzzer {
    pub fn new(fixed_list: Vec<Vec<u8>>) -> LoopFixedStringsFuzzer {
        LoopFixedStringsFuzzer {
            list: fixed_list,
            curi: 0,
        }
    }
}

impl super::Fuzz for LoopFixedStringsFuzzer {
    fn append_fuzzed_immut(&self, step: usize, buf: &mut Vec<u8>) {
        if self.list.len() == 0 {
            log_e(&format!("ERRR:FixedStringsFuzzer:AppendFuzzed:Step {}: Empty list to work with", step));
            return;
        }
        let curi = step % self.list.len();
        let tosend = self.list[curi].clone();
        for b in tosend {
            buf.push(b)
        }
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        self.curi = step % self.list.len();
        self.append_fuzzed_immut(step, buf);
    }
}

impl crate::cfgfiles::FromVecStrings for LoopFixedStringsFuzzer {

    fn get_name() -> String {
        return "LoopFixedStringsFuzzer".to_string();
    }

    ///
    /// The config file should contain below, for this
    /// ### FuzzerType:LoopFixedStringsFuzzer:InstanceName
    /// * this requires the below keys
    ///   * list
    ///     * specify list of strings to use for fuzz generation
    ///
    fn from_vs(vs: &mut VecDeque<String>) -> LoopFixedStringsFuzzer {
        let l = vs.pop_front();
        if l.is_none() {
            panic!("ERRR:LoopFixedStringsFuzzer:FromStringVec:Got empty vector");
        }
        let _l = l.unwrap(); // This should identify this particular type of Fuzzer and a runtime instance name
        let spacesprefix = Self::get_spacesprefix(vs);
        let fixedlist = Self::get_values(vs, "list", spacesprefix).expect("ERRR:LoopFixedStringsFuzzer:GetList:");
        LoopFixedStringsFuzzer::new(fixedlist)
    }
}


///
/// Randomly select from predefined list of strings
///
#[derive(Debug)]
pub struct RandomFixedStringsFuzzer {
    list: Vec<Vec<u8>>,
}

impl RandomFixedStringsFuzzer {
    pub fn new(fixed_list: Vec<Vec<u8>>) -> RandomFixedStringsFuzzer {
        RandomFixedStringsFuzzer {
            list: fixed_list,
        }
    }
}

impl super::Fuzz for RandomFixedStringsFuzzer {
    fn append_fuzzed_immut(&self, step: usize, buf: &mut Vec<u8>) {
        if self.list.len() == 0 {
            log_e(&format!("ERRR:FixedStringsFuzzer:AppendFuzzed:Step {}: Empty list to work with", step));
            return;
        }
        let curi = rand::random::<usize>() % self.list.len();
        let tosend = self.list[curi].clone();
        for b in tosend {
            buf.push(b)
        }
    }

    fn append_fuzzed(&mut self, step: usize, buf: &mut Vec<u8>) {
        return self.append_fuzzed_immut(step, buf);
    }
}

impl crate::cfgfiles::FromVecStrings for RandomFixedStringsFuzzer {

    fn get_name() -> String {
        return "RandomFixedStringsFuzzer".to_string();
    }

    ///
    /// The config file should contain below, for this
    /// ### FuzzerType:RandomFixedStringsFuzzer:InstanceName
    /// * this requires the below keys
    ///   * list
    ///     * specify list of strings to use for fuzz generation
    ///
    fn from_vs(vs: &mut VecDeque<String>) -> RandomFixedStringsFuzzer {
        let l = vs.pop_front();
        if l.is_none() {
            panic!("ERRR:RandomFixedStringsFuzzer:FromStringVec:Got empty vector");
        }
        let _l = l.unwrap(); // This should identify this particular type of Fuzzer and a runtime instance name
        let spacesprefix = Self::get_spacesprefix(vs);
        let fixedlist = Self::get_values(vs, "list", spacesprefix).expect("ERRR:RandomFixedStringsFuzzer:GetList:");
        RandomFixedStringsFuzzer::new(fixedlist)
    }
}
