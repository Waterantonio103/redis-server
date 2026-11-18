//associated resp types

use crate::parser::RespValue;

#[derive(Debug, Clone)]
pub struct RespArray {
    data: Vec<RespValue>,
    length: usize,
}

impl RespArray {
    pub fn new(data: &[RespValue], length: usize) -> Self {
        Self { data: data.to_vec(), length }
    }
}

#[derive(Debug, Clone)]
pub struct BlkStr {
    data: Vec<u8>,
    length: usize,
}

impl BlkStr {
    pub fn new(data: &[u8], length: usize) -> Self {
        Self { data: data.to_vec(), length }
    }
}

#[derive(Debug, Clone)]
pub struct SmplStr {
    data: String,
}

impl SmplStr {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

#[derive(Debug, Clone)]
pub struct RespError {
    code: Vec<u8>,
    msg: Vec<u8>,
}

impl RespError {
    pub fn new(code: &[u8], msg: &[u8]) -> Self {
        Self { code: code.to_vec(), msg: msg.to_vec() }
    }
}

