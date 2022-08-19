//!
//! Handle Config files
//!
//! HanishKVC, 2022
//!

use std::collections::VecDeque;


pub trait FromStringVec {
    fn get_spacesprefix(sv: &VecDeque<String>) -> usize {
        let l = sv.front();
        let mut spacesprefix = 0;
        if l.is_some() {
            let l = l.unwrap();
            for c in l.chars() {
                if !c.is_whitespace() {
                    break;
                }
                spacesprefix += 1;
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
        let spacesprefix = Self::get_spacesprefix(sv);
        while true {
            let cursp = Self::get_spacesprefix(sv);
            if spacesprefix != cursp {
                panic!("ERRR:FromStringVec:{}-{}:Prefix whitespaces mismatch:{} != {}", Self::get_name(), key, spacesprefix, cursp);
            }
            let l = sv.pop_front();
            let curline = l.unwrap();
        }
        Vec::new()
    }

    fn get_name() -> String;
    fn from_sv(sv: &mut VecDeque<String>) -> Self;
}
