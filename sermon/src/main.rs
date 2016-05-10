extern crate serial;


use std::io;
use std::io::prelude::*;
use std::sync::mpsc::{
    channel,
    Receiver,
    TryRecvError,
};
use std::thread;

use serial::prelude::*;


fn main() {
    let path = "/dev/ttyACM0";

    let mut serial_port = open_port(path)
        .expect("Failed to open serial port");

    let input = start_input_reader();

    loop {
        if let Err(error) = io::copy(&mut serial_port, &mut io::stdout()) {
            if error.kind() != io::ErrorKind::TimedOut {
                panic!("Failed to print serial output: {}");
            }
        }

        match input.try_recv() {
            Ok(input) =>
                serial_port.write_all(input.as_bytes())
                    .expect("Failed to write to serial port"),

            Err(TryRecvError::Empty)        => (),
            Err(TryRecvError::Disconnected) => panic!("Channel disconnected"),
        };
    }
}


fn open_port(path: &str) -> serial::Result<serial::SystemPort> {
    let mut port = try!(serial::open(path));

    try!(port.configure(&serial::PortSettings {
        baud_rate   : serial::Baud9600,
        char_size   : serial::Bits8,
        parity      : serial::ParityNone,
        stop_bits   : serial::Stop1,
        flow_control: serial::FlowNone,
    }));

    Ok(port)
}

fn start_input_reader() -> Receiver<String> {
    let (sender, receiver) = channel();

    thread::spawn(move || {
        let stdin = io::stdin();

        loop {
            let mut input = String::new();
            stdin.read_line(&mut input)
                .expect("Failed to read from stdin");
            sender.send(input)
                .expect("Failed to send input via channel");
        }
    });

    receiver
}
