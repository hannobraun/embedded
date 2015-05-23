use pio::{
	P27,
	PIO_B,
};
use rtt::RTT;


// This function is the entry point for our application and the handler function
// for the reset interrupt.
pub fn start() {
	unsafe {
		// Enable pin 27 of PIO_B (pin 13 on the Arduino Due) and configure it
		// for output.
		(*PIO_B).pio_enable    = P27;
		(*PIO_B).output_enable = P27;

		// Set the timer to a resolution of a millisecond.
		(*RTT).mode = 0x00000020;

		// Continuously set and clear output on pin 27 of PIO_B (pin 13 on the
		// Arduino Due). This blinks the Due's built-in LED, which is the single
		// purpose of this program.
		loop {
			(*PIO_B).set_output_data = P27;
			sleep_ms(200);
			(*PIO_B).clear_output_data = P27;
			sleep_ms(800);
		}
	}
}

// As the name suggests, this function sleeps for a given number of
// milliseconds. Our replacement for Arduino's delay function.
fn sleep_ms(milliseconds: u32) {
	// TODO: Since the timer resolution is 1024 Hz and not 1000 Hz, this
	//       function is not completely precise. Please don't use it for serious
	//       timekeeping.
	// TODO: Even though it might look that way to the untrained eye, this
	//       function doesn't take overflow into account. Rust's behavior in
	//       debug mode is to panic, if an integer operation overflows. While
	//       this code should work when compiled in release mode, it would be
	//       much nicer and more reliable to explicitly use wrapping integers.
	unsafe {
		let sleep_until = (*RTT).value + milliseconds;
		while (*RTT).value < sleep_until {}
	}
}
