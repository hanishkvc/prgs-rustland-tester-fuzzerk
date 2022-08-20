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
    let fci = rtm.fcimmuts(cla[2].as_str()).unwrap();
    let got1 = fci.get(1);
    println!("Got1:\n\t{:?}\n\t{}", got1, String::from_utf8_lossy(&got1));
    let got2 = fci.get(2);
    println!("Got2:\n\t{:?}\n\t{}", got2, String::from_utf8_lossy(&got2));
}
