//!
//! The Data (Manager) of VM
//! HanishKVC, 2022
//!

use crate::datautils;

use super::DataKind;
use super::datas::{Variant, VDataType};
use super::xopdata::XOpData;
use super::Context;


///
/// NOTE: The program logic currently implements a simple one pass compilation, which inturn
/// only does a partial/quasi ahead of time (AOT) compilation.
///
/// As a part and result of this mechanism, the DataM::compile
/// * identifies the type of literal value and inturn creates a appropriate variant entity
/// * identifies a possible var name and inturn sets it up as a variable.
///   However do keep in mind that this acts more like a pointer/alias to the actual underlying variable.
///   * the underlying var could be
///     * a function argument (is read only)
///     * a local variable OR
///     * a global variable.
///
#[derive(Debug, Clone)]
pub(crate) enum DataM {
    Value(Variant),
    Variable(DataKind, String),
    XOp(XOpData),
}


impl DataM {

    ///
    /// * int literals should start with numeric char
    /// * string literals should be in double quotes ""
    /// * buf8 literals should start with $0x
    /// * special literals should start with __
    /// * a xop+data should start with ! and end with )
    /// * anything else is treated as a Var name.
    ///   * it needs to start with a alpabhetic char
    ///   * it could either be a func arg or local variable or a global variable.
    ///
    pub fn compile(ctxt: &Context, sdata: &str, stype: &str, smsg: &str) -> DataM {
        let mut sdata = ctxt.tstrx.from_str(sdata, true);
        if sdata.remaining_len() == 0 {
            panic!("ERRR:{}:DataM:Compile:{}:Data token empty", smsg, stype);
        }
        let schar = sdata.char_first().unwrap();
        let echar = sdata.char_last().unwrap();

        if schar.is_numeric() || schar == '+' || schar == '-' {
            let idata = datautils::intvalue(sdata.the_str()).expect(&format!("ERRR:{}:DataM:Compile:IntLiteral[{}]:Conversion", smsg, sdata));
            return DataM::Value(Variant::IntValue(idata));
        }

        if sdata.len() >= 2 {

            if schar == '"' || echar == '"' {
                if schar != echar {
                    panic!("ERRR:{}:DataM:Compile:StringLiteral:Mising double quote at one of the ends:[{}]", smsg, sdata);
                }
                let tdata = sdata.nexttok(' ', true).expect(&format!("ERRR:{}:DataM:Compile:StringLiteral~[{}]:Processing...", smsg, sdata));
                if sdata.remaining_len() > 0 {
                    panic!("ERRR:{}:DataM:Compile:StringLiteral:Extra data [{}] beyond end of the string[{}]???", smsg, sdata, tdata);
                }
                let mut rdata = tdata.as_str();
                rdata = rdata.strip_prefix('"').expect(&format!("ERRR:{}:DataM:Compile:StringLiteral:Missing double quote at start of {}", smsg, sdata));
                rdata = rdata.strip_suffix('"').expect(&format!("ERRR:{}:DataM:Compile:StringLiteral:Missing double quote at end of {}", smsg, sdata));
                return DataM::Value(Variant::StrValue(rdata.to_string()));
            }

            if sdata.len() > 2 {
                if sdata.the_str().starts_with("$0x") {
                    let bdata = datautils::vu8_from_hex(&sdata.the_str()[3..]).expect(&format!("ERRR:{}:DataM:Compile:BufHexString:Conversion:{}", smsg, sdata));
                    return DataM::Value(Variant::BufValue(bdata));
                }
                if sdata.the_str().starts_with("__") {
                    if sdata.the_str() == "__TIME__STAMP__" {
                        return DataM::Value(Variant::XTimeStamp);
                    }
                    if sdata.the_str().starts_with("__RANDOM__BYTES__") {
                        let (_random, bytelen) = sdata.the_str().split_once("__BYTES__").expect(&format!("ERRR:{}:DataM:Compile:RandomBytes:{}", smsg, sdata));
                        let bytelen = usize::from_str_radix(bytelen, 10).expect(&format!("ERRR:{}:DataM:Compile:RandomBytes:{}", smsg, sdata));
                        return DataM::Value(Variant::XRandomBytes(bytelen));
                    }
                    panic!("ERRR:{}:DataM:Compile:{}:Unknown Special Tag {}???", smsg, stype, sdata);
                }
                if schar == '!' && echar == ')' {
                    let xop = sdata.peel_bracket('(').unwrap();
                    let sarg1;
                    let bdm2;
                    let tdata;
                    match xop.as_str() {
                        "!byteele" | "!be" | "!arrayele" | "!ae" => {
                            // ALERT: For now does not support
                            // * indexing of indexing using xcasting syntax
                            let tindex;
                            (tdata, tindex) = sdata.split_once(',').expect(&format!("ERRR:{}:DataM:Compile:XCast:{}:Extracting args:{}", smsg, xop, sdata));
                            sarg1 = tdata.as_str();
                            let idm = DataM::compile(ctxt, &tindex, stype, &format!("{}:XCast-{}:{}",smsg, xop, tindex));
                            bdm2 = Some(Box::new(idm));
                        }
                        _ => {
                            sarg1 = sdata.the_str();
                            bdm2 = None;
                        }
                    }
                    let dm = DataM::compile(ctxt, sarg1, stype, &format!("{}:XCast-{}:{}",smsg, xop, sarg1));
                    let boxdm = Box::new(dm);
                    let xdata = match xop.as_str() {
                        "!str" => XOpData::Str(boxdm),
                        "!strhex" => XOpData::StrHex(boxdm),
                        "!strtrim" => XOpData::StrTrim(boxdm),
                        "!byteele" | "!be" => XOpData::ByteEle(boxdm, bdm2.unwrap()),
                        "!arrayele" | "!ae" => XOpData::ArrayEle(boxdm, bdm2.unwrap()),
                        _ => panic!("ERRR:{}:DataM:{}:Unknown XCast type:{:?}", smsg, stype, xop),
                    };
                    return DataM::XOp(xdata);
                }
            }

        }

        if !schar.is_alphabetic() {
            panic!("ERRR:{}:DataM:{}:Variable name {} should start with a alphabetic char", smsg, stype, sdata);
        }
        let index;
        let var;
        if echar == ']' {
            var = sdata.peel_bracket('[').expect(&format!("ERRR:{}:DataM:Compile:{}:Invalid array indexing???:{}", smsg, stype, sdata));
            index = sdata.to_string();
            sdata = ctxt.tstrx.from_str(&var, true);
        } else {
            index = "".to_string();
        }

        let mut datakind = DataKind::Variable;
        if ctxt.bcompilingfunc {
            let fi = ctxt.funcs.get(&ctxt.compilingfunc).unwrap();
            if fi.1.contains(&sdata.to_string()){
                datakind = DataKind::FuncArg;
            }
        }
        let dm = DataM::Variable(datakind, sdata.to_string());
        if index == "" {
            return dm;
        }
        let idm = DataM::compile(ctxt, &index, stype, &format!("{}:Indexing:{}", smsg, index));
        return DataM::XOp(XOpData::ByteEle(Box::new(dm), Box::new(idm)));
    }

