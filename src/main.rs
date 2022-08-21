//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use std::io::{self, Write};
use fuzzerk::cfgfiles;
use fuzzerk::rtm;
use loggerk::*;
use argsclsk;


fn handle_cmdline() -> (String, String) {
    let mut clargs = argsclsk::ArgsCmdLineSimpleManager::new();

    let mut cfgfc = String::new();
    let mut cfgfc_handler = |iarg: usize, args: &Vec<String>|-> usize {
        cfgfc = args[iarg+1].clone();
        1
    };
    clargs.add_handler("--cfgfc", &mut cfgfc_handler);

    let mut fc = String::new();
    let mut fc_handler = |iarg: usize, args: &Vec<String>|-> usize {
        fc = args[iarg+1].clone();
        1
    };
    clargs.add_handler("--fc", &mut fc_handler);

    clargs.process_args();

    return (cfgfc, fc);
}


fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");

    let (cfgfc, fc) = handle_cmdline();

    let mut rtm = rtm::RunTimeManager::new();
    cfgfiles::parse_file(&cfgfc, &mut rtm);
    let fci = rtm.fcimmuts(&fc).unwrap();


    let gotfuzz = fci.get(1);
    log_d(&format!("Got1:\n\t{:?}\n\t{}", gotfuzz, String::from_utf8_lossy(&gotfuzz)));

    let mut so = io::stdout().lock();
    let gotr = so.write_all(&gotfuzz);
    if gotr.is_err() {
        log_e(&format!("ERRR:MFKU:StdOutWrite:{}", gotr.unwrap_err()));
    }
}
