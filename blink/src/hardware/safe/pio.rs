use core::marker::PhantomData;

use hardware::base::pio;


pub fn a() -> Controller { Controller(pio::PIO_A) }
pub fn b() -> Controller { Controller(pio::PIO_B) }
pub fn c() -> Controller { Controller(pio::PIO_C) }
pub fn d() -> Controller { Controller(pio::PIO_D) }
pub fn e() -> Controller { Controller(pio::PIO_E) }
pub fn f() -> Controller { Controller(pio::PIO_F) }


pub struct Controller(*mut pio::Controller);

macro_rules! impl_controller {
    ( $($number:expr, $fn_name:ident, $const_name:ident;)* ) => {
        impl Controller {
            $(
                /// Create a new interface to pin $number of this controller.
                ///
                /// This function is unsafe, as only one Pin instance should
                /// exist per Pin. If multiple instances exist, the compiler
                /// can not statically enforce the correct use of the API.
                ///
                /// Initially, both the status and the output status are
                /// undefined. This has two reasons:
                /// - The pin might have been used before during the program.
                ///   We don't know what status it was left in.
                /// - After the system has been powered up, we can't rely on
                ///   pins being in a specific state (see data sheet, chapter
                ///   31.5.2).
                pub unsafe fn $fn_name(&self)
                    -> Pin<status::Undefined, output_status::Undefined>
                {
                    let &Controller(controller) = self;
                    Pin::new(pio::$const_name, controller)
                }
            )*
        }
    }
}

// Generate `Controller` implementation. Each line generates a function that
// returns a specific pin. More functions can be added as needed.
impl_controller! {
    27, pin_27, P27;
}


/// Represents a single I/O pin.
///
/// Pin has two type parameters that encode its current status. This enables
/// the compiler to enforce correct use of the API at compile-time, e.g. making
/// it impossible to set an output value on a pin that is configured for input.
///
/// Please note that this abstraction restricts the capabilities of the
/// underlying hardware by limiting the operations it supports to only one pin
/// at a time. If you need to manipulate multiple pins at once in performance-
/// sensitive code, this abstraction might not be the right fit.
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

impl<OutputStatus> Pin<status::Undefined, OutputStatus> {
    pub fn enable(self) -> Pin<status::Enabled, output_status::Undefined> {
        unsafe {
            (*self.controller).pio_enable.write(self.mask)
        }

        Pin::new(self.mask, self.controller)
    }
}

impl Pin<status::Enabled, output_status::Undefined> {
    pub fn enable_output(self)
        -> Pin<status::Enabled, output_status::Enabled>
    {
        unsafe {
            (*self.controller).output_enable.write(self.mask)
        }

        Pin::new(self.mask, self.controller)
    }
}

impl Pin<status::Enabled, output_status::Enabled> {
    pub fn set_output(&self) {
        unsafe {
            (*self.controller).set_output_data.write(self.mask)
        }
    }

    pub fn clear_output(&self) {
        unsafe {
            (*self.controller).clear_output_data.write(self.mask)
        }
    }
}


mod status {
    pub struct Undefined;
    pub struct Enabled;
}

mod output_status {
    pub struct Undefined;
    pub struct Enabled;
}
