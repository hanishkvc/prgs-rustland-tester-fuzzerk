
use crate::datautils;

use super::{DataM, Context, datas::{VDataType, Variant}};



#[derive(Debug, Clone)]
pub(crate) enum XOpData {
    Str(Box<DataM>),
    StrHex(Box<DataM>),
    StrTrim(Box<DataM>),
    ByteEle(Box<DataM>, Box<DataM>),
    ArrayEle(Box<DataM>, Box<DataM>),
}


impl XOpData {

    pub fn identify(&self) -> String {
        match self {
            Self::Str(dm) => format!("!Str({})", dm.identify()),
            Self::StrTrim(dm) => format!("!StrTrim({})", dm.identify()),
            Self::StrHex(dm) => format!("!StrHex({})", dm.identify()),
            Self::ByteEle(dm, index) => format!("!ByteEle({}, {})", dm.identify(), index.identify()),
            Self::ArrayEle(ddm, idm) => format!("!ArrayEle({}, {})", ddm.identify(), idm.identify()),
        }
    }

    pub fn get_string(&self, ctxt: &mut Context) -> Result<String, String> {
        match self {
            Self::Str(dm) => {
                let tv = dm.get_type_value(ctxt);
                if tv.is_err() {
                    return Err(format!("XOpData:Str:GetString:{:?}:{}", self, tv.unwrap_err()));
                }
                let (vtype, vvalue) = tv.unwrap();
                match vtype {
                    VDataType::Buffer => return Ok(String::from_utf8_lossy(&vvalue.get_bufvu8()).to_string()),
                    _ => return Ok(vvalue.get_string()),
                }
            }
            Self::StrTrim(dm) => {
                let tv = dm.get_type_value(ctxt);
                if tv.is_err() {
                    return Err(format!("XOpData:StrTrim:GetString:{:?}:{}", self, tv.unwrap_err()));
                }
                let (vtype, vvalue) = tv.unwrap();
                let sdata = match vtype {
                    VDataType::Buffer => String::from_utf8_lossy(&vvalue.get_bufvu8()).to_string(),
                    _ => vvalue.get_string(),
                };
                return Ok(sdata.trim().to_string());
            }
            Self::StrHex(dm) => {
                let bdata = dm.get_bufvu8(ctxt);
                if bdata.is_err() {
                    return Err(format!("XOpData:StrHex:GetString:{:?}:{}", self, bdata.unwrap_err()));
                }
                return Ok(datautils::hex_from_vu8(&bdata.unwrap()));
            }
            Self::ByteEle(dm, index) => {
                let i = index.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:ByteEle:GetString:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let bval = dm.get_byteelement(ctxt, i.unwrap());
                if bval.is_err() {
                    return Err(format!("XOpData:ByteEle:GetString:{:?}:IndexedData:{}", self, bval.unwrap_err()));
                }
                let cval = char::from_u32( bval.unwrap() as u32).unwrap();
                return Ok(cval.to_string());
            }
            Self::ArrayEle(ddm, idm) => {
                let i = idm.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:ArrayEle:GetString:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let vval = ddm.get_arrayelement(ctxt, i.unwrap());
                if vval.is_err() {
                    return Err(format!("XOpData:ArrayEle:GetString:{:?}:IndexedData:{}", self, vval.unwrap_err()));
                }
                return Ok(vval.unwrap().get_string());
            }
        }
    }

    pub fn get_isize(&self, ctxt: &mut Context) -> Result<isize, String> {
        match self {
            Self::ByteEle(dm, index) => {
                let i = index.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetISize:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let bval = dm.get_byteelement(ctxt, i.unwrap());
                if bval.is_err() {
                    return Err(format!("XOpData:GetISize:{:?}:IndexedData:{}", self, bval.unwrap_err()));
                }
                return Ok(bval.unwrap() as isize);
            }
            Self::ArrayEle(ddm, idm) => {
                let i = idm.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetISize:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let vval = ddm.get_arrayelement(ctxt, i.unwrap());
                if vval.is_err() {
                    return Err(format!("XOpData:GetISize:{:?}:IndexedData:{}", self, vval.unwrap_err()));
                }
                let ival = vval.unwrap().get_isize();
                if ival.is_err() {
                    return Err(format!("XOpData:GetISize:{:?}:Value:{}", self, ival.unwrap_err()));
                }
                return Ok(ival.unwrap());
            }
            _ => { // All other XOps are str generating, so do the xop as part of get_string
                let sdata = self.get_string(ctxt);
                if sdata.is_err() {
                    return Err(format!("XOpData:GetISize:Casting:{:?}:{}", self, sdata.unwrap_err()));
                }
                let sdata = sdata.unwrap();
                let ival = Variant::StrValue(sdata).get_isize();
                if ival.is_err() {
                    return Err(format!("XOpData:GetISize:Converting:{:?}:{}", self, ival.unwrap_err()));
                }
                return Ok(ival.unwrap());
            }
        }
    }

