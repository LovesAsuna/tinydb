use std::collections::HashMap;
use anyhow::Result;

use crate::model::KVPair;
use crate::model::Value;
use crate::storage::Storage;

pub struct MemTable {
    name: String,
    inner: HashMap<String, Value>,
}

impl MemTable {
    pub fn new(name: String) -> Self {
        MemTable {
            name,
            inner: HashMap::new(),
        }
    }
}

impl Storage for MemTable {
    fn get(&self, key: &str) -> Result<Option<Value>> {
        Ok(self.inner.get(key).map(|&v| v))
    }

    fn set(&mut self, key: &str, value: Value) -> Result<Option<Value>> {
        let _value = self.inner.entry(key.to_owned()).or_default();
        let old = _value.clone();
        *_value = value;
        Ok(Some(old))
    }

    fn contains(&self, key: &str) -> Result<bool> {
        Ok(self.inner.contains_key(&key.to_owned()))
    }

    fn del(&mut self, key: &str) -> Result<Option<Value>> {
        Ok(self.inner.remove(&key.to_owned()))
    }

    fn get_all(&self) -> Result<Vec<KVPair>> {
        let mut res = Vec::with_capacity(self.inner.len());
        for (k, v) in self.inner.iter() {
            let pair = KVPair {
                key: k.clone(),
                value: Some(v.clone()),
            };
            res.push(pair);
        }
        Ok(res)
    }

    fn get_iter(&self) -> Result<Box<dyn Iterator<Item=KVPair>>> {
        let iter = self.inner.iter().map(|(k, v)| {
            KVPair {
                key: k.clone(),
                value: Some(v.clone()),
            }
        });
        Ok(Box::new(iter))
    }
}