//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use fuzzerk::cfgfiles;
use fuzzerk::rtm;
use loggerk::*;
use argsclsk;
use fuzzerk::iob;


fn handle_cmdline() -> (String, String, usize, String) {
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

    let mut ioaddr = String::from("console");
    let mut ioaddr_handler = |iarg: usize, args: &Vec<String>|-> usize {
        ioaddr = args[iarg+1].clone();
        1
    };
    clargs.add_handler("--ioaddr", &mut ioaddr_handler);

    clargs.process_args();

    return (cfgfc, fc, loopcnt, ioaddr);
}


fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");

    let (cfgfc, fc, loopcnt, ioaddr) = handle_cmdline();

    let mut rtm = rtm::RunTimeManager::new();
    cfgfiles::parse_file(&cfgfc, &mut rtm);
    let fci = rtm.fcimmuts(&fc).unwrap();

    let mut zenio = iob::IOBridge::new(&ioaddr);
    for i in 0..loopcnt {
        let gotfuzz = fci.get(i);
        log_d(&format!("\n\nGot:{}:\n\t{:?}\n\t{}", i, gotfuzz, String::from_utf8_lossy(&gotfuzz)));
        let gotr = zenio.write(gotfuzz);
        if gotr.is_err() {
            log_e(&format!("ERRR:MFuzzerKU:ZenIOWrite:{}:{}", i, gotr.unwrap_err()));
        }
        let gotr = zenio.flush();
        if gotr.is_err() {
            log_e(&format!("ERRR:MFuzzerKU:ZenIOFlush:{}:{}", i, gotr.unwrap_err()));
        }
    }
}
