use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::Arc;

use crate::database::Database;
use crate::storage::memory::MemTable;

struct MemoryDatabase {
    inner: HashMap<String, Arc<RefCell<MemTable>>>,
}

impl Database<MemTable> for MemoryDatabase {
    fn tables(&self) -> anyhow::Result<Vec<String>> {
        Ok(self.inner.keys().map(|k| k.clone()).collect::<Vec<_>>())
    }

    fn get_or_create_table(&mut self, name: &str) -> anyhow::Result<Arc<RefCell<MemTable>>> {
        let entry = self.inner.entry(name.to_owned());
        let table = entry.or_insert(Arc::new(RefCell::new(MemTable::new(name.to_owned()))));
        Ok(Arc::clone(table))
    }
}