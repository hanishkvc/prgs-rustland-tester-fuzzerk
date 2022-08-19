//!
//! RunTimeManager
//!
//! HanishKVC, 2022
//!

use std::collections::{HashMap, VecDeque};


const TYPEMARKER_FUZZER: String = "FuzzerType";
const TYPEMARKER_FUZZCHAIN: String = "FuzzChain";

struct RunTimeManager<'a> {
    fuzzers: HashMap<String, &'a mut dyn Fuzz>,
}

impl<'a> RunTimeManager<'a> {
    pub fn new() -> RunTimeManager<'a> {
        RunTimeManager { fuzzers: HashMap::new() }
    }

    pub fn handle_cfggroup(cg: VecDeque<String>) {
        let l = cg.front().unwrap();
        let la: Vec<&str> = l.split(':').collect();
        if la[0] == TYPEMARKER_FUZZER {
            match la[1] {

            }
        }
    }

}
