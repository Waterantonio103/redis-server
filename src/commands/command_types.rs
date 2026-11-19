use crate::decode::resp_types::BulkString;

pub const COMMANDS: [&[u8]; 2] = [PING, ECHO];
#[derive(Debug, PartialEq)]
pub enum Command {
    Ping,
    Echo,
    None,
}

pub const PING: &[u8] = "PING".as_bytes();
#[derive(Debug)]
pub struct Ping;


pub const ECHO: &[u8] = "ECHO".as_bytes();
#[derive(Debug)]
pub struct Echo {
    msg: BulkString,
}
