use std::io::{Read, Write, Seek};

pub trait Buffer: Read + Write + Seek + Send + Sync + 'static { }

impl<T> Buffer for T
where T: Read + Write + Seek + Send + Sync + 'static { }

pub struct State {
    pub buf: Box<dyn Buffer>,
}
