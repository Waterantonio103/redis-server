//associated resp types

use crate::decode::RespValue;

#[derive(Debug, Clone)]
pub struct RespArray {
    data: Vec<RespValue>,
    length: usize,
}

impl RespArray {
    pub fn new(data: &[RespValue], length: usize) -> Self {
        Self { data: data.to_vec(), length }
    }

    pub fn iter(&self) -> impl Iterator<Item = &RespValue> {
        self.data.iter()
    }

    pub fn get(&self, index: usize) -> Option<&RespValue> {
        self.data.get(index)
    }
}

impl<'a> IntoIterator for &'a RespArray {
    type Item = &'a RespValue;
    type IntoIter = std::slice::Iter<'a , RespValue>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

#[derive(Debug, Clone)]
pub struct BulkString {
    data: Vec<u8>,
    length: usize,
}

impl BulkString {
    pub fn new(data: &[u8], length: usize) -> Self {
        Self { data: data.to_vec(), length }
    }

    pub fn data(&self) ->  &[u8] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}

#[derive(Debug, Clone)]
pub struct SimpleString {
    data: String,
}

impl SimpleString {
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

