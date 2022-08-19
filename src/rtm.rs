//!
//! RunTimeManager
//!
//! HanishKVC, 2022
//!

use std::collections::{HashMap, VecDeque};

use crate::Fuzz;
use crate::cfgfiles::FromStringVec;
use crate::fixed;

const TYPEMARKER_FUZZER: &str = "FuzzerType";
const TYPEMARKER_FUZZCHAIN: &str = "FuzzChain";

struct RunTimeManager<'a> {
    fuzzers: HashMap<String, &'a mut dyn Fuzz>,
}

impl<'a> RunTimeManager<'a> {
    pub fn new() -> RunTimeManager<'a> {
        RunTimeManager { fuzzers: HashMap::new() }
    }

    pub fn handle_cfggroup(cg: &mut VecDeque<String>) {
        let l = cg.front().unwrap();
        let la: Vec<&str> = l.split(':').collect();
        if la[0] == TYPEMARKER_FUZZER {
            match la[1] {
                "LoopFixedStringsFuzzer" => {
                    fixed::LoopFixedStringsFuzzer::from_sv(cg);
                },
                _ => todo!(),
            }
        }
    }

}
