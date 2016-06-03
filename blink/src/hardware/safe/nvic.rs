use hardware::base::nvic::{
    ISER,
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
            (*ISER)[0].write(RTT);
        }
    }
}