    ///
    /// Check if I am a value variant or not
    ///
    pub fn is_value(&self) -> bool {
        match self {
            DataM::Value(_) => true,
            DataM::Variable(_, _) => false,
            DataM::XOp(_) => false,
        }
    }

    ///
    /// Check if I am a variable variant or not
    ///
    pub fn is_variable(&self) -> bool {
        match self {
            DataM::Value(_) => false,
            DataM::Variable(_, _) => true,
            DataM::XOp(_) => false,
        }
    }

    pub fn identify(&self) -> String {
        match self {
            Self::Value(vval) => format!("Val:{}", vval.get_string()),
            Self::Variable(_datakind, vname) => format!("Var:{}", vname),
            Self::XOp(xdata) => xdata.identify(),
        }
    }

    ///
    /// This supports infering
    /// * Value's type in both AheadOfTimeCompilation as well as Run phase
    /// * Variable's type only during Run phase (So be careful)
    ///
    pub fn get_type(&self, ctxt: &Context) -> VDataType {
        match self {
            Self::Value(valv) => {
                return valv.get_type();
            }
            Self::Variable(datakind,vid) => {
                let ovalue = ctxt.var_get(datakind, vid);
                if ovalue.is_some() {
                    return ovalue.unwrap().get_type();
                }
            }
            Self::XOp(xdata) => {
                return xdata.get_type(ctxt);
            }
        }
        return VDataType::Unknown;
    }

