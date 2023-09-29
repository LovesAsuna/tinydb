use anyhow::Result;

use crate::model::KVPair;
use crate::model::value::Value;

pub(crate) mod memory;

pub trait Storage {
    fn get(&self, key: &str) -> Result<Option<Value>>;
    fn set(&mut self, key: &str, value: Value) -> Result<Option<Value>>;
    fn contains(&self, key: &str) -> Result<bool>;
    fn del(&mut self, key: &str) -> Result<Option<Value>>;
    fn get_all(&self) -> Result<Vec<KVPair>>;
    fn get_iter(&self) -> Result<Box<dyn Iterator<Item=KVPair>>>;
}