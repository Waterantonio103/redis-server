#![allow(unused)]

pub mod command_types;

use crate::{commands::command_types::{COMMANDS, Command, ECHO, Echo, PING, Ping}, decode::{RespValue::{self, Array, BlkStr, Error, Int, SmplStr}, resp_types::{BulkString, RespArray}}};

//get a respvalue -> match it to decompose -> match first inputs -> redirect to logic
//(ex: array(blkstr("echo"), blkstr("hello")) -> cmp/match first blkstr to known command -> redirect to logic of said command )

//handle commands constructed by parser
pub fn handle_command(cmd: &RespValue) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        Array(a) => { handle_array(a); },
        BlkStr(b) => { handle_blk_str(b); },
        SmplStr(s) => {},
        Error(e) => {},
        Int(i) => {},
    }
    Ok(())
}

fn handle_array(array: &RespArray) {
    for value in array {
        handle_command(value);
    }
    todo!()
}

fn handle_blk_str(input: &BulkString) -> Command {
    //job: look at blkstr and compare it to existing commands
    let mut command = Command::None;
    for (index, &command_type) in COMMANDS.iter().enumerate() {
        if input.data().eq_ignore_ascii_case(command_type) {
            match COMMANDS[index] {
                PING => { command = Command::Ping; },
                ECHO => { command = Command::Echo; },
                _ => {},
            }
        }
    }
    command
}

#[cfg(test)]
mod tests {
    use crate::decode::resp_types::BulkString;

use super::*;

    #[test]
    fn test_reffed_respvalue() {
        let cmd = RespValue::Array(RespArray::new(&[RespValue::BlkStr(BulkString::new(&[0], 1))], 1));
        let _out = handle_command(&cmd).unwrap();
    }

    #[test]
    fn test_blk_str() {
        let input = BulkString::new("PING".as_bytes(), 4);
        let output = handle_blk_str(&input);
        println!("{output:?}");
        assert!(output==Command::Ping);
    }
}