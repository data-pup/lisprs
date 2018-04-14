use std::convert::TryFrom;

use lisp_token::LispToken;

pub enum ValueParseError {
    TempError,
}

pub enum _LispValue {
    UInt(u32),
    Int(i32),
    Float(f32),
}

impl TryFrom<LispToken> for _LispValue {
    type Error = ValueParseError;
    fn try_from(token: LispToken) -> Result<_LispValue, ValueParseError> {
        unimplemented!();
    }
}
