use crate::{commands::{AssignError}, decode::{RespValue, resp_types::{BulkString, RespArray}}};

//when adding commands be sure to check everywhere
pub const COMMANDS: [&[u8]; 2] = [PING, ECHO];
#[derive(Debug, PartialEq)]
pub enum CommandType {
    Ping,
    Echo,
    None,
}

pub const PING: &[u8] = "PING".as_bytes();
#[derive(Debug)]
pub struct Ping;

pub const ECHO: &[u8] = "ECHO".as_bytes();
#[derive(Debug)]
pub struct Echo<'a> {
    msg: &'a BulkString,
}

impl<'a> Echo<'a> {
    #[deprecated(note="check logic and error cases")]
    pub fn new(msg: &'a BulkString) -> Self {
        Echo { msg }
    }

    pub fn from_array(array: &'a RespArray) -> Result<Self, AssignError> {
        //get echo from array and ensure its right structure
		if array.len() != 2 { return Err(AssignError::InvalidStructure) }
		match array.get(1).unwrap() {
			RespValue::BlkStr(str) => Ok(Self::new(str)),
			_ => Err(AssignError::InvalidEncoding),
		}
    }

    pub fn data_clone(self) -> BulkString {
        self.msg.clone()
    }

    //prolly useless function so wait
    // pub fn get_expected(&mut self, input: &'a RespArray, index: usize) -> AssignResult<()> {
    //     if let Some(value) = input.get(index) {
    //         match value {
    //             RespValue::BlkStr(blkstr) => { 
    //                 self.msg = blkstr;
    //                 return Valid(());
    //             },
    //             _ => { return Error(AssignError::InvalidEncoding); },
    //         }
    //     } else {
    //         return Error(AssignError::ArrayIndex);
    //     }
    // }
}
