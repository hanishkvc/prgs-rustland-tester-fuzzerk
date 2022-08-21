//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use std::env;
use fuzzerk::cfgfiles;
use fuzzerk::rtm;
use loggerk::*;

fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");
    let cla: Vec<String> = env::args().collect();
    let mut rtm = rtm::RunTimeManager::new();
    cfgfiles::parse_file(cla[1].as_str(), &mut rtm);
    let fci = rtm.fcimmuts(cla[2].as_str()).unwrap();
    let got1 = fci.get(1);
    log_i(&format!("Got1:\n\t{:?}\n\t{}", got1, String::from_utf8_lossy(&got1)));
    let got2 = fci.get(2);
    log_i(&format!("Got2:\n\t{:?}\n\t{}", got2, String::from_utf8_lossy(&got2)));
}
