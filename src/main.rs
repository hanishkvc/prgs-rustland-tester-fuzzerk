//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use std::env;
use fuzzerk::cfgfiles;
use fuzzerk::rtm;

fn main() {
    println!("MinimalFuzzerKUtil");
    let cla: Vec<String> = env::args().collect();
    let mut rtm = rtm::RunTimeManager::new();
    cfgfiles::parse_file(cla[1].as_str(), &mut rtm);
}
