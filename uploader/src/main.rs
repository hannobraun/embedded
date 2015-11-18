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
use std::num::ParseIntError;

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
			let address = parse_u32_hex(args.next(), "address");

			let value = sam_ba.read_word(address).expect("Failed to read word");

			print!("{:0>8x}\n", value);
		},
		"write-word" => {
			let address = parse_u32_hex(args.next(), "address");
			let value   = parse_u32_hex(args.next(), "value");

			sam_ba.write_word(address, value).expect("Failed to write word");
		},

		"read-byte" => {
			let address = parse_u32_hex(args.next(), "address");

			let value = sam_ba.read_byte(address).expect("Failed to read byte");

			print!("{:0>2x}\n", value);
		},
		"write-byte" => {
			let address = parse_u32_hex(args.next(), "address");
			let value   = parse_u8_hex(args.next(), "value");

			sam_ba.write_byte(address, value).expect("Failed to write byte");
		},

		_ =>
			print!("Unknown command: {}\n", command),
	}
}


fn parse_u8_hex(arg: Option<String>, name: &str) -> u8 {
	parse_hex(arg, name, u8::from_str_radix)
}

fn parse_u32_hex(arg: Option<String>, name: &str) -> u32 {
	parse_hex(arg, name, u32::from_str_radix)
}

fn parse_hex<T>(
	arg  : Option<String>,
	name : &str,
	parse: fn(&str, u32) -> Result<T, ParseIntError>,
) -> T {
	let arg = arg.expect(&format!("Expected {} argument", name));
	parse(&arg, 16)
		.expect(&format!("Expected {} to be a hexadecimal number", name))
}
