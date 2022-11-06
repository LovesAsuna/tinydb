use std::collections::HashMap;
use crate::Database;
use anyhow::Result;

// Table the table of the database.
pub struct Table<'a, T> {
    name: String,
    database: &'a Database<T>,
}

impl<T> Database<T> {
    /// get a specific table from the database.
    pub fn get_table(&self, name: String) -> Table<T> {
        Table {
            name,
            database: self,
        }
    }

    /// drop a specific table from the database.
    pub fn drop_table(&self, name: String) -> Result<()> {
        let mut storage = self.storage.lock().unwrap();
        let mut data = storage.read()?;
        data.remove(&name);
        storage.write(data).map(|_| ())
    }
}

impl<T: Clone> Table<'_, T> {
    /// insert a new document into table.
    pub fn insert(&self, items: &[T]) -> Result<usize> {
        let mut storage = self.database.storage.lock().unwrap();
        let mut data = storage.read()?;
        let slot = data.entry(self.name.clone()).or_default();
        slot.append(&mut items.to_vec());
        let len = slot.len();
        storage.write(data).map(|_| len)
    }

    /// delete all matching documents in table.
    pub fn delete(&self, condition: Box<dyn Fn(&T) -> bool>) -> Result<Vec<T>> {
        let mut storage = self.database.storage.lock().unwrap();
        let mut data: HashMap<String, Vec<T>> = storage.read()?;
        if !data.contains_key(&self.name) {
            return Ok(Vec::new());
        }
        let slot = data.remove(&self.name).unwrap();
        let mut out = Vec::new();
        let mut left = Vec::new();
        for item in slot {
            if !condition(&item) {
                out.push(item);
            } else {
                left.push(item);
            }
        }
        data.insert(self.name.clone(), left);
        Ok(out)
    }

    /// update all macthing documents with updater.
    pub fn update(&self, updater: Box<dyn Fn(&mut T)>, condition: Box<dyn Fn(&T) -> bool>) -> Result<()> {
        let mut storage = self.database.storage.lock().unwrap();
        let mut data: HashMap<String, Vec<T>> = storage.read()?;
        if !data.contains_key(&self.name) {
            return Ok(());
        }
        let slot = data.remove(&self.name).unwrap();
        let mut out = Vec::new();
        for mut item in slot {
            if condition(&item) {
                updater(&mut item);
            }
            out.push(item);
        }
        data.insert(self.name.clone(), out);
        Ok(())
    }

    /// select for all documents matching condition.
    pub fn select(&self, condition: Box<dyn Fn(&T) -> bool>) -> Result<Vec<T>> where T: Clone {
        let mut storage = self.database.storage.lock().unwrap();
        let mut data: HashMap<String, Vec<T>> = storage.read()?;
        if !data.contains_key(&self.name) {
            return Ok(Vec::new());
        }
        let slot = data.remove(&self.name).unwrap();
        let mut out = Vec::new();
        for item in slot {
            if condition(&item) {
                out.push(item.clone());
            }
        }
        data.insert(self.name.clone(), out.clone());
        Ok(out)
    }
}