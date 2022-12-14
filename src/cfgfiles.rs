//!
//! Handle Config files
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead};

use datautilsk::hex;
use loggerk::{log_d, log_w, log_o};


const LIST_MAXVALUES: usize = 1024;


pub trait FromVecStrings {

    ///
    /// Get number of spaces in front of any text data, in the top most string in the vector.
    /// NOTE: The top most string is not removed from the vector.
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
    /// \t, \r, \n, \", \\
    ///
    fn str_deescape(ins: &str) -> Result<String, String> {
        let mut bescape = false;
        let mut outs = String::new();
        for c in ins.chars() {
            //println!("{}",c);
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
                        '"' => outs.push(c),
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
    /// Handle the provided string appropriately and return a Vec<u8>
    /// * trim the provided string wrt literal whitespaces at either side
    ///   * if one uses escape sequences like \t|\n|\r... at either end, such whitespaces will be retained
    /// * remove double quotes, if it was used to protect the string
    ///   * double quotes mainly help, when we want whitespaces at either end of the string
    ///   * if one wants the resultant string to contain double quotes at either end, put a 2nd double quote, where required.
    ///   * if there is double quotes only at one end of the string, it wont be removed.
    /// * interpret the given string has a hex string, if it starts with $0x
    fn strval_process(ins: &str) -> Result<Vec<u8>, String> {
        log_d(&format!("DBUG:FromVS:StrValProcess:{}:{}", Self::get_name(), ins));
        let mut outs = ins.trim();
        if outs.len() >= 2 {
            if outs.starts_with("$0x") {
                let vdata = hex::vu8_from_hex(&outs[3..]);
                return vdata;
            }
            let mut outschars = outs.chars();
            let startchar = outschars.nth(0).unwrap();
            let endchar = outschars.last().unwrap();
            if (startchar == endchar) && (startchar == '"') {
                outs = outs.strip_prefix('"').unwrap();
                outs = outs.strip_suffix('"').unwrap();
            }
            let outs = Self::str_deescape(outs);
            if outs.is_err() {
                return Err(outs.unwrap_err());
            }
            return Ok(Vec::from(outs.unwrap()));
        }
        Ok(Vec::from(outs))
    }

    ///
    /// Get the value (single) associated with the specified key. Empty value is Ok
    ///
    /// * The key-value pair should be specified as key:value | key: value | key: " value " | key: $0xvalue...
    /// * The key-value pair should be indented with whitespaces chars to match the specified spacesprefix.
    fn get_value_emptyok(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<Vec<u8>, String> {
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
        return Self::strval_process(lt.1);
    }

    ///
    /// Get the value (single) associated with the specified key. Empty value is not Ok with this.
    ///
    /// * The key-value pair should be specified as key:value | key: value | key: " value " | key: $0xvalue...
    /// * The key-value pair should be indented with whitespaces chars to match the specified spacesprefix.
    fn get_value(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<Vec<u8>, String> {
        let val = Self::get_value_emptyok(vs, key, spacesprefix);
        if val.is_err() {
            return val;
        }
        let val = val.unwrap();
        if val.len() == 0 {
            return Err(format!("ERRR:FromVS:GetValue:{}:No value given for {:?}", key, val));
        }
        return Ok(val);
    }

    ///
    /// Get the int value associated with the specified key. Empty value is not Ok with this.
    ///
    fn get_ivalue(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<isize, String> {
        let svalue = Self::get_value(vs, key, spacesprefix);
        if svalue.is_err() {
            return Err(svalue.unwrap_err());
        }
        let svalue = String::from_utf8(svalue.unwrap()).unwrap();
        let ivalue = isize::from_str_radix(svalue.trim(), 10);
        if ivalue.is_err() {
            return Err(format!("ERRR:FromVS:GetIValue:{}:Conversion of {} to Int err", key, svalue))
        }
        Ok(ivalue.unwrap())
    }

    ///
    /// Retrieve the list of values associated with the specified key
    /// * key needs to be in its own line with
    ///   * empty/no value following it OR
    ///     * \[WHITESPACE*\]SomeKey:
    ///   * a single value immidiately following it OR
    ///     * \[WHITESPACE*\]SomeKey: A_Single Value
    ///   * a count of the number of values which is contained in this list
    ///     * \[WHITESPACE*\]SomeKey: NumOfValues
    /// * each value in a multivalue list needs to be on its own line, indented further in compared to its key, with a optional ',' termination
    ///   * \[WHITESPACE*\]WHITESPACE*The Value
    ///   * the WHITESPACES at begin and end of the Value string will be trimmed.
    ///   * even if the optional ',' termination char is used, any spaces at end of the value string before ',' will be trimmed.
    ///
    fn get_values(vs: &mut VecDeque<String>, key: &str, spacesprefix: usize) -> Result<Vec<Vec<u8>>, String> {
        let sheadval = Self::get_value_emptyok(vs, key, spacesprefix);
        if sheadval.is_err() {
            return Err(sheadval.unwrap_err());
        }
        let sheadval = String::from_utf8(sheadval.unwrap()).unwrap();
        let numvalues;
        let mut vdata = Vec::new();
        if sheadval.len() != 0 {
            let tnumvalues = usize::from_str_radix(&sheadval, 10);
            if tnumvalues.is_err() {
                let curline = Self::strval_process(&sheadval);
                if curline.is_err() {
                    return Err(curline.unwrap_err());
                }
                log_w(&format!("WARN:FromVS:GetValues:{}-{}:Assuming list has only a single value [{:?}]", Self::get_name(), key, curline));
                vdata.push(curline.unwrap());
                return Ok(vdata);
            }
            numvalues = tnumvalues.unwrap();
            if numvalues > LIST_MAXVALUES {
                log_w(&format!("WARN:FromVS:GetValues:{}-{}:Assuming list of values contains [{}] values", Self::get_name(), key, numvalues));
            } else {
                log_o(&format!("INFO:FromVS:GetValues:{}-{}:Assuming list of values contains [{}] values", Self::get_name(), key, numvalues));
            }
        } else {
            numvalues = LIST_MAXVALUES;
            log_w(&format!("WARN:FromVS:GetValues:{}-{}:Will accept max of [{}] values for this list", Self::get_name(), key, numvalues));
        }
        let childsp = Self::get_spacesprefix(vs);
        let mut icurvalue = 0;
        while icurvalue <= numvalues {
            icurvalue += 1;
            let cursp = Self::get_spacesprefix(vs);
            if (cursp == spacesprefix) || (cursp == 0) {
                break;
            }
            if childsp != cursp {
                if cursp == spacesprefix {
                    break;
                }
                return Err(format!("ERRR:FromVS:GetValues:{}:Prefix whitespaces mismatch wrt values or ???:{} != {}", key, childsp, cursp));
            }
            let l = vs.pop_front();
            let curline = l.unwrap();
            let mut curline = curline.trim();
            if curline.chars().last().unwrap() == ',' {
                curline = curline.strip_suffix(",").unwrap();
            }
            let curline = Self::strval_process(curline);
            if curline.is_err() {
                return Err(curline.unwrap_err());
            }
            let curline = curline.unwrap();
            log_d(&format!("DBUG:FromVS:GetValues:{}-{}:[{:?}]", Self::get_name(), key, curline));
            vdata.push(curline);
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
