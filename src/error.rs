use std::any::TypeId;
use std::fmt::{Debug, Display, Formatter};
use std::result;

pub type Result<T> = result::Result<T, Error>;

pub struct Error {
    origin: Box<dyn std::error::Error>,
    expr: String
}

impl Error {
    pub fn from_source(origin: Box<dyn std::error::Error>) -> Self {
        let expr = origin.to_string();
        Error {
            origin,
            expr
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.origin, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.origin, f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self.origin.as_ref())
    }
}