use hardware::base::rtt::RTT;


pub struct Timer;

impl Timer {
	/// Create an interface to the timer hardware.
	/// In theory, this should be an unsafe operation, as creating multiple
	/// interfaces to the same hardware will lead to confusion. In practice, the
	/// configuration is hardcoded here, so multiple instances of Timer can
	/// coexist peacefully without confusing each other.
	pub fn new() -> Timer {
		// Set the timer to a resolution of a millisecond
		unsafe { (*RTT).mode = 0x00000020; }
		Timer
	}

	/// As the name suggests, this function sleeps for a given number of
	/// milliseconds.
	pub fn sleep_ms(&self, milliseconds: u32) {
		// TODO: Since the timer resolution is 1024 Hz and not 1000 Hz, this
		//       function is not completely precise. Please don't use it for
		//       serious timekeeping.
		// TODO: Even though it might look that way to the untrained eye, this
		//       function doesn't take overflow into account. Rust's behavior in
		//       debug mode is to panic, if an integer operation overflows.
		//       While this code should work when compiled in release mode, it
		//       would be much nicer and more reliable to explicitly use
		//       wrapping integers.
		// TODO: This function doesn't really sleep. Rather, it waits busily,
		//       wasting a lot of resources.
		// TODO: This way of reading the timer value may not be accurate.
		//       According to section 13.4 of the data sheet:
		//       "As this value can be updated asynchronously from the Master
		//       Clock, it is advisable to read this register twice at the same
		//       value to improve accuracy of the returned value."
		unsafe {
			let sleep_until = (*RTT).value + milliseconds;
			while (*RTT).value < sleep_until {}
		}
	}
}
