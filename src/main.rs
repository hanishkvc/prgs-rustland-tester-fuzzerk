//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use fuzzerk::cfgfiles;
use std::env;

fn main() {
    println!("MinimalFuzzerKUtil");
    let cla: Vec<String> = env::args().collect();
    cfgfiles::parse_file(cla[1].as_str());
}
