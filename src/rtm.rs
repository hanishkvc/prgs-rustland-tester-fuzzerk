//!
//! RunTimeManager - helps create and maintain the list of fuzzers and fuzzchains
//! specified using cfgfiles mechanism.
//!
//! HanishKVC, 2022
//!

use std::collections::{HashMap, VecDeque};
use std::rc::Rc;
use std::cell::RefCell;

use loggerk::log_d;

use crate::{Fuzz, FuzzChain};
use crate::cfgfiles::{FromVecStrings, HandleCfgGroup};
use crate::{fixed, random};


const TYPEMARKER_FUZZER: &str = "FuzzerType";
const TYPEMARKER_FUZZCHAIN: &str = "FuzzChain";

pub struct RunTimeManager {
    fuzzers: HashMap<String, Rc<RefCell<dyn Fuzz>>>,
    fchains: HashMap<String, FuzzChain>,
}

impl RunTimeManager {

    pub fn new() -> RunTimeManager {
        RunTimeManager {
            fuzzers: HashMap::new(),
            fchains: HashMap::new(),
        }
    }

    pub fn fchain(&mut self, name: &str) -> Option<&mut FuzzChain> {
        let fchains = self.fchains.get_mut(name);
        if fchains.is_none() {
            return None;
        }
        let fchains = fchains.unwrap();
        return Some(fchains);
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
                    let fuzzer = Rc::new(RefCell::new(fuzzer));
                    log_d(&format!("DBUG:RunTimeManager:HandleCfgGroup:Created LoopFixedStringsFuzzer [{}] ie [{:?}]", la[2], fuzzer));
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                "RandomFixedStringsFuzzer" => {
                    let fuzzer = fixed::RandomFixedStringsFuzzer::from_vs(cg);
                    let fuzzer = Rc::new(RefCell::new(fuzzer));
                    log_d(&format!("DBUG:RunTimeManager:HandleCfgGroup:Created RandomFixedStringsFuzzer [{}] ie [{:?}]", la[2], fuzzer));
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                "RandomRandomFuzzer" => {
                    let fuzzer = random::RandomRandomFuzzer::from_vs(cg);
                    let fuzzer = Rc::new(RefCell::new(fuzzer));
                    log_d(&format!("DBUG:RunTimeManager:HandleCfgGroup:Created RandomRandomFuzzer [{}] ie [{:?}]", la[2], fuzzer));
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                "RandomFixedFuzzer" | "RandomFixedFuzzerPrintables" => {
                    let fuzzer = random::RandomFixedFuzzer::from_vs(cg);
                    let fuzzer = Rc::new(RefCell::new(fuzzer));
                    log_d(&format!("DBUG:RunTimeManager:HandleCfgGroup:Created RandomFixedFuzzer [{}] ie [{:?}]", la[2], fuzzer));
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                "Buf8sRandomizeFuzzer" => {
                    let fuzzer = random::Buf8sRandomizeFuzzer::from_vs(cg);
                    let fuzzer = Rc::new(RefCell::new(fuzzer));
                    log_d(&format!("DBUG:RunTimeManager:HandleCfgGroup:Created Buf8sRandomizeFuzzer [{}] ie [{:?}]", la[2], fuzzer));
                    self.fuzzers.insert(la[2].to_string(), fuzzer);
                },
                _ => panic!("ERRR:RunTimeManager:HandleCfgGroup:UnknownFuzzer:{:?}",la),
            }
        } else if la[0] == TYPEMARKER_FUZZCHAIN {
            let mut fc = FuzzChain::new();
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
            log_d(&format!("DBUG:RunTimeManager:HandleCfgGroup:Created FuzzChain [{}]", la[2]));
            self.fchains.insert(la[2].to_string(), fc);
        }
    }

}