    pub fn get_type_char(&self, ctxt: &Context) -> char {
        let vtype = self.get_type(ctxt);
        match vtype {
            VDataType::Unknown => '?',
            VDataType::Integer => 'i',
            VDataType::String => 's',
            VDataType::Buffer => 'b',
            VDataType::Special => 'b',
        }
    }

    ///
    /// * Int -> Int
    /// * String -> Try interpret the string as a textual literal value of a integer
    /// * Buf -> Try interpret the buf as the underlying raw byte values of a integer
    /// * XTimeStamp -> milliseconds from UnixEpoch truncated
    /// * XRandomBytes -> a randomly generated Int (limited to min(Int size,requested bytes))
    ///
    pub fn get_isize(&self, ctxt: &mut Context) -> Result<isize, String> {
        match self {
            Self::Value(oval) => {
                return oval.get_isize();
            }
            Self::Variable(datakind, vid) => {
                let ovalue = ctxt.var_get(datakind, vid);
                if ovalue.is_some() {
                    let ivalue = ovalue.unwrap().get_isize();
                    if ivalue.is_ok() {
                        return ivalue;
                    }
                    return Err(format!("DataM:GetISize:Var:{}", ivalue.unwrap_err()));
                }
                return Err(format!("DataM:GetISize:Var:Unknown:{}", vid));
            }
            Self::XOp(xdata) => {
                let ival = xdata.get_isize(ctxt);
                if ival.is_err() {
                    return Err(format!("DataM:GetISize:XCast:{:?}:{}", xdata, ival.unwrap_err()));
                }
                return Ok(ival.unwrap());
            }
        }
    }

    ///
    /// Return a positive interger value, this is built upon get_isize
    /// If the underlying value is negative, then it will panic
    ///
    pub fn get_usize(&self, ctxt: &mut Context) -> Result<usize, String> {
        let ival = self.get_isize(ctxt);
        match ival {
            Ok(ival) => {
                if ival < 0 {
                    return Err("DataM:GetUSize: Negative int value not supported here".to_string());
                }
                return Ok(ival as usize);
            }
            Err(msg) => return Err(format!("DataM:GetUSize:{}", msg)),
        }
    }

    ///
    /// * Returns Int values as equivalent string literal form
    /// * Returns String as is
    /// * Returns Buf8 data as a hex string
    /// * AnyVar follows the order of 1st check IntVars, then StrVars and then finally BufVars
    /// * XTimeStamp returns milliseconds from UnixEpoch
    /// * XRandomBytes returns random generated bytes converted to string using utf8_lossy
    ///
    pub fn get_string(&self, ctxt: &mut Context) -> Result<String, String> {
        match self {
            Self::Value(oval) => {
                return Ok(oval.get_string());
            }
            DataM::Variable(datakind, vid) => {
                let ovalue = ctxt.var_get(datakind, vid);
                if ovalue.is_some() {
                    return Ok(ovalue.unwrap().get_string());
                }
                return Err(format!("DataM:GetString:Var:Unknown:{}", vid));
            }
            Self::XOp(xdata) => {
                let sdata = xdata.get_string(ctxt);
                if sdata.is_err() {
                    return Err(format!("DataM:GetString:XCast:{:?}:{}", self, sdata.unwrap_err()));
                }
                return Ok(sdata.unwrap());
            }
        }
    }

