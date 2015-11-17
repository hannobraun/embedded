use serial;
use serial::prelude::*;


pub fn init_port(path: &str) -> serial::Result<serial::SystemPort> {
	let mut port = serial::open(path).unwrap();

	try!(port.reconfigure(&|settings| {
		try!(settings.set_baud_rate(serial::Baud115200));
		settings.set_char_size(serial::Bits8);
		settings.set_parity(serial::ParityNone);
		settings.set_stop_bits(serial::Stop1);

		Ok(())
	}));

	Ok(port)
}
