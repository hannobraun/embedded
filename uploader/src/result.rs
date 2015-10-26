use std::io;
use std::result;

use byteorder;


pub type Result<T> = result::Result<T, Error>;


#[derive(Debug)]
pub enum Error {
	ByteOrder(byteorder::Error),
	Io(io::Error),
}

impl From<byteorder::Error> for Error {
	fn from(error: byteorder::Error) -> Self {
		Error::ByteOrder(error)
	}
}

impl From<io::Error> for Error {
	fn from(io_error: io::Error) -> Self {
		Error::Io(io_error)
	}
}
