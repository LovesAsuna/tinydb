use crate::Storage;
use anyhow::Result;

// MiddlewareCaching
struct MiddleWareCaching<T> {
    storage: Box<dyn Storage<T>>,
    cache: Vec<u8>,
    count: isize,
    size: isize
}

impl<'a, T: serde::Deserialize<'a> + serde::Serialize> MiddleWareCaching<T> {
    /// Create a new CachingMiddleware instance.
    fn new(storage: Box<dyn Storage<T>>, size: isize) -> Self {
        MiddleWareCaching {
            storage,
            cache: Default::default(),
            count: Default::default(),
            size
        }
    }

    /// Read data from MiddlewareCaching cache.
    fn read(&mut self) -> Result<T> {
        if self.cache.is_empty() {
            return self.storage.read();
        }
        let res = serde_json::from_slice(Box::leak(Box::new(self.cache.clone())))?;
        Ok(res)
    }

    /// Write data to MiddlewareCaching cache.
    fn write(&mut self, any: T) -> Result<usize> {
        let json = serde_json::to_string(&any)?;
        self.cache = json.into_bytes();
        self.count += 1;
        if self.count > self.size {
            return self.storage.write(any);
        }
        return Ok(self.cache.len())
    }

    /// Close the MemoryStorage instance.
    fn close(self) {}
}