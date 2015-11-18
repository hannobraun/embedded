use std::io::{
	copy,
	sink,
};
use std::io::prelude::*;

use byteorder::{
	LittleEndian,
	ReadBytesExt,
};
use serial;

use result::Result;
use utils::ignore_timeout;


/// Interface to the SAM-BA monitor. See data sheet, chapter 20.4.
pub struct SamBa {
	port: serial::SystemPort,
}

impl SamBa {
	pub fn new(port: serial::SystemPort) -> Self {
		SamBa {
			port: port,
		}
	}

	/// Configures serial communication to be in normal mode (i.e. data is
	/// binary).
	pub fn set_normal_mode(&mut self) -> Result<()> {
		try!(write!(self.port, "N#"));

		// SAM-BA seems to send some kind of reply to that command, but I can't
		// find any information about what that reply actually is and why we
		// would need it. Let's just throw away anything that comes back and
		// never think about it again.
		try!(ignore_timeout(copy(&mut self.port, &mut sink())));

		Ok(())
	}

	pub fn display_version(&mut self) -> Result<String> {
		try!(write!(self.port, "V#"));

		let mut version = String::new();
		try!(ignore_timeout(self.port.read_to_string(&mut version)));

		Ok(version)
	}

	pub fn read_word(&mut self, address: u32) -> Result<u32> {
		try!(write!(self.port, "w{:0>8X},#", address));
		let result = try!(self.port.read_u32::<LittleEndian>());

		Ok(result)
	}

	pub fn write_word(&mut self, address: u32, value: u32) -> Result<()> {
		try!(write!(self.port, "W{:0>8X},{:0>8X}#", address, value));
		Ok(())
	}

	pub fn read_byte(&mut self, address: u32) -> Result<u8> {
		try!(write!(self.port, "o{:0>8X},#", address));
		let result = try!(self.port.read_u8());

		Ok(result)
	}

	pub fn write_byte(&mut self, address: u32, value: u8) -> Result<()> {
		try!(write!(self.port, "O{:0>8X},{:0>2X}#", address, value));
		Ok(())
	}
}
