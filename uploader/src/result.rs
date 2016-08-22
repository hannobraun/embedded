use std::io;
use std::result;


pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
    Io(io::Error),
}

impl From<io::Error> for Error {
    fn from(io_error: io::Error) -> Self {
        Error::Io(io_error)
    }
}
