//!
//! Handle Config files
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead};
use loggerk::log_d;


pub trait FromVecStrings {

    ///
    /// Get number of spaces in front of any text data, in the top most string in the vector.
    ///
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

    ///
    /// Support a subset of escape chars like
    /// \t, \r, \n
    ///
    fn str_deescape(ins: &str) -> Result<String, String> {
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
                        _ => return Err(format!("ERRR:CfgFiles:StrDeEscape:Unsupported escape char {}", c)),
                    }
                    bescape = false;
                } else {
                    outs.push(c)
                }
            }
        }
        Ok(outs)
    }

    ///
    /// Get the value (single) associated with the specified key. Empty value is Ok
    ///
    /// The key - value pair should be specified as key:value
    ///
    ///     * All data following ':' till end of line, will be returned as value, after trimming for spaces at either ends.
    ///     * The key:value pair should be indented with whitespaces chars to match the specified spacesprefix.
    ///
    fn get_value_emptyok(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<String, String> {
        let cursp = Self::get_spacesprefix(vs);
        if cursp != spacesprefix {
            return Err(format!("ERRR:FromVS:GetValueEmptyOk:{}:Prefix whitespaces mismatch:{} != {}", key, spacesprefix, cursp));
        }
        let l = vs.pop_front();
        if l.is_none() {
            return Err(format!("ERRR:FromVS:GetValueEmptyOk:{}:No data to process", key))
        }
        let l = l.unwrap();
        let l = l.trim();
        let lt = l.split_once(':');
        if lt.is_none() {
            return Err(format!("ERRR:FromVS:GetValueEmptyOk:{}:key-value delimiter ':' missing", key));
        }
        let lt = lt.unwrap();
        if lt.0 != key {
            return Err(format!("ERRR:FromVS:GetValueEmptyOk:{}:Expected key {}, got key {}", key, key, lt.0));
        }
        log_d(&format!("DBUG:FromVS:GetValueEmptyOk:{}-{}:[{:?}]", Self::get_name(), key, lt));
        return Self::str_deescape(lt.1.trim());
    }

    ///
    /// Get the value (single) associated with the specified key. Empty value is not Ok with this.
    ///
    fn get_value(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<String, String> {
        let val = Self::get_value_emptyok(vs, key, spacesprefix);
        if val.is_err() {
            return val;
        }
        let val = val.unwrap();
        if val.len() == 0 {
            return Err(format!("ERRR:FromVS:GetValue:{}:No value given for {}", key, val));
        }
        return Ok(val);
    }

    ///
    /// Retrieve the list of values associated with the specified key
    /// * key needs to be in its own line with empty/no value following it
    ///   * \[WHITESPACE*\]SomeKey:
    /// * each value needs to be on its own line, with a optional ',' termination
    ///   * \[WHITESPACE*\]WHITESPACE*The Value
    ///   * the WHITESPACES at begin and end of the Value string will be trimmed.
    ///   * if the optional ',' termination char is used, then any spaces at end of the value string before ','
    ///     will be retained.
    ///
    fn get_values(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<Vec<String>, String> {
        let sheadval = Self::get_value_emptyok(vs, key, spacesprefix);
        if sheadval.is_err() {
            return Err(sheadval.unwrap_err());
        }
        let sheadval = sheadval.unwrap();
        if sheadval.len() != 0 {
            return Err(format!("ERRR:FromVS:GetValues:{}:Non array value {} found???", key, sheadval));
        }
        let childsp = Self::get_spacesprefix(vs);
        let mut vdata = Vec::new();
        loop {
            let cursp = Self::get_spacesprefix(vs);
            if (cursp == spacesprefix) || (cursp == 0) {
                break;
            }
            if childsp != cursp {
                return Err(format!("ERRR:FromVS:GetValues:{}:Prefix whitespaces mismatch:{} != {}", key, spacesprefix, cursp));
            }
            let l = vs.pop_front();
            let curline = l.unwrap();
            let mut curline = curline.trim();
            if curline.chars().last().unwrap() == ',' {
                curline = curline.strip_suffix(",").unwrap();
            }
            let curline = Self::str_deescape(curline);
            if curline.is_err() {
                return Err(curline.unwrap_err());
            }
            let curline = curline.unwrap();
            log_d(&format!("DBUG:FromVS:GetValues:{}-{}:[{:?}]", Self::get_name(), key, curline));
            vdata.push(curline.to_string());
        }
        Ok(vdata)
    }

    ///
    /// The name of the struct / enum implementing this trait.
    ///
    fn get_name() -> String;

    ///
    /// Implementer should parse the passed Vector of Strings and Generator a instance of itself.
    ///
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
        log_d(&format!("CfgFiles:CfgGroup:{:#?}", cgdata));
        handler.handle_cfggroup(&mut cgdata);
    }
}
