//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use core::time;
use std::collections::HashMap;
use std::thread;

use fuzzerk::cfgfiles;
use fuzzerk::rtm;
use loggerk::*;
use argsclsk;
use fuzzerk::iob;


///
/// Specify the config file which sets up the fuzzers and the fuzzchains
/// * --cfgfc <path/file>
///
/// Specify the fuzzchain to run
/// * --fc <fcname>
///
/// Specify how many times the fuzzchain should be run
/// * --loopcnt <number>
///
/// Specify the io type to use and its type specific address
/// * --ioaddr <iotype:addr>
///   * console
///   * tcpclient:ipaddress:port
///   * tlsclient:ipaddress:port
///
/// Specify additional arguments if any for the io modules
/// * --ioarg <key>=<value>
///
fn handle_cmdline() -> (String, String, usize, String, HashMap<String, String>) {
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
        loopcnt = usize::from_str_radix(&args[iarg+1], 10).expect(&format!("ERRR:MFuzzerKU:HandleCmdline:Invalid loopcnt:{}", loopcnt));
        1
    };
    clargs.add_handler("--loopcnt", &mut loopcnt_handler);

    let mut ioaddr = String::from("console");
    let mut ioaddr_handler = |iarg: usize, args: &Vec<String>|-> usize {
        ioaddr = args[iarg+1].clone();
        1
    };
    clargs.add_handler("--ioaddr", &mut ioaddr_handler);

    let mut ioargs: HashMap<String, String> = HashMap::new();
    let mut ioarg_handler = |iarg: usize, args: &Vec<String>|-> usize {
        let ioarg = args[iarg+1].clone();
        let (k,v) = ioarg.split_once("=").expect(&format!("ERRR:MFuzzerKU:HandleCmdline:IOArg:{}", ioarg));
        ioargs.insert(k.to_string(), v.to_string());
        1
    };
    clargs.add_handler("--ioarg", &mut ioarg_handler);

    clargs.process_args();

    return (cfgfc, fc, loopcnt, ioaddr, ioargs);
}


fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");

    let (cfgfc, fc, loopcnt, ioaddr, ioargs) = handle_cmdline();

    let mut rtm = rtm::RunTimeManager::new();
    cfgfiles::parse_file(&cfgfc, &mut rtm);

    let runcmds: Vec<String> = Vec::new();

    let mut zenio = iob::IOBridge::None;
    let mut istep = 0;
    let mut gotfuzz = Vec::<u8>::new();
    let mut icur = 0;
    loop {
        icur += 1;
        let cmd =  runcmds[icur];
        if cmd == "iob new" {
            if let iob::IOBridge::None = zenio {
            } else {
                // close existing zenio
            }
            zenio = iob::IOBridge::new(&ioaddr, &ioargs);
        }
        if cmd.starts_with("fc") {
            let (fctag, fcid) = cmd.split_once(" ").unwrap();
            let fci = rtm.fcimmuts(&fc).unwrap();
            gotfuzz = fci.get(istep);
            log_d(&format!("\n\nGot:{}:\n\t{:?}\n\t{}", istep, gotfuzz, String::from_utf8_lossy(&gotfuzz)));
            istep += 1;
        }
        if cmd == "iob write" {
            let gotr = zenio.write(&gotfuzz);
            if gotr.is_err() {
                log_e(&format!("ERRR:MFuzzerKU:ZenIOWrite:{}:{}", i, gotr.unwrap_err()));
            }
        }
        if cmd == "iob flush" {
            let gotr = zenio.flush();
            if gotr.is_err() {
                log_e(&format!("ERRR:MFuzzerKU:ZenIOFlush:{}:{}", i, gotr.unwrap_err()));
            }
        }
        if cmd.starts_with("sys sleep") {
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
