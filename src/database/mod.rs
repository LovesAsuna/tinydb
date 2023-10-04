use std::cell::RefCell;
use std::sync::Arc;

use anyhow::Result;

use crate::storage::Storage;

mod memory;

pub trait Database<T: Storage> {
    fn tables(&self) -> Result<Vec<String>>;

    fn get_or_create_table(&mut self, name: &str) -> Result<Arc<RefCell<T>>>;

    fn drop_table(&mut self, name: &str) -> Result<()>;
}