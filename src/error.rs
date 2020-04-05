use std;
use std::fmt::{self, Display};
use std::error::Error as StdError;

use serde::{de, ser};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub struct Error(anyhow::Error);

impl ser::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(anyhow::Error::msg(msg.to_string()))
    }
}

impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error(anyhow::Error::msg(msg.to_string()))
    }
}

impl Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(formatter)
    }
}

impl StdError for Error {}

impl From<anyhow::Error> for Error {
    fn from(e: anyhow::Error) -> Error {
        Error(e)
    }
}

use std::result::Result as StdResult;

pub trait StdResultExt<T> {
    fn e(self) -> StdResult<T, Error>;
}

impl<T, E> StdResultExt<T> for StdResult<T, E>
where E: StdError + Send + Sync + 'static {
    fn e(self) -> StdResult<T, Error> {
        self.map_err(|e| Error(anyhow::Error::new(e)))
    }
}

pub trait ResultExt {
    fn cmd_eof(&self) -> bool;
}

impl<T> ResultExt for Result<T> {
    fn cmd_eof(&self) -> bool {
        match self {
            Ok(_) => false,
            Err(ref e) => {
                if let Some(e) = e.0.downcast_ref::<serde_json::Error>() {
                    e.is_eof()
                } else {
                    false
                }
            }
        }
    }
}
