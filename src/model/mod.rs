pub struct Value {
    pub value: Option<value::Value>
}

pub mod value {
    pub enum Value {
        String(String),
        Binary([u8]),
        Integer(i64),
        Float(f64),
        Bool(bool)
    }
}

pub struct KVPair {
    pub key: String,
    pub value: Option<Value>
}