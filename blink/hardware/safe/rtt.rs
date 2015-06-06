use hardware::base::rtt::RTT;


pub struct Timer;

impl Timer {
	/// Create an interface to the timer hardware.
	/// In theory, this should be an unsafe operation, as creating multiple
	/// interfaces to the same hardware will lead to confusion. In practice, the
	/// configuration is hardcoded here, so multiple instances of Timer can
	/// coexist peacefully without confusing each other.
	pub fn new() {
		// Set the timer to a resolution of a millisecond
		unsafe { (*RTT).mode = 0x00000020; }
	}
}
