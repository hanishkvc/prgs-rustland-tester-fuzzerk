//!
//! Handle Config files
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead};


pub trait FromVecStrings {
    fn get_spacesprefix(vs: &VecDeque<String>) -> usize {
        let l = vs.front();
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

    fn str_deescape(ins: &str) -> String {
        let mut bescape = false;
        let mut outs = String::new();
        for c in ins.chars() {
            if c == '\\' {
                if bescape {
                    outs.push(c);
                    bescape = false;
                } else {
                    bescape = true;
                }
            } else {
                if bescape {
                    match c {
                        't' => outs.push('\x09'),
                        'n' => outs.push('\x0A'),
                        'r' => outs.push('\x0D'),
                        _ => panic!("ERRR:CfgFiles:StrDeEscape:Unsupported escape char {}", c),
                    }
                    bescape = false;
                } else {
                    outs.push(c)
                }
            }
        }
        outs
    }

    fn get_value_emptyok(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> String {
        let cursp = Self::get_spacesprefix(vs);
        if cursp != spacesprefix {
            panic!("ERRR:FromStringVec:{}-{}:Prefix whitespaces mismatch:{} != {}", Self::get_name(), key, spacesprefix, cursp);
        }
        let l = vs.pop_front();
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
        println!("DBUG:FromStringVec:{}-{}:[{:?}]", Self::get_name(), key, lt);
        let val = Self::str_deescape(lt.1);
        return val;
    }

    fn get_value(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> String {
        let val = Self::get_value_emptyok(vs, key, spacesprefix);
        if val.len() == 0 {
            panic!("ERRR:FromStringVec:{}-{}:No value given for {}", Self::get_name(), key, val);
        }
        return val;
    }

    ///
    /// Retrieve the list of values associated with the specified key
    /// * key needs to be in its own line with empty/no value following it
    ///   * \[WHITESPACE*\]SomeKey:
    /// * each value needs to be on its own line, with a optional ',' termination
    ///   * \[WHITESPACE*\]WHITESPACE*The Value
    ///
    fn get_values(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Vec<String> {
        let sheadval = Self::get_value_emptyok(vs, key, spacesprefix);
        if sheadval.len() != 0 {
            panic!("ERRR:FromStringVec:{}-{}:has non array value {} ???", Self::get_name(), key, sheadval);
        }
        let childsp = Self::get_spacesprefix(vs);
        let mut vdata = Vec::new();
        loop {
            let cursp = Self::get_spacesprefix(vs);
            if (cursp == spacesprefix) || (cursp == 0) {
                break;
            }
            if childsp != cursp {
                panic!("ERRR:FromStringVec:{}-{}:Prefix whitespaces mismatch:{} != {}", Self::get_name(), key, spacesprefix, cursp);
            }
            let l = vs.pop_front();
            let curline = l.unwrap();
            let mut curline = curline.trim();
            if curline.chars().last().unwrap() == ',' {
                curline = curline.strip_suffix(",").unwrap();
            }
            let curline = Self::str_deescape(curline);
            println!("DBUG:FromStringVec:{}-{}:[{:?}]", Self::get_name(), key, curline);
            vdata.push(curline.to_string());
        }
        vdata
    }

    fn get_name() -> String;
    fn from_vs(vs: &mut VecDeque<String>) -> Self;
}


///
/// Retreive the next CfgGroup from the given file
/// * Empty lines at the begining are skipped.
/// * Lines starting with # are treated as comments and ignored
///   where ever thye may be.
/// * A CfgGroup needs to begin with a line, which has AlphaNumeric char
///   at the 0th position itself.
///   * Ideally subsequent lines in the CfgGroup should be indented, but
///     this logic doesnt bother about same. It is for any handler of the
///     CfgGroup to enforce such requirement.
/// * A empty line, after a bunch of non empty lines, terminates a CfgGroup.
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


pub trait HandleCfgGroup {
    fn handle_cfggroup(&mut self, vs: &mut VecDeque<String>);
}


pub fn parse_file(sfile: &str, handler: &mut dyn HandleCfgGroup) {
    let f = File::open(sfile);
    if f.is_err() {
        panic!("ERRR:CfgFiles:ParseFile:{}:{}", sfile, f.unwrap_err());
    }
    let f = f.unwrap();
    let mut fbr = BufReader::new(f);
    loop {
        let mut cgdata = get_cfggroup(&mut fbr);
        if cgdata.len() == 0 {
            break;
        }
        println!("CfgFiles:CfgGroup:{:#?}", cgdata);
        handler.handle_cfggroup(&mut cgdata);
    }
}
