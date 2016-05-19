use hardware::base::pmc::PMC;
use hardware::safe::peripherals::Peripheral;


pub fn enable_peripheral_clock_0(peripheral: &Peripheral) {
    unsafe {
        (*PMC).peripheral_clock_enable_0.write(peripheral.mask());
    }
}
