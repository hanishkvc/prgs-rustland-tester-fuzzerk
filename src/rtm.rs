//!
//! RunTimeManager
//!
//! HanishKVC, 2022
//!

use std::collections::{HashMap, VecDeque};

use crate::{Fuzz, FuzzChainImmuts};
use crate::cfgfiles::{FromVecStrings, HandleCfgGroup};
use crate::fixed;

const TYPEMARKER_FUZZER: &str = "FuzzerType";
const TYPEMARKER_FUZZCHAIN: &str = "FuzzChain";

pub struct RunTimeManager<'a> {
    fuzzers: HashMap<String, Box<dyn Fuzz>>,
    fcimmuts: HashMap<String, Box<FuzzChainImmuts<'a>>>,
}

impl<'a> RunTimeManager<'a> {

    pub fn new() -> RunTimeManager<'a> {
        RunTimeManager {
            fuzzers: HashMap::new(),
            fcimmuts: HashMap::new(),
        }
    }

}

impl<'a> HandleCfgGroup for RunTimeManager<'a> {

    fn handle_cfggroup(&mut self, cg: &mut VecDeque<String>) {
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
        } else if la[0] == TYPEMARKER_FUZZCHAIN {
            let fc = FuzzChainImmuts::new();
            let fc = Box::new(fc);
            let l = cg.pop_front(); // Skip the Type identifier
            for l in cg {
                let l = l.trim();
                let fuzzer = self.fuzzers.get(l);
                if fuzzer.is_none() {
                    panic!("ERRR:RunTimeManager:HandleCfgGroup:Reference to unknown fuzzer {}", l);
                }
                let fuzzer = fuzzer.unwrap();
                fc.append(fuzzer.as_ref());
            }
            self.fcimmuts.insert(la[2].to_string(), fc);
        }

    }

}