    ///
    /// * returns int values as underlying byte values based vector in the native endianess format
    /// * Returns String as the underlying byte values based vector
    /// * Returns Buf8 data as is
    /// * AnyVar follows the order of 1st check IntVars, then StrVars and then finally BufVars
    /// * XTimeStamp -> milliseconds from UnixEpoch, as the underlying byte values of the int
    /// * XRandomBytes returns random generated bytes
    ///
    pub fn get_bufvu8(&self, ctxt: &mut Context) -> Result<Vec<u8>, String> {
        match self {
            Self::Value(oval) => {
                return Ok(oval.get_bufvu8());
            }
            DataM::Variable(datakind, vid) => {
                let ovalue = ctxt.var_get(datakind, vid);
                if ovalue.is_some() {
                    return Ok(ovalue.unwrap().get_bufvu8());
                }
                return Err(format!("DataM:GetBuf:Var:Unknown:{}", vid));
            }
            Self::XOp(xdata) => {
                let bval = xdata.get_bufvu8(ctxt);
                if bval.is_err() {
                    return Err(format!("DataM:GetBuf:XCast:{}", bval.unwrap_err()));
                }
                return Ok(bval.unwrap());
            }
        }
    }

    pub fn get_byteelement(&self, ctxt: &mut Context, index: usize) -> Result<u8, String> {
        // ALERT: This duplicates variant's byteelement logic
        // If the logic is changed in future, this has to be accounted at both places.
        let ival = self.get_bufvu8(ctxt);
        match ival {
            Ok(ival) => {
                return Ok(ival[index]);
            }
            Err(msg) => return Err(format!("DataM:GetByteEle:{}", msg)),
        }
    }

    /*
    fn get_arrayelement(&self, ctxt: &mut Context, index: usize) -> Result<Variant, String> {
        let avals = self.get_value(ctxt);
        match avals {
            Ok(vval) => return vval.get_arrayelement(index),
            Err(msg) => return Err(format!("DataM:GetArrayEle:{}", msg))
        }
    }
    */

    pub fn get_arrayelement(&self, ctxt: &mut Context, index: usize) -> Result<Variant, String> {
        match self {
            Self::Value(vval) => {
                let oval = vval.get_arrayelement(index);
                if oval.is_err() {
                    return Err(format!("DataM:GetArrayEle:Value:{:?}:{}", self, oval.unwrap_err()));
                }
                return oval;
            }
            Self::Variable(datakind, vname) => {
                let oval = ctxt.var_get(datakind, vname);
                match oval {
                    Some(vval) => {
                        let oval = vval.get_arrayelement(index);
                        if oval.is_err() {
                            return Err(format!("DataM:GetArrayEle:Var:{:?}:{}", self, oval.unwrap_err()));
                        }
                        return oval;
                    }
                    None => {
                        return Err(format!("DataM:GetArrayEle:Var:{:?}:Unknown var", self));
                    }
                }
            }
            Self::XOp(xdata) => {
                let vval = xdata.get_arrayelement(ctxt, index);
                if vval.is_err() {
                    return Err(format!("DataM:GetArrayEle:XCast:{:?}:{}", self, vval.unwrap_err()));
                }
                return vval;
            }
        }
    }

    #[allow(dead_code)]
    fn get_value(&self, ctxt: &mut Context) -> Result<Variant, String> {
        match self {
            Self::Value(oval) => return Ok(oval.clone()),
            Self::Variable(datakind, vname) => {
                let oval = ctxt.var_get(datakind, vname);
                if oval.is_some() {
                    return Ok(oval.unwrap().clone());
                }
                return Err(format!("DataM:GetVal:Var:Unknown:{}", vname));
            }
            Self::XOp(xdata) => {
                let vval = xdata.get_value(ctxt);
                if vval.is_err() {
                    return Err(format!("DataM:GetVal:XCast:{}", vval.unwrap_err()));
                }
                return Ok(vval.unwrap());
            }
        }
    }

    pub fn get_type_value(&self, ctxt: &mut Context) -> Result<(VDataType, Variant), String> {
        match self {
            Self::Value(oval) => return Ok((oval.get_type(), oval.clone())),
            Self::Variable(datakind, vname) => {
                let oval = ctxt.var_get(datakind, vname);
                if oval.is_some() {
                    let oval = oval.unwrap();
                    return Ok((oval.get_type(), oval.clone()));
                }
                return Err(format!("DataM:GetTypeVal:Var:Unknown:{}", vname));
            }
            Self::XOp(xdata) => {
                let oval = xdata.get_value(ctxt);
                if oval.is_err() {
                    return Err(format!("DataM:GetTypeVal:XCast:{}", oval.unwrap_err()));
                }
                let oval = oval.unwrap();
                return Ok((oval.get_type(), oval));
            }
        }
    }

