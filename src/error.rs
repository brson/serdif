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

use std::result::Result as StdResult;

pub trait ResultExt<T> {
    fn e(self) -> StdResult<T, Error>;
}

impl<T, E> ResultExt<T> for StdResult<T, E>
where E: StdError + Send + Sync + 'static {
    fn e(self) -> StdResult<T, Error> {
        self.map_err(|e| Error(anyhow::Error::new(e)))
    }
}
