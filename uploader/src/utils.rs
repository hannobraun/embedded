use std::io;


pub fn ignore_timeout<T: Default>(result: io::Result<T>) -> io::Result<T> {
	if let Err(ref error) = result {
		if error.kind() == io::ErrorKind::TimedOut {
			return Ok(Default::default())
		}
	}

	result
}
