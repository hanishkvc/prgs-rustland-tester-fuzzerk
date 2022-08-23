//!
//! A minimal FuzzerK util
//!
//! HanishKVC, 2022
//!

use core::time;
use std::collections::HashMap;
use std::thread;

use fuzzerk::rtm;
use loggerk::*;
use argsclsk;
use fuzzerk::iob;
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
fn handle_cmdline() -> (String, String, usize, String, HashMap<String, String>, String) {
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

    clargs.process_args();

    return (cfgfc, fc, loopcnt, ioaddr, ioargs, prgfile);
}



fn main() {
    log_init();
    log_o("MinimalFuzzerKUtil");

    let (cfgfc, fc, loopcnt, ioaddr, ioargs, prgfile) = handle_cmdline();

    let mut vm = vm::VM::new();
    vm.load_fcrtm(&cfgfc);
    if prgfile.len() == 0 {
        vm.predefined_prg(&fc, loopcnt);
    } else {
        vm.load_prg(&prgfile);
    }

    let mut zenio = iob::IOBridge::None;
    let mut istep = 0;
    let mut gotfuzz = Vec::<u8>::new();
    let mut loopcnt = 0;
    let mut icmd = 0;
    loop {
        let cmd =  &runcmds[icmd];
        log_d(&format!("INFO:MFuzzerKU:Cmd:{}:{}", icmd, cmd));
        if cmd.starts_with("fc ") {
            let (_fctag, fcid) = cmd.split_once(" ").unwrap();
            let fci = rtm.fcimmuts(&fcid).unwrap();
            gotfuzz = fci.get(istep);
            log_d(&format!("\n\nGot:{}:\n\t{:?}\n\t{}", istep, gotfuzz, String::from_utf8_lossy(&gotfuzz)));
            istep += 1;
        }
        if cmd.starts_with("sys sleep") {
            thread::sleep(time::Duration::from_secs(1));
        }
        if cmd.starts_with("loop") {
            let cmdparts: Vec<&str> = cmd.split(" ").collect();
            let loopcmd = cmdparts[1];
            if loopcmd == "inc" {
                loopcnt += 1;
            } else {
                let loopcheck = usize::from_str_radix(cmdparts[2], 10).unwrap();
                let foricmd = usize::from_str_radix(cmdparts[4], 10).unwrap();
                let adjtype = cmdparts[3];
                match loopcmd {
                    "iflt" => {
                        if loopcnt >= loopcheck {
                            break;
                        }
                    },
                    _ => todo!("ERRR:MFuzzerKU:Unknow loop cmd")
                }
                if adjtype == "relpos" {
                    icmd += foricmd;
                    continue;
                } else {
                    icmd = foricmd;
                    continue;
                }
            }
        }
        icmd += 1;
    }
}
