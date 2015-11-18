/// Program for interfacing with the SAM-BA bootloader on a SAM3X8E
/// microcontroller via USB.
///
/// Several comments refer the the SAM3X/SAM3A data sheet, available at the
/// following URI:
/// http://www.atmel.com/Images/Atmel-11057-32-bit-Cortex-M3-Microcontroller-SAM3X-SAM3A_Datasheet.pdf


mod eefc;
mod result;
mod sam_ba;
mod serial_port;
mod utils;


extern crate byteorder;
extern crate serial;


use std::env;

use eefc::{
	Eefc,
	GetGpnvmBit,
	GpnvmNumber,
	SetGpnvmBit,
};
use sam_ba::SamBa;


fn main() {
	let mut args = env::args();
	args.next().expect("Expected program name as first entry in args");

	let device_path = args.next().expect("Expected device path argument");
	let command     = args.next().expect("Expected command argument");

	let port = serial_port::init(&device_path)
		.expect("Failed to initialize serial port");

	let mut sam_ba = SamBa::new(port);
	let     eefc_0 = Eefc::eefc_0();

	sam_ba.set_normal_mode().expect("Failed to set normal mode");

	match command.as_ref() {
		"version" => {
			let version = sam_ba.display_version()
				.expect("Failed to retrieve version");

			print!("{}", version)
		},

		"boot-mode" => {
			let result =
				eefc_0.execute_command::<GetGpnvmBit, _>(
					&mut sam_ba,
					GpnvmNumber::BootModeSelection,
				)
				.expect("Failed to get GPNVM bit");

			print!("{:0>8x}\n", result)
		},
		"boot-from-flash" => {
			eefc_0.execute_command::<SetGpnvmBit, _>(
				&mut sam_ba,
				GpnvmNumber::BootModeSelection,
			)
			.expect("Failed to set GPNVM bit");
		},

		"read-word" => {
			let address = args.next().expect("Expected address argument");
			let address = u32::from_str_radix(&address, 16)
				.expect("Expected address to be a hexadecimal number");

			let value = sam_ba.read_word(address).expect("Failed to read word");

			print!("{:0>8x}\n", value);
		},
		"write-word" => {
			let address = args.next().expect("Expected address argument");
			let value   = args.next().expect("Expected value argument");

			let address = u32::from_str_radix(&address, 16)
				.expect("Expected address to be a hexadecimal number");
			let value = u32::from_str_radix(&value, 16)
				.expect("Expected value to be a hexadecimal number");

			sam_ba.write_word(address, value).expect("Failed to write word");
		},

		_ =>
			print!("Unknown command: {}\n", command),
	}
}
