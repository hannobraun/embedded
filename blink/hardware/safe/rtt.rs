use core::num::Wrapping;

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

	pub fn value(&self) -> u32 {
		// TODO: This way of reading the timer value may not be accurate.
		//       According to section 13.4 of the data sheet:
		//       "As this value can be updated asynchronously from the Master
		//       Clock, it is advisable to read this register twice at the same
		//       value to improve accuracy of the returned value."
		//       I'm not sure what that actually means. Can the value be updated
		//       in the background, so that only some bits have changed? In that
		//       case it would make sense to read twice, to make sure the update
		//       has finished.
		//       I don't really buy that though. I'm guessing that writing the
		//       value is atomic and I can't really read some in-between state.
		//       In that case, reading twice doesn't make any sense and I don't
		//       really understand what that comment from the data sheet means.
		unsafe { (*RTT).value }
	}

	/// Sleep for the given number of milliseconds.
	pub fn sleep_ms(&self, milliseconds: u32) {
		// TODO: Since the timer resolution is 1024 Hz and not 1000 Hz, this
		//       function is not completely precise. Please don't use it for
		//       serious timekeeping.
		// TODO: This function doesn't really sleep. Rather, it waits busily,
		//       wasting a lot of resources.
		// TODO: The behavior of this function is wrong, if the addition wraps.
		//       Instead of sleeping, the function will return immediately.
		let sleep_until = Wrapping(self.value()) + Wrapping(milliseconds);
		while self.value() < sleep_until.0 {}
	}
}
