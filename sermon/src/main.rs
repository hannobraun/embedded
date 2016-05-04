extern crate serial;


use std::io;

use serial::prelude::*;


fn main() {
    let path = "/dev/ttyACM0";

    let mut serial_port = open_port(path)
        .expect("Failed to open serial port");

    loop {
        if let Err(error) = io::copy(&mut serial_port, &mut io::stdout()) {
            if error.kind() != io::ErrorKind::TimedOut {
                panic!("Failed to print serial output: {}");
            }
        }
    }
}


fn open_port(path: &str) -> serial::Result<serial::SystemPort> {
    let mut port = try!(serial::open(path));

    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(serial::Baud9600));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);

        Ok(())
    }));

    Ok(port)
}
