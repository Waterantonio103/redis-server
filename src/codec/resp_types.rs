//associated resp types

use crate::codec::decode::RespValue;

#[derive(Debug, Clone)]
pub struct RespArray {
    data: Vec<RespValue>,
    length: usize,
}

impl RespArray {
    //add logic to check length is correct, be sure to do this everywhere son
    pub fn new(data: &[RespValue]) -> Self {
        Self { data: data.to_vec(), length: data.len() }
    }

    pub fn iter(&self) -> impl Iterator<Item = &RespValue> {
        self.data.iter()
    }

    pub fn len(&self) -> usize {
        self.data.len()
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
    pub fn new(data: &[u8]) -> Self {
        Self { data: data.to_vec(), length: data.len() }
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
    data: Vec<u8>,
}

impl SimpleString {
    pub fn new(data: &[u8]) -> Self {
        Self { data: data.to_vec() }
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

