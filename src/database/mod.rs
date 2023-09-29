mod memory;

use std::cell::RefCell;
use std::sync::Arc;
use crate::storage::Storage;
use anyhow::Result;

pub trait Database<T: Storage> {
    fn tables(&self) -> Result<Vec<String>>;

    fn get_or_create_table(&mut self, name: &str) -> Result<Arc<RefCell<T>>>;
}