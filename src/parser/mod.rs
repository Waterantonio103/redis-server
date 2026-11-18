#![allow(unused)]

pub mod resp_types;

use std::ops::{Mul, Neg};

use winnow::{ModalResult, Parser, ascii::digit1, combinator::{opt, peek}, error::{ContextError, ErrMode, StrContext, StrContextValue}, stream::ToUsize, token::{self, any, one_of, take, take_until}};
use RespValue::*;

use crate::parser::Sign::{Minus, Plus};
use crate::parser::resp_types::{BlkStr, RespArray, SmplStr, RespError};

pub const CRLF: &str = "\r\n";
pub const CR: u8 = 13;
pub const LF: u8 = 10;
pub const ASTERISK: u8 = 42;
pub const DOLLAR: u8 = 36;
pub const PLUS: u8 = 43;
pub const HYPHEN: u8 = 45;
pub const COLON: u8 = 58;
pub const SPACE: u8 = 32;

#[derive(Debug, Clone)]
pub enum RespValue {
    Array(RespArray),
    BlkStr(BlkStr),
    SmplStr(SmplStr),
    Error (RespError),
    Int(i64),
}

#[derive(Debug)]
enum Sign {
    Plus,
    Minus
}

impl Sign {
    fn assign<T: Neg<Output = T>>(&self, val: T) -> T {
        match self {
            Plus => val,
            Minus => val.neg(),
        }
    }
}

pub fn parse(input: &mut &[u8]) -> ModalResult<RespValue> {
    //global parser
    parse_dispatcher(input)?;
    todo!()
}

fn parse_dispatcher(input: &mut &[u8]) -> ModalResult<RespValue> {
    //send off to correct parser depending on prefix
    match peek(any).parse_next(input)? {
        ASTERISK => parse_array(input),
        DOLLAR => parse_blk_str(input),
        PLUS => parse_smpl_str(input),
        HYPHEN => parse_resp_err(input),
        COLON => parse_resp_int(input),
        _ => { return Err(ErrMode::Cut(ContextError::new())); }
    }
}

fn parse_array(input: &mut &[u8]) -> ModalResult<RespValue> {
    //use size to create a vector of args
    let size = parse_array_header(input)?;

    let mut items = Vec::new();

    for _ in 0..size {
        items.push(parse_dispatcher(input)?);
    }

    Ok(Array(RespArray::new(&items, items.len())))
}

fn parse_blk_str(input: &mut &[u8]) -> ModalResult<RespValue> {
    //parse sequence : prefix_size_crlf_item_crlf
    parse_blk_prefix(input)?;

    let size = parse_size(input)?;

    parse_crlf(input)?;

    let item = parse_item_payload(input, size)?;

    parse_crlf(input)?;

    Ok(BlkStr(BlkStr::new(item, item.len())))
}

fn parse_smpl_str(input: &mut &[u8]) -> ModalResult<RespValue> {
    //sequence: prefix_str_crlf
    parse_smpl_prefix(input)?;

    let item = parse_item_condition(input, CRLF)?;

    parse_crlf(input)?;

    if let Ok(output) = str::from_utf8(item).map(String::from) {
        Ok(SmplStr(SmplStr::new(output)))
    } else {
        Err(ErrMode::Cut(ContextError::new()))
    }
}

fn parse_resp_err(input: &mut &[u8]) -> ModalResult<RespValue> {
    //sequence: prefix_code_space_msg_crlf
    parse_err_prefix(input)?;

    let code = parse_item_condition(input, " ")?;

    parse_space(input)?;

    let msg = parse_item_condition(input, CRLF)?;

    parse_crlf(input)?;

    Ok(Error(RespError::new(code, msg)))
}

fn parse_resp_int(input: &mut &[u8]) -> ModalResult<RespValue> {
    //sequence: prefix_optional[+/-]_item_crlfary
    parse_int_prefix(input)?;

    let optional_sign = parse_sign(input)?;

    //clamping to avoid error, to review since this is potentially very poor design choice
    let int = parse_size(&mut parse_item_condition(input, CRLF)?)?.clamp(0, i64::MAX.cast_unsigned());

    parse_crlf(input)?;

    if let Some(sign) = optional_sign {
        Ok(Int(sign.assign(int.cast_signed())))
    } else {
        Ok(Int(int as i64))
    }
}

