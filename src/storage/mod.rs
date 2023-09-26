mod memory;

use crate::model::KVPair;
use crate::model::value::Value;

pub trait Storage {
    fn get(&self, table: &str, key: &str) -> anyhow::Result<Option<Value>>;
    fn set(&self, table: &str, key: String, value: Value) -> anyhow::Result<Option<Value>>;
    fn contains(&self, table: &str, key: &str) -> anyhow::Result<bool>;
    fn del(&self, table: &str, key: &str) -> anyhow::Result<Option<Value>>;
    fn get_all(&self, table: &str) -> anyhow::Result<Vec<KVPair>>;
    fn get_iter(&self, table: &str) -> anyhow::Result<Box<dyn Iterator<Item = KVPair>>>;
}