//!
//! RunTimeManager
//!
//! HanishKVC, 2022
//!

use std::collections::{HashMap, VecDeque};

use crate::Fuzz;
use crate::cfgfiles::FromVecStrings;
use crate::fixed;

const TYPEMARKER_FUZZER: &str = "FuzzerType";
const TYPEMARKER_FUZZCHAIN: &str = "FuzzChain";

struct RunTimeManager {
    fuzzers: HashMap<String, Box<dyn Fuzz>>,
}

impl RunTimeManager {
    pub fn new() -> RunTimeManager {
        RunTimeManager { fuzzers: HashMap::new() }
    }

    pub fn handle_cfggroup(&mut self, cg: &mut VecDeque<String>) {
        let l = cg.front().unwrap().clone();
        let la: Vec<&str> = l.split(':').collect();
        if la[0] == TYPEMARKER_FUZZER {
            match la[1] {
                "LoopFixedStringsFuzzer" => {
                    let fuzzer = fixed::LoopFixedStringsFuzzer::from_vs(cg);
                    let fuzzer = Box::new(fuzzer);
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                "RandomFixedStringsFuzzer" => {
                    let fuzzer = fixed::RandomFixedStringsFuzzer::from_vs(cg);
                    let fuzzer = Box::new(fuzzer);
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                _ => todo!(),
            }
        }
    }

}
