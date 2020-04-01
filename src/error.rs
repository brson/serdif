use std;
use std::fmt::{self, Display};

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

impl std::error::Error for Error {}

