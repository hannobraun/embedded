use hardware::base::nvic::{
    ISER0,
    RTT,
};


pub struct Nvic(());

impl Nvic {
    pub unsafe fn new() -> Self {
        // `Nvic` has a private unit (`()`) field, to make it impossible to
        // create an instance, except by using this constructor.
        Nvic(())
    }

    pub fn enable_rtt(&mut self) {
        unsafe {
            (*ISER0).write(RTT);
        }
    }
}
