extern crate serial;


use std::io;
use std::io::prelude::*;

use serial::prelude::*;


fn main() {
    let path = "/dev/ttyACM0";

    let serial_port = open_port(path)
        .expect("Failed to open serial port");

    let mut last_value = None;
    let mut num_errors = 0;

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

        print!("Number of Errors: {} / Value: {:0>#04X}", num_errors, value);

        if let Some(last_value) = last_value {
            if (value as i16 - last_value as i16).abs() != 1 {
                print!(" <= Error\n");
                num_errors += 1;
            }
            else {
                print!("\n");
            }
        }
        else {
            print!("\n");
        }

        last_value = Some(value);


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
