use std::collections::HashMap;
use std::sync::Mutex;
use crate::storages;
use anyhow::Result;

pub type Map<T> = HashMap<String, Vec<T>>;
pub type MapStorage<T> = Box<dyn storages::Storage<Map<T>>>;

// Database the tiny_db database.
pub struct Database<T> {
    pub storage: Mutex<MapStorage<T>>,
    table: String,
}

// tiny_db create a new database with the interface of storage.
pub fn tiny_db<T: Default>(storage: MapStorage<T>) -> Result<Database<T>> {
    let db = Database {
        storage: Mutex::new(storage),
        table: "_default".to_string(),
    };
    // lock
    {
        let mut storage = db.storage.lock().unwrap();
        let data = storage.read();
        let default_map: HashMap<String, Vec<T>> = [("_default".to_string(), vec![T::default()])].into_iter().collect();
        match data {
            Ok(data) => {
                if data.is_empty() {
                    storage.write(default_map)?;
                }
            }
            Err(_) => {
                storage.write(default_map)?;
            }
        }
    }
    Ok(db)
}

impl<T> Database<T> {
    // Close close the database.
    fn close(self) {}

    // Tables get the names of all tables in the database.
    fn tables(&mut self) -> Result<Vec<String>> {
        let mut storage = self.storage.lock().unwrap();
        let res: Result<Map<T>> = storage.read();
        match res {
            Ok(map) => {
                let mut v = Vec::new();
                for key in map.into_keys() {
                    v.push(key);
                }
                Ok(v)
            }
            Err(e) => {
                Err(e)
            }
        }
    }
}