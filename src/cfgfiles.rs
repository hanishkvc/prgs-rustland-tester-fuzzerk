//!
//! Handle Config files
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead};


pub trait FromStringVec {
    fn get_spacesprefix(sv: &VecDeque<String>) -> usize {
        let l = sv.front();
        let mut spacesprefix = 0;
        if l.is_some() {
            let l = l.unwrap();
            if l.trim().len() != 0 {
                for c in l.chars() {
                    if !c.is_whitespace() {
                        break;
                    }
                    spacesprefix += 1;
                }
            }
        }
        return spacesprefix;
    }

    fn get_value(sv: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> String {
        let cursp = Self::get_spacesprefix(sv);
        if cursp != spacesprefix {
            panic!("ERRR:FromStringVec:{}-{}:Prefix whitespaces mismatch:{} != {}", Self::get_name(), key, spacesprefix, cursp);
        }
        let l = sv.pop_front();
        if l.is_none() {
            panic!("ERRR:FromStringVec:{}-{}:No data to process", Self::get_name(), key)
        }
        let l = l.unwrap();
        let l = l.trim();
        let lt = l.split_once(':');
        if lt.is_none() {
            panic!("ERRR:FromStringVec:{}-{}:key-value delimiter ':' missing", Self::get_name(), key)
        }
        let lt = lt.unwrap();
        if lt.0 != key {
            panic!("ERRR:FromStringVec:{}-{}:Expected key {}, got key {}", Self::get_name(), key, key, lt.0)
        }
        if lt.1.len() == 0 {
            panic!("ERRR:FromStringVec:{}-{}:No value given for {}", Self::get_name(), key, lt.0)
        }
        return lt.1.to_string();
    }

    fn get_values(sv: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Vec<String> {
        let sheadval = Self::get_value(sv, key, spacesprefix);
        if sheadval.len() != 0 {
            panic!("ERRR:FromStringVec:{}-{}:has non array value {} ???", Self::get_name(), key, sheadval);
        }
        let childsp = Self::get_spacesprefix(sv);
        let mut vdata = Vec::new();
        loop {
            let cursp = Self::get_spacesprefix(sv);
            if (cursp == spacesprefix) || (cursp == 0) {
                break;
            }
            if childsp != cursp {
                panic!("ERRR:FromStringVec:{}-{}:Prefix whitespaces mismatch:{} != {}", Self::get_name(), key, spacesprefix, cursp);
            }
            let l = sv.pop_front();
            let curline = l.unwrap();
            vdata.push(curline);
        }
        vdata
    }

    fn get_name() -> String;
    fn from_sv(sv: &mut VecDeque<String>) -> Self;
}


fn get_cfggroup(fbr: &mut BufReader<File>) -> VecDeque<String> {
    let mut vdata = VecDeque::new();
    loop {
        let mut sbuf = String::new();
        let gotr = fbr.read_line(&mut sbuf);
        if gotr.is_err() {
            panic!("ERRR:CfgFiles:GetCfgGroup:read failed {}", gotr.unwrap_err());
        }
        let gotr = gotr.unwrap();
        if gotr == 0 {
            break;
        }
        if sbuf.trim().len() == 0 {
            if vdata.len() > 0 {
                break;
            }
            continue;
        }
        let c = sbuf.chars().nth(0).expect("ERRR:CfgFiles:GetCfgGroup:While checking for CfgGroup start or Comment");
        if c == '#' { // Skip comments
            continue;
        }
        if vdata.len() == 0 {
            if !c.is_alphanumeric() {
                panic!("ERRR:CfgFiles:GetCfgGroup:Didnt get the expected start of a new CfgGroup:{}", sbuf);
            }
        }
        vdata.push_back(sbuf)
    }
    vdata
}

fn parse_file(sfile: &str) {
    let f = File::open(sfile);
    if f.is_err() {
        panic!("ERRR:CfgFiles:ParseFile:{}:{}", sfile, f.unwrap_err());
    }
    let f = f.unwrap();
    let mut fbr = BufReader::new(f);
    loop {
        let cgdata = get_cfggroup(&mut fbr);
        if cgdata.len() == 0 {
            break;
        }
    }
}
