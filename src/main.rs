//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use std::{collections::HashMap, process};

use loggerk::*;
use argsclsk;
use fuzzerk::vm;


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
/// Enable logging/printing of debug messages, if required using
/// * --blogdebug <true|yes>
///
fn handle_cmdline() -> (String, String, usize, String, HashMap<String, String>, String, bool) {
    let mut clargs = argsclsk::ArgsCmdLineSimpleManager::new();

    let mut cfgfc = String::new();
    let mut cfgfc_handler = |iarg: usize, args: &Vec<String>|-> usize {
        cfgfc = args[iarg+1].clone();
        1
    };
    clargs.add_handler("--cfgfc", &mut cfgfc_handler);

    let mut prgfile = String::new();
    let mut prgfile_handler = |iarg: usize, args: &Vec<String>|-> usize {
        prgfile = args[iarg+1].clone();
        1
    };
    clargs.add_handler("--prgfile", &mut prgfile_handler);

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

    let mut blogdebug = false;
    let mut blogdebug_handler = |iarg: usize, args: &Vec<String>| -> usize {
        let slogdebug = &args[iarg+1];
        let vyes = vec!["yes", "true"];
        if vyes.contains(&slogdebug.as_str()) {
            blogdebug = true;
        }
        1
    };
    clargs.add_handler("--blogdebug", &mut blogdebug_handler);

    clargs.process_args();

    return (cfgfc, fc, loopcnt, ioaddr, ioargs, prgfile, blogdebug);
}



fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");

    let (cfgfc, fc, loopcnt, ioaddr, ioargs, prgfile, blogdebug) = handle_cmdline();
    log_config(true, true, true, blogdebug, true);

    let mut vm = vm::VM::new();
    if cfgfc.len() == 0 {
        log_o(&format!("NOTE:FuzzerK:Args: --cfgfc <Fuzz++CfgFile> is a simple mechanism to create fuzzers and fuzzchains, usable in most cases"));
    }
    vm.load_fcrtm(&cfgfc);
    if prgfile.len() == 0 {
        if loopcnt <= 1 {
            log_o(&format!("NOTE:FuzzerK:Args: --loopcnt <ANumber> allows one to control how many times to loop through fuzzchain generation and io handshake"));
        }
        if fc.len() == 0 {
            log_w(&format!("WARN:FuzzerK:Args: If no --prgfile <ThePrgFile>, then --fc <FuzzChainId> is needed along with --cfgfc <Fuzz++Cfgfile>"));
            process::exit(1);
        }
        vm.predefined_prg(&fc, loopcnt, &ioaddr, &ioargs);
    } else {
        vm.load_prg(&prgfile);
    }

    vm.run();
}
