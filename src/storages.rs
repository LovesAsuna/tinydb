use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Read, Seek, Write};
use std::io::SeekFrom::Start;
use anyhow::Result;

// Storage an interface of Storage & Middleware.
// Should implement the method of Read | Write | Close.
pub trait Storage<T> {
    fn read(&mut self) -> Result<T>;
    fn write(&mut self, any: T) -> Result<usize>;
    fn close(self);
}

// StorageJSON store the data in a JSON file.
pub struct StorageJSON {
    handle: File,
}

impl StorageJSON {
    // create a new JSONStorage instance.
    pub fn new(file: String) -> Self {
        let i1 = file.find('\\');
        let i2 = file.find('/');
        let dir = match (i1, i2) {
            (Some(i), None) => {
                file[..i].to_string()
            }
            (None, Some(i)) => {
                file[..i].to_string()
            }
            _ => { file }
        };
        StorageJSON {
            handle: OpenOptions::new().read(true).write(true).open(dir).unwrap(),
        }
    }
}

impl<'a, T: serde::Deserialize<'a> + serde::Serialize + Default> Storage<T> for StorageJSON {
    fn read(&mut self) -> Result<T> {
        let mut str = String::new();
        self.handle.seek(Start(0))?;
        self.handle.read_to_string(&mut str)?;
        if str.is_empty() {
            return Ok(T::default());
        }
        let res = serde_json::from_str(Box::leak(Box::new(str)))?;
        Ok(res)
    }

    fn write(&mut self, any: T) -> Result<usize> {
        let json = serde_json::to_string(&any)?;
        self.handle.seek(Start(0)).unwrap();
        let res = self.handle.write(json.as_bytes())?;
        Ok(res)
    }

    fn close(self) {}
}

pub struct MemoryStorage {
    memory: Vec<u8>,
}

impl MemoryStorage {
    pub fn new() -> Self {
        MemoryStorage {
            memory: Default::default(),
        }
    }
}

impl<'a, T: serde::Deserialize<'a> + serde::Serialize> Storage<T> for MemoryStorage {
    fn read(&mut self) -> Result<T> {
        if self.memory.is_empty() {
            return Err(anyhow::Error::from(io::Error::from(ErrorKind::UnexpectedEof)));
        }
        let res = serde_json::from_slice(Box::leak(Box::new(self.memory.clone())))?;
        Ok(res)
    }

    fn write(&mut self, any: T) -> Result<usize> {
        let s = serde_json::to_string(&any)?;
        self.memory = s.into_bytes();
        Ok(self.memory.len())
    }

    fn close(self) {}
}