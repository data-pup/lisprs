use std::convert::TryFrom;

use lisp_token::LispToken;
use lisp_token::LispToken::{
    OpenExpression,
    Operator,
    Variable,
    Value,
    CloseExpression,
};

pub enum ValueParseError {
    StringParseError,
    UnexpectedOperator,
    UnexpectedParentheses,
    UnexpectedSymbol,
}

pub enum LispValue {
    Int(i32),
    Float(f32),
}

impl TryFrom<LispToken> for LispValue {
    type Error = ValueParseError;
    fn try_from(token: LispToken) -> Result<LispValue, ValueParseError> {
        match token {
            Value(val) => LispValue::parse_value_str(&val),
            OpenExpression | CloseExpression =>
                Err(ValueParseError::UnexpectedParentheses),
            Operator(_) => Err(ValueParseError::UnexpectedOperator),
            Variable(_) => Err(ValueParseError::UnexpectedSymbol),
        }
    }
}

impl LispValue {
    fn parse_value_str(val: &str) -> Result<LispValue, ValueParseError> {
        let i_res = LispValue::parse_int(&val);
        if i_res.is_ok() {
            i_res
        } else {
            LispValue::parse_float(&val)
        }
    }

    fn parse_int(val: &str) -> Result<LispValue, ValueParseError> {
        match val.parse::<i32>() {
            Ok(i) => Ok(LispValue::Int(i)),
            Err(_) => Err(ValueParseError::StringParseError)
        }
    }

    fn parse_float(val: &str) -> Result<LispValue, ValueParseError> {
        match val.parse::<f32>() {
            Ok(f) => Ok(LispValue::Float(f)),
            Err(_) => Err(ValueParseError::StringParseError)
        }
    }
}
