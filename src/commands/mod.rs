#![allow(unused)]
use crate::{commands::{cmd_packager::{CommandArray, CommandWrapper, pkg_resp_val}, command_types::{CommandType, Echo}}, decode::{RespValue, resp_types::{BulkString, RespArray}}};

pub mod cmd_packager;
pub mod command_types;

//take original respvalue and packaged command and build based on that

#[derive(Debug)]
pub enum AssignError {
    UnknownCommand,
    InvalidEncoding,
    // ArrayIndex,
    InvalidStructure,
}

//un-serialized response type
#[derive(Debug)]
pub struct Response {
    data: RespValue
}

impl Response {
    #[deprecated(note="check logic and error cases")]
    pub fn new(data: RespValue) -> Self {
        Response { data }
    }

    //temporary fn
    #[deprecated(note="temporary placeholder function -- to be replaced")]
    pub fn empty() -> Self {
        Response { data: RespValue::Int(0) }
    }

    pub fn from_blk_str(input: BulkString) -> Self {
        Response { data: RespValue::BlkStr(input) }
    }
}

pub fn build_response(input: &RespValue) -> Result<Response, AssignError> {
    if let RespValue::Array(array) = input {
        dbg!(pkg_resp_val(input));
        match pkg_resp_val(input) {
            CommandWrapper::Array(arr) => {
                return Ok(handle_array(arr, array)?);
            },
            CommandWrapper::Cmd(cmd) => {
                return Ok(handle_cmd(cmd, array)?);
            },
            CommandWrapper::Ignore => { return Ok(Response::empty()); },
        }
    } else {
        return Err(AssignError::InvalidStructure)
    }
}

fn handle_array(cmd_array: CommandArray, resp_array: &RespArray) -> Result<Response, AssignError> {
    for cmd_wrap in cmd_array.into_iter() {
        match cmd_wrap {
            CommandWrapper::Array(inner_arr) => { return Ok(handle_array(inner_arr, resp_array)?); },
            CommandWrapper::Cmd(cmd) => {
                return Ok(handle_cmd(cmd, resp_array)?);
            },
            CommandWrapper::Ignore => { return Ok(Response::empty()); },
        }
    }
    Ok(Response::empty())
}

fn handle_cmd(cmd: CommandType, resp_array: &RespArray) -> Result<Response, AssignError> {
    match cmd {
        CommandType::Ping => { return Ok(Response::empty()); },
        CommandType::Echo => {
            //build response type from echo
            return Ok(Response::from_blk_str(Echo::from_array(resp_array)?.data_clone()));
        },
        CommandType::None => { return Ok(Response::empty()) },
    }
}

#[cfg(test)]
mod tests {
    use crate::decode::resp_types::{BulkString, RespArray};
    use super::*;

    #[test]
    fn test_pkg() {
        let input = RespValue::Array(RespArray::new(
            &[
                RespValue::BlkStr(BulkString::new("ECHO".as_bytes())),
                RespValue::BlkStr(BulkString::new("test".as_bytes())),
            ])
        );
        let pkg = pkg_resp_val(&input);
        println!("{input:?}");
        println!("{pkg:?}");

        let should_be_empty = RespValue::Int(0);
        //this scenario handles an empty iterator by still iterating
        //probable scenario, if its None, return an error
        should_be_empty.iter().into_iter().flatten().map(|val| {
            println!("{val:?}");
        }).for_each(drop);
    }
}