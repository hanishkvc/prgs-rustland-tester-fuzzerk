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


fn handle_cmdline() -> (String, String, usize) {
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

    let mut loopcnt = 1usize;
    let mut loopcnt_handler = |iarg: usize, args: &Vec<String>|-> usize {
        loopcnt = usize::from_str_radix(&args[iarg+1], 10).expect(&format!("ERRR:MFuzzerKU: Invalid loopcnt:{}", loopcnt));
        1
    };
    clargs.add_handler("--loopcnt", &mut loopcnt_handler);

    clargs.process_args();

    return (cfgfc, fc, loopcnt);
}


fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");

    let (cfgfc, fc, loopcnt) = handle_cmdline();

    let mut rtm = rtm::RunTimeManager::new();
    cfgfiles::parse_file(&cfgfc, &mut rtm);
    let fci = rtm.fcimmuts(&fc).unwrap();

    let mut so = io::stdout().lock();
    for i in 0..loopcnt {
        let gotfuzz = fci.get(i);
        log_d(&format!("\n\nGot:{}:\n\t{:?}\n\t{}", i, gotfuzz, String::from_utf8_lossy(&gotfuzz)));
        let gotr = so.write_all(&gotfuzz);
        if gotr.is_err() {
            log_e(&format!("ERRR:MFuzzerKU:StdOutWrite:{}:{}", i, gotr.unwrap_err()));
        }
        let gotr = so.flush();
        if gotr.is_err() {
            log_e(&format!("ERRR:MFuzzerKU:StdOutFlush:{}:{}", i, gotr.unwrap_err()));
        }
    }
}
