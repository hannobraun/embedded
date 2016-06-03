use hardware::base::nvic::ISER;
use hardware::safe::peripherals::Peripheral;


pub struct Nvic(());

impl Nvic {
    pub unsafe fn new() -> Self {
        // `Nvic` has a private unit (`()`) field, to make it impossible to
        // create an instance, except by using this constructor.
        Nvic(())
    }

    pub fn enable(&mut self, peripheral: &Peripheral) {
        unsafe {
            (*ISER)[peripheral.index()].write(peripheral.mask());
        }
    }
}
