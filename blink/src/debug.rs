use hardware::safe::uart::{
    self,
    Uart,
};


pub static mut UART: Option<Uart> = None;


pub unsafe fn init(pin: uart::UndefinedPin) {
    UART = Some(Uart::new(pin));
}


macro_rules! print {
    ($($args:tt)*) => {
        {
            #[allow(unused_unsafe)]
            let uart = unsafe { &mut $crate::debug::UART };

            // Ignore logging errors. It's not worth killing the program
            // because of failed debug output. It would be nicer to save the
            // error and report it later, however.
            if let &mut Some(ref mut uart) = uart {
                use core::fmt::Write;
                let _ = write!(uart, $($args)*);
            }
        }
    }
}

macro_rules! println {
    ($fmt:expr)               => ( print!(concat!($fmt, '\n')) );
    ($fmt:expr, $($args:tt)*) => ( print!(concat!($fmt, '\n'), args) );
}
