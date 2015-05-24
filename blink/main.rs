use hardware::api::pio;
use hardware::base::rtt::RTT;


// This function is the entry point for our application and the handler function
// for the reset interrupt.
pub fn start() {
	// Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
	// Arduino Due, which is the built-in LED (labelled "L").
	let led = unsafe { pio::b().pin_27() };

	let led = led
		.enable()
		.enable_output();

	// Set the timer to a resolution of a millisecond.
	unsafe { (*RTT).mode = 0x00000020; }

	// TODO: Since we're not doing anything about the watchdog, the program is
	//       being restarted every 17-18 seconds. This messes up our nice
	//       blinking pattern.
	loop {
		led.set_output();
		sleep_ms(200);
		led.clear_output();
		sleep_ms(800);
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
