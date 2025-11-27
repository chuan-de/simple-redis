mod decode;
mod encode;

use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

pub trait RespDecode {
    fn decode(buf: Self) -> Result<RespFrame, anyhow::Error>;
}

#[enum_dispatch(RespEncode)]
#[derive(Debug,PartialEq)]
pub enum RespFrame {
    SimpleString(SimpleString),
    Error(SimpleError),
    Integer(i64),
    BulkString(BulkString),
    NullBulkString(RespNullBulkString),
    Array(RespArray),
    NullArray(RespNullArray),
    NUll(RespNull),
    Boolean(bool),
    Double(f64),
    Map(RespMap),
    Set(RespSet),
}

#[derive(Debug,PartialEq,Eq)]
pub struct SimpleString(String);
#[derive(Debug,PartialEq,Eq)]
pub struct SimpleError(String);
#[derive(Debug,PartialEq,Eq)]
pub struct BulkString(Vec<u8>);
#[derive(Debug,PartialEq,Eq)]
pub struct RespNull;
#[derive(Debug,PartialEq)]
pub struct RespArray(Vec<RespFrame>);
#[derive(Debug,PartialEq,Eq)]
pub struct RespNullArray;
#[derive(Debug,PartialEq,Eq)]
pub struct RespNullBulkString;
#[derive(Debug,PartialEq)]
pub struct RespMap(HashMap<String, RespFrame>);
#[derive(Debug,PartialEq)]
pub struct RespSet(Vec<RespFrame>);

impl Deref for SimpleString {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for BulkString {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespArray {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for RespMap {
    type Target = HashMap<String, RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for RespMap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for RespSet {
    type Target = Vec<RespFrame>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl SimpleString {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleString(s.into())
    }
}

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}

impl BulkString {
    pub fn new(s: impl Into<Vec<u8>>) -> Self {
        BulkString(s.into())
    }
}

impl RespArray {
    pub fn new(v: impl Into<Vec<RespFrame>>) -> Self {
        RespArray(v.into())
    }
}

impl RespMap {
    pub fn new() -> Self {
        RespMap(HashMap::new())
    }
}

impl RespSet {
    pub fn new(v:impl Into<Vec<RespFrame>>) -> Self {
        RespSet(v.into().into_iter().collect())
    }
}