    pub fn get_bufvu8(&self, ctxt: &mut Context) -> Result<Vec<u8>, String> {
        match self {
            Self::ByteEle(dm, index) => {
                let i = index.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetBuf:{:?}:Index:{}", self, i.unwrap_err()));
                }
                let bval = dm.get_byteelement(ctxt, i.unwrap());
                if bval.is_err() {
                    return Err(format!("XOpData:GetBuf:{:?}:{}", self, bval.unwrap_err()));
                }
                let mut bvec = Vec::new();
                bvec.push(bval.unwrap());
                return Ok(bvec);
            }
            Self::ArrayEle(ddm, idm) => {
                let i = idm.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetBuf:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let vval = ddm.get_arrayelement(ctxt, i.unwrap());
                if vval.is_err() {
                    return Err(format!("XOpData:GetBuf:{:?}:IndexedData:{}", self, vval.unwrap_err()));
                }
                return Ok(vval.unwrap().get_bufvu8());
            }
            _ => {
                // All other XOps are str generating, so do the xop as part of get_string
                let sdata = self.get_string(ctxt);
                if sdata.is_err() {
                    return Err(format!("XOpData:GetBuf:Casting:{:?}:{}", self, sdata.unwrap_err()));
                }
                return Ok(Variant::StrValue(sdata.unwrap()).get_bufvu8());
            }
        }
    }

    pub fn get_value(&self, ctxt: &mut Context) -> Result<Variant, String> {
        match self {
            Self::ByteEle(dm, index) => {
                let i = index.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetValue:{:?}:Index:{}", self, i.unwrap_err()));
                }
                let bval = dm.get_byteelement(ctxt, i.unwrap());
                if bval.is_err() {
                    return Err(format!("XOpData:GetValue:{:?}:{}", self, bval.unwrap_err()));
                }
                return Ok(Variant::IntValue(bval.unwrap() as isize));
            }
            Self::ArrayEle(ddm, idm) => {
                let i = idm.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetValue:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let vval = ddm.get_arrayelement(ctxt, i.unwrap());
                if vval.is_err() {
                    return Err(format!("XOpData:GetValue:{:?}:IndexedData:{}", self, vval.unwrap_err()));
                }
                return vval;
            }
            _ => {
                // All other XOps are str generating, so do the xop as part of get_string
                let sdata = self.get_string(ctxt);
                if sdata.is_err() {
                    return Err(format!("XOpData:GetValue:Casting:{:?}:{}", self, sdata.unwrap_err()));
                }
                return Ok(Variant::StrValue(sdata.unwrap()));
            }
        }
    }

    pub fn get_arrayelement(&self, ctxt: &mut Context, index: usize) -> Result<Variant, String> {
        match self {
            Self::ByteEle(_ddm, _idm) => {
                return Err(format!("XOpData:GetArrayEle:{:?}:Not allowed on a ByteEle", self));
            }
            Self::ArrayEle(ddm, idm) => {
                let i = idm.get_usize(ctxt);
                if i.is_err() {
                    return Err(format!("XOpData:GetArrayEle:{:?}:GetIndex:{}", self, i.unwrap_err()));
                }
                let vval = ddm.get_arrayelement(ctxt, i.unwrap());
                if vval.is_err() {
                    return Err(format!("XOpData:GetArrayEle:{:?}:IndexedData:{}", self, vval.unwrap_err()));
                }
                let rval = vval.unwrap().get_arrayelement(index);
                if rval.is_err() {
                    return Err(format!("XOpData:GetArrayEle:{:?}:Value:{}", self, rval.unwrap_err()));
                }
                return rval;
            }
            _ => {
                // All other XOps are str generating, so do the xop as part of get_string
                let sdata = self.get_string(ctxt);
                if sdata.is_err() {
                    return Err(format!("XOpData:GetArrayEle:Casting:{:?}:{}", self, sdata.unwrap_err()));
                }
                let rval = Variant::StrValue(sdata.unwrap()).get_arrayelement(index);
                if rval.is_err() {
                    return Err(format!("XOpData:GetArrayEle:Indexing:{:?}:{}", self, rval.unwrap_err()));
                }
                return rval;
            }
        }
    }

    pub fn get_type(&self, ctxt: &Context) -> VDataType {
        match self {
            Self::ByteEle(_,_) => return VDataType::Integer,
            Self::ArrayEle(ddm, _idm) => return ddm.get_type(ctxt),
            _ => return VDataType::String, // All other XOps are str generating, so this
        }
    }

}
