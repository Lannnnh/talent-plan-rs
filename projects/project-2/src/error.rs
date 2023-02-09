use failure::Fail;
use std::io;

#[derive(Debug, Fail)]
pub enum KvError {
    #[fail(display = "Key not found")]
    KeyNotFound,
    #[fail(display = "{}", _0)]
    IOError(#[cause] io::Error),
}

pub type Result<T> = std::result::Result<T, KvError>;

impl From<io::Error> for KvError {
    fn from(err: io::Error) -> KvError {
        KvError::IOError(err)
    }
}