fn parse_array_prefix(input: &mut &[u8]) -> ModalResult<()> {
    ASTERISK
        .context(StrContext::Expected(
            StrContextValue::Description("Array prefix '*'")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_smpl_prefix(input: &mut &[u8]) -> ModalResult<()> {
    PLUS
        .context(StrContext::Expected(
            StrContextValue::Description("Simple string prefix '+'")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_int_prefix(input: &mut &[u8]) -> ModalResult<()> {
    COLON
        .context(StrContext::Expected(
            StrContextValue::Description("Integer prefix ':'")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_blk_prefix(input: &mut &[u8]) -> ModalResult<()> {
    DOLLAR
        .context(StrContext::Expected(
            StrContextValue::Description("Bulk string prefix '$'")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_err_prefix(input: &mut &[u8]) -> ModalResult<()> {
    HYPHEN
        .context(StrContext::Expected(
            StrContextValue::Description("Error prefix '-'")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_space(input: &mut &[u8]) -> ModalResult<()> {
    SPACE
        .context(StrContext::Expected(
            StrContextValue::Description("Empty space")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_size(input: &mut &[u8]) -> ModalResult<u64> {
    digit1
        .context(StrContext::Expected(
            StrContextValue::Description("Integer")
        ))
        .try_map(|bytes: &[u8]| {
            str::from_utf8(bytes)
                .unwrap()
                .parse::<u64>()
        })
        .parse_next(input)
}

fn parse_crlf(input: &mut &[u8]) -> ModalResult<()> {
    #[allow(const_item_mutation)]
    CRLF
        .context(StrContext::Expected(
            StrContextValue::Description("CRLF Terminator")
        ))
        .parse_next(input)?;
    Ok(())
}

fn parse_sign(input: &mut &[u8]) -> ModalResult<Option<Sign>> {
    match opt(one_of(['+', '-'])).parse_next(input)? {
        Some(sign) => {
            match sign {
                PLUS => Ok(Some(Plus)),
                HYPHEN => Ok(Some(Minus)),
                _ => Ok(None)
            }
        } ,
        None => { Ok(None) }
    }
}

fn parse_array_header(input: &mut &[u8]) -> ModalResult<u64> {
    //parse sequence: prefix_size_crlf
    parse_array_prefix(input)?;

    let size = parse_size(input)?;

    parse_crlf(input)?;

    Ok(size)
}

fn parse_item_payload<'a, T: ToUsize>(input: &mut &'a [u8], size: T) -> ModalResult<&'a [u8]> {
    let item = take(size).parse_next(input)?;
    Ok(item)
}

fn parse_item_condition<'a>(input: &mut &'a [u8], condition: &'a str) -> ModalResult<&'a [u8]> {
    take_until(0.., condition).parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_header() {
        let mut input = "*4\r\nECHO\r\n".as_bytes();
        let size = parse_array_header(&mut input).unwrap();
        assert_eq!(size, 4);
        assert_eq!(input, "ECHO\r\n".as_bytes());
    }

    #[test]
    fn test_item() {
        let mut input = "ECHO\r\n".as_bytes();
        let output = parse_item_payload(&mut input, 4_u64).unwrap();
        assert_eq!(input, CRLF.as_bytes());
        assert_eq!(output, "ECHO".as_bytes());
    }

    #[test]
    fn test_array() -> ModalResult<()> {
        let mut input = "*2\r\n$5\r\nECHO\0\r\n*2\r\n$5\r\nHELLO\r\n+--BYE--\r\n".as_bytes();
        let output = parse_array(&mut input)?;
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_blk_str() -> ModalResult<()> {
        let mut input = "$6\r\n\0ECHO\0\r\n".as_bytes();
        let output = parse_blk_str(&mut input)?;
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_conditional_item() -> ModalResult<()> {
        let mut input = "HELLOABC12349--+=\r\n".as_bytes();
        let output = parse_item_condition(&mut input, CRLF)?;
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_smpl_str() -> ModalResult<()> {
        let mut input = "+HELP123-_+=\r\n".as_bytes();
        let output = parse_smpl_str(&mut input)?;
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_sign() -> ModalResult<()> {
        let mut input = "+4000".as_bytes();
        let output = parse_sign(&mut input)?.unwrap();
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_assign() {
        let num: i64 = 456;
        let neg_num = Minus.assign(num);
        let pos_num = Plus.assign(num);
        println!("{neg_num}");
        println!("{pos_num}");
        let neg_num: i64 = -456;
        let reverse_num: i64 = Minus.assign(neg_num);
        let same_num: i64 = Plus.assign(neg_num);
        println!("{reverse_num}");
        println!("{same_num}");
    }

    #[test]
    fn test_parse_int() -> ModalResult<()> {
        let mut input = ":-0030456\r\n".as_bytes();
        let output = parse_resp_int(&mut input)?;
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_parse_err() -> ModalResult<()> {
        let mut input = "-WRONGTYPE wrong associated type\r\n".as_bytes();
        let output = parse_resp_err(&mut input)?;
        println!("{output:?}");
        Ok(())
    }

    #[test]
    fn test_parse_large_input() -> ModalResult<()> {
        let mut input = "*3\r\n+i\r\n$5\r\ngreat\r\n*3\r\n-ERR invalid person\r\n+james\r\n:-1243190\r\n".as_bytes();
        let output = parse_array(&mut input)?;
        println!("{output:?}");
        Ok(())
    }
}