    #[allow(dead_code)]
    fn get_bufvu8_mut<'a>(&'a self, ctxt: &'a mut Context, smsg: &str) -> &'a mut Vec<u8> {
        match self {
            Self::Value(_) => panic!("ERRR:{}:GetBufVu8Mut:Cant return mutable ref to values", smsg),
            Self::Variable(datakind, vid) => {
                if let DataKind::FuncArg = datakind {
                    panic!("ERRR:{}:GetBufVu8Mut:FuncArg cant be used mutably/to-write-to its existing buf, currently", smsg);
                }
                if ctxt.var_islocal(vid) {
                    panic!("ERRR:{}:GetBufVu8Mut:Localvar cant be used mutably/to-write-to its existing buf, currently", smsg);
                }
                let vvalue = ctxt.globals.get_mut(vid).unwrap();
                let gotr = vvalue.get_bufvu8_mut();
                if gotr.is_none() {
                    panic!("ERRR:{}:GetBufVu8Mut:Some issue with getting mut buf wrt:{}", smsg, vid);
                }
                return gotr.unwrap();
            }
            Self::XOp(_xdata) => {
                panic!("ERRR:{}:DataM:GetBufVu8Mut:XCast:{:?}:Not supported", smsg, self);
            }
        }
    }

    pub fn set_isize(&self, ctxt: &mut Context, vvalue: isize) -> Result<(), String> {
        match  self {
            DataM::Value(_) => return Err("DataM:SetISize:Val:Cant set a value!".to_string()),
            DataM::Variable(datakind, vname) => {
                let vvalue = Variant::IntValue(vvalue);
                let ok = ctxt.var_set(datakind, vname, vvalue, false);
                if ok.is_ok() {
                    return ok;
                }
                return Err(format!("DataM:SetISize:{}", ok.unwrap_err()));
            }
            Self::XOp(_xdata) => {
                return Err(format!("DataM:SetISize:XCast:{:?}:Not supported", self));
            }
        }
    }

    #[allow(dead_code)]
    fn set_string(&self, ctxt: &mut Context, vvalue: String) -> Result<(), String> {
        match  self {
            DataM::Value(_) => return Err("DataM:SetString:Val:Cant set a value!".to_string()),
            DataM::Variable(datakind, vname) => {
                let vvalue = Variant::StrValue(vvalue);
                let ok = ctxt.var_set(datakind, vname, vvalue, false);
                if ok.is_ok() {
                    return ok;
                }
                return Err(format!("DataM:SetString:{}", ok.unwrap_err()));
            }
            Self::XOp(_xdata) => {
                return Err(format!("DataM:SetString:XCast:{:?}:Not supported", self));
            }
        }
    }

    pub fn set_bufvu8(&self, ctxt: &mut Context, vvalue: Vec<u8>) -> Result<(), String> {
        match  self {
            DataM::Value(_) => return Err("DataM:SetBuf:Val:Cant set a value!".to_string()),
            DataM::Variable(datakind, vname) => {
                let vvalue = Variant::BufValue(vvalue);
                let ok = ctxt.var_set(datakind, vname, vvalue, false);
                if ok.is_ok() {
                    return ok;
                }
                return Err(format!("DataM:SetBuf:{}", ok.unwrap_err()));
            }
            Self::XOp(_xdata) => {
                return Err(format!("DataM:SetBuf:XCast:{:?}:Not supported", self));
            }
        }
    }

    pub fn set_value(&self, ctxt: &mut Context, vvalue: Variant, bforcelocal: bool) -> Result<(), String> {
        match  self {
            DataM::Value(_) => return Err("DataM:SetValue:Val:Cant set a value! to a value".to_string()),
            DataM::Variable(datakind, vname) => {
                let ok = ctxt.var_set(datakind, vname, vvalue, bforcelocal);
                if ok.is_ok() {
                    return  ok;
                }
                return Err(format!("DataM:SetValue:{}", ok.unwrap_err()));
            }
            Self::XOp(_xdata) => {
                return Err(format!("DataM:SetValue:XCast:{:?}:Not supported", self));
            }
        }
    }

}

