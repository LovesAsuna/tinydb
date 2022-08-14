use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::sync::Mutex;
use crate::storages;
use crate::Result;

// Database the tiny_db database.
pub struct Database<T> {
    pub storage: Mutex<Box<dyn storages::Storage<T>>>,
    table: String,
}

// tiny_db create a new database with the interface of storage.
pub fn tiny_db<T>(storage: Box<dyn storages::Storage<HashMap<String, Vec<T>>>>) -> Result<Database<HashMap<String, Vec<T>>>> {
    let db = Database {
        storage: Mutex::new(storage),
        table: "_default".to_string(),
    };
    let mut storage = db.storage.lock().unwrap();
    let mut data = storage.read();
    match data {
        Ok(data) => {
            if data.is_empty() {
                storage.write([("_default".to_string(), Vec::new())].into_iter().collect::<HashMap<_, _>>())?;
            }
            drop(storage);
            Ok(db)
        }
        Err(e) => {
            if e.to_string() == ErrorKind::UnexpectedEof.to_string() {
                storage.write([("_default".to_string(), Vec::new())].into_iter().collect::<HashMap<_, _>>())?;
                drop(storage);
                Ok(db)
            } else {
                Err(e)
            }
        }
    }
}

impl<T> Database<HashMap<String, Vec<T>>> {
    // Close close the database.
    fn close(self) {}

    // Tables get the names of all tables in the database.
    fn tables(&mut self) -> Result<Vec<String>> {
        let mut storage = self.storage.lock().unwrap();
        let res: Result<HashMap<String, Vec<T>>> = storage.read();
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