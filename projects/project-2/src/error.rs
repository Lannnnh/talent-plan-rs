use failure::Fail;

#[derive(Debug, Fail)]
pub enum KvError {
    #[fail(display = "Key not found")]
    KeyNotFound,
}

pub type Result<T> = std::result::Result<T, KvError>;