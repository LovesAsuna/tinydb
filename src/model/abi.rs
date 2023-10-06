pub struct CommandResponse {
    pub status: u32,
    pub message: String,
    pub values: Vec<Value>,
    pub pairs: Vec<KVPair>,
}

#[derive(Default, Clone)]
pub struct Value {
    pub value: Option<value::Value>,
}

impl Value {
    fn new() -> Self {
        Value {
            value: None
        }
    }

    pub fn value(&self) -> &Option<value::Value> {
        &self.value
    }
}

pub mod value {
    pub enum Value {
        String(String),
        Binary([u8]),
        Integer(i64),
        Float(f64),
        Bool(bool),
    }
}

pub struct KVPair {
    pub key: String,
    pub value: Option<Value>,
}

#[derive(Debug)]
pub struct Hget {
    pub table : String,
    pub key: String,
}

#[derive(Debug)]
pub struct Hmget {
    pub table : String,
    pub keys: Vec<String>
}

#[derive(Debug)]
pub struct Hgetall {
    pub table : String,
}

#[derive(Debug)]
pub struct Hset {
    pub table : String,
    pub pair: Option<KVPair>,
}

#[derive(Debug)]
pub struct Hmset {
    pub table : String,
    pub pairs: Vec<KVPair>,
}

#[derive(Debug)]
pub struct Hdel {
    pub table : String,
    pub key: String,
}

#[derive(Debug)]
pub struct Hmdel {
    pub table : String,
    pub keys: Vec<String>
}

#[derive(Debug)]
pub struct Hexist {
    pub table: String,
    pub key: String,
}

#[derive(Debug)]
pub struct Hmexist {
    pub table: String,
    pub keys: Vec<String>,
}

pub struct Hdrop {
    pub table : String,
}

pub struct Hmdrop {
    pub tables : Vec<String>,
}
