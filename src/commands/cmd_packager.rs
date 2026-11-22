#![allow(unused)]

use crate::{commands::command_types::{COMMANDS, CommandType, ECHO, Echo, PING, Ping}, decode::{RespValue::{self, Array, BlkStr, Error, Int, SmplStr}, resp_types::{BulkString, RespArray, RespError, SimpleString}}};

//get a respvalue -> match it to decompose -> match first inputs -> redirect to logic
//(ex: array(blkstr("echo"), blkstr("hello")) -> cmp/match first blkstr to known command -> redirect to logic of said command )

#[derive(Debug, PartialEq)]
pub enum CommandWrapper {
    Array(CommandArray),
    Cmd(CommandType),
    Ignore,
}

impl CommandWrapper {
    #[deprecated(note="check logic and error cases")]
    //temporary helper
    fn new() -> Self {
        Self::Cmd(CommandType::None)
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandArray {
    commands: Vec<CommandWrapper>
}

impl CommandArray {
    pub fn new(commands: Vec<CommandWrapper>) -> Self {
        Self { commands }
    }

    pub fn iter(&self) -> impl Iterator<Item = &CommandWrapper> {
        self.commands.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut CommandWrapper> {
        self.commands.iter_mut()
    }

    pub fn into_iter(self) -> impl Iterator<Item = CommandWrapper> {
        self.commands.into_iter()
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }

    pub fn first_mut(&mut self) -> &mut CommandWrapper {
        self.commands.get_mut(0).unwrap()
    }

    pub fn remove(&mut self, index: usize) {
        self.commands.remove(index);
    }
}

//handle commands constructed by parser
pub fn pkg_resp_val(input: &RespValue) -> CommandWrapper {
    match input {
        Array(a) => pkg_array(a),
        BlkStr(b) => pkg_blk_str(b),
        _ => CommandWrapper::Ignore,
    }
}

fn pkg_array(array: &RespArray) -> CommandWrapper {
    //unwrapping for now be sure to handle later
    let mut commands = Vec::new();
    for value in array {
        commands.push(pkg_resp_val(value));
    }
    CommandWrapper::Array(CommandArray::new(commands))
}

fn pkg_blk_str(input: &BulkString) -> CommandWrapper {
    //job: look at blkstr and compare it to existing commands
    let mut command = CommandType::None;
    for (index, &command_type) in COMMANDS.iter().enumerate() {
        if input.data().eq_ignore_ascii_case(command_type) {
            match COMMANDS[index] {
                //be sure to update this when new commands come
                PING => { command = CommandType::Ping; },
                ECHO => { command = CommandType::Echo; },
                _ => {},
            }
        }
    }
    CommandWrapper::Cmd(command)
}

#[cfg(test)]
mod tests {
    use crate::decode::resp_types::BulkString;

use super::*;

    #[test]
    fn test_reffed_respvalue() {
        let cmd = RespValue::Array(RespArray::new(&[RespValue::BlkStr(BulkString::new(&[0]))]));
        let out = pkg_resp_val(&cmd);
        println!("{out:?}");
        assert_eq!(out, CommandWrapper::Array(CommandArray { commands: vec![CommandWrapper::Cmd(CommandType::None)] }))
    }

    #[test]
    fn test_blk_str() {
        let input = BulkString::new("ping".as_bytes());
        let output = pkg_blk_str(&input);
        println!("{output:?}");
        assert!(output==CommandWrapper::Cmd(CommandType::Ping));
    }

    #[test]
    fn test_array() {
        let input = Array(RespArray::new(&[RespValue::BlkStr(BulkString::new("echo".as_bytes())), 
        RespValue::Array(RespArray::new(&[RespValue::BlkStr(BulkString::new("lol".as_bytes()))]))]
        ));
        let output = pkg_resp_val(&input);
        println!("{output:?}");
    }

    //echo hello -> hello 
}