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

pub fn build_response(input: &RespValue) -> Result<Vec<Response>, AssignError> {
    if let RespValue::Array(array) = input {
        dbg!(pkg_resp_val(input));
        let mut pkg = pkg_resp_val(input);
        correct_array_structure(&mut pkg);
        Ok(handle_cmdwrap(pkg, array)?)
    } else {
        Err(AssignError::InvalidStructure)
    }
}

fn handle_cmdwrap(cmd_wrap: CommandWrapper, resp_array: &RespArray) -> Result<Vec<Response>, AssignError> {
    let mut responses = Vec::new();
    match cmd_wrap {
        CommandWrapper::Array(inner_arr) => { responses.extend(handle_array(inner_arr, resp_array)?); },
        CommandWrapper::Cmd(cmd) => {
            responses.push(handle_cmd(cmd, resp_array)?);
        },
        CommandWrapper::Ignore => { return Err(AssignError::InvalidEncoding); },
    }
    Ok(responses)
}

//handling array input by iterating and extending
fn handle_array(cmd_array: CommandArray, resp_array: &RespArray) -> Result<Vec<Response>, AssignError> {
    let mut responses: Vec<Response> = Vec::new();
    for cmd_wrap in cmd_array.into_iter() {
        dbg!(&cmd_wrap);
        match cmd_wrap {
            CommandWrapper::Array(inner_arr) => { responses.extend(handle_array(inner_arr, resp_array)?); },
            CommandWrapper::Cmd(cmd) => {
                responses.push(handle_cmd(cmd, resp_array)?);
            },
            CommandWrapper::Ignore => { return Err(AssignError::InvalidEncoding); },
        }
    }
    Ok(responses)
}

//logic for building responses based on command
fn handle_cmd(cmd: CommandType, resp_array: &RespArray) -> Result<Response, AssignError> {
    match cmd {
        CommandType::Ping => { return Ok(Response::from_blk_str(BulkString::new("PONG".as_bytes()))); },
        CommandType::Echo => {
            //build response type from echo
            return Ok(Response::from_blk_str(Echo::from_array(resp_array)?.data_clone()));
        },
        CommandType::None => { return Err(AssignError::UnknownCommand) },
    }
}

//purpose is to correct structure based on if another thing is marked as a command when initial command does not expect it
//example : echo ping -> gets rid of CommandType::Ping whilst keeping the ping bytes to send as a response
fn correct_array_structure(input: &mut CommandWrapper) {
    if let CommandWrapper::Array(array) = input {
        let first_element = array.first_mut();
        match first_element {
            CommandWrapper::Array(_) => { correct_array_structure(first_element); },
            CommandWrapper::Cmd(cmd) => {
                match cmd {
                    CommandType::Echo => { 
                        for index in 0..array.len() {
                            if index == 0 { continue; }
                            array.remove(array.len() - index);
                        } 
                },
                    _ => {},
                }
            }
            CommandWrapper::Ignore => {},
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{commands::command_types::{ECHO, PING}, decode::resp_types::{BulkString, RespArray}};
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

    #[test]
    fn test_array_handler() -> Result<(), AssignError> {
        let cmd_array = CommandArray::new(
            vec![
                CommandWrapper::Array(CommandArray::new(
                    vec![CommandWrapper::Cmd(CommandType::Echo)]
                )), 
                CommandWrapper::Cmd(CommandType::Ping)
                ]
        );
        let resp_array = RespArray::new(
            &[
                RespValue::Array(RespArray::new(&[RespValue::BlkStr(BulkString::new(ECHO))])), 
                RespValue::BlkStr(BulkString::new(PING))
            ]
        );

        let result = handle_array(cmd_array, &resp_array)?;
        println!("{result:?}");
        Ok(())
    }
}