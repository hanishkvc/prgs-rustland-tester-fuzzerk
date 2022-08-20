//!
//! RunTimeManager
//!
//! HanishKVC, 2022
//!

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

use crate::{Fuzz, FuzzChainImmuts};
use crate::cfgfiles::{FromVecStrings, HandleCfgGroup};
use crate::fixed;

const TYPEMARKER_FUZZER: &str = "FuzzerType";
const TYPEMARKER_FUZZCHAIN: &str = "FuzzChain";

pub struct RunTimeManager {
    fuzzers: HashMap<String, Rc<dyn Fuzz>>,
    fcimmuts: HashMap<String, Rc<FuzzChainImmuts>>,
}

impl RunTimeManager {

    pub fn new() -> RunTimeManager {
        RunTimeManager {
            fuzzers: HashMap::new(),
            fcimmuts: HashMap::new(),
        }
    }

    pub fn fcimmuts(&self, name: &str) -> Option<Rc<FuzzChainImmuts>> {
        let fcimmuts = self.fcimmuts.get(name);
        if fcimmuts.is_none() {
            return None;
        }
        let fcimmuts = fcimmuts.unwrap();
        return Some(fcimmuts.clone());
    }

}

impl HandleCfgGroup for RunTimeManager {

    fn handle_cfggroup(&mut self, cg: &mut VecDeque<String>) {
        let l = cg.front().unwrap().clone();
        let mut la: Vec<&str> = l.split(':').collect();
        la[2] = la[2].trim();
        if la[0] == TYPEMARKER_FUZZER {
            match la[1] {
                "LoopFixedStringsFuzzer" => {
                    let fuzzer = fixed::LoopFixedStringsFuzzer::from_vs(cg);
                    let fuzzer = Rc::new(fuzzer);
                    println!("DBUG:RunTimeManager:HandleCfgGroup:Created LoopFixedStringsFuzzer [{}]", la[2]);
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                "RandomFixedStringsFuzzer" => {
                    let fuzzer = fixed::RandomFixedStringsFuzzer::from_vs(cg);
                    let fuzzer = Rc::new(fuzzer);
                    println!("DBUG:RunTimeManager:HandleCfgGroup:Created RandomFixedStringsFuzzer [{}]", la[2]);
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                _ => todo!(),
            }
        } else if la[0] == TYPEMARKER_FUZZCHAIN {
            let mut fc = FuzzChainImmuts::new();
            let _l = cg.pop_front(); // Skip the Type identifier
            for l in cg {
                let l = l.trim();
                let fuzzer = self.fuzzers.get(l);
                if fuzzer.is_none() {
                    panic!("ERRR:RunTimeManager:HandleCfgGroup:Reference to unknown fuzzer {}", l);
                }
                let fuzzer = fuzzer.unwrap();
                fc.append(fuzzer.clone());
            }
            let fc = Rc::new(fc);
            self.fcimmuts.insert(la[2].to_string(), fc);
        }
    }

}
