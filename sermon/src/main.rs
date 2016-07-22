extern crate serial;


use std::io;
use std::io::prelude::*;

use serial::prelude::*;


fn main() {
    let path = "/dev/ttyACM0";

    let serial_port = open_port(path)
        .expect("Failed to open serial port");

    for value in serial_port.bytes() {
        let value = match value {
            Ok(value) => value,
            Err(error) => {
                if error.kind() != io::ErrorKind::TimedOut {
                    panic!("Failed to print serial output: {}", error);
                }
                else {
                    continue;
                }
            },
        };

        print!("{}\n", value);

        if let Err(error) = io::stdout().flush() {
            panic!("Failed to flush stdout: {}", error);
        }
    }
}


fn open_port(path: &str) -> serial::Result<serial::SystemPort> {
    let mut port = try!(serial::open(path));

    try!(port.configure(&serial::PortSettings {
        baud_rate   : serial::Baud9600,
        char_size   : serial::Bits8,
        parity      : serial::ParityEven,
        stop_bits   : serial::Stop1,
        flow_control: serial::FlowNone,
    }));

    Ok(port)
}
