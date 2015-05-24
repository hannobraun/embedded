use core::marker::PhantomData;

use hardware::base::pio;


pub fn a() -> Controller { Controller(pio::PIO_A) }
pub fn b() -> Controller { Controller(pio::PIO_B) }
pub fn c() -> Controller { Controller(pio::PIO_C) }
pub fn d() -> Controller { Controller(pio::PIO_D) }
pub fn e() -> Controller { Controller(pio::PIO_E) }
pub fn f() -> Controller { Controller(pio::PIO_F) }


pub struct Controller(*mut pio::Controller);

// TODO: Generate functions for all pins with a macro.
impl Controller {
	/// Create a new interface to a pin of this controller.
	///
	/// This function is unsafe, as only one Pin instance should exist per Pin.
	/// If multiple instances exist, the compiler can not statically enforce the
	/// correct use of the API.
	pub unsafe fn pin_27(&self) -> Pin<StatusUndefined, OutputStatusUndefined> {
		let &Controller(controller) = self;
		Pin::new(pio::P27, controller)
	}
}


/// Represents a single I/O pin.
///
/// Pin has two type parameters that encode its current status. This enables the
/// compiler to enforce correct use of the API at compile-time, e.g. making it
/// impossible to set an output value on a pin that is configured for input.
// TODO: Pin restricts the capabilities of the underlying hardware severely, by
//       limiting the operations it supports to only one pin at a time.
pub struct Pin<Status, OutputStatus> {
	mask      : u32,
	controller: *mut pio::Controller,

	status       : PhantomData<Status>,
	output_status: PhantomData<OutputStatus>,
}

impl<Status, OutputStatus> Pin<Status, OutputStatus> {
	fn new(mask: u32, controller: *mut pio::Controller)
		-> Pin<Status, OutputStatus>
	{
		Pin {
			mask      : mask,
			controller: controller,

			status       : PhantomData,
			output_status: PhantomData,
		}
	}
}

impl<OutputStatus> Pin<StatusUndefined, OutputStatus> {
	pub fn enable(self) -> Pin<StatusEnabled, OutputStatus> {
		unsafe { (*self.controller).pio_enable = self.mask };

		Pin::new(self.mask, self.controller)
	}
}

impl Pin<StatusEnabled, OutputStatusUndefined> {
	pub fn enable_output(self) -> Pin<StatusEnabled, OutputStatusEnabled> {
		unsafe { (*self.controller).output_enable = self.mask };

		Pin::new(self.mask, self.controller)
	}
}

impl Pin<StatusEnabled, OutputStatusEnabled> {
	pub fn set_output(&self) {
		unsafe { (*self.controller).set_output_data = self.mask };
	}

	pub fn clear_output(&self) {
		unsafe { (*self.controller).clear_output_data = self.mask };
	}
}


pub struct StatusUndefined;
pub struct StatusEnabled;

pub struct OutputStatusUndefined;
pub struct OutputStatusEnabled;
