use hardware::base::pmc::{
    MAINF_MASK,
    MAINFRDY,
    PMC,
    SLOW_CLOCK_FREQUENCY_HZ,
};
use hardware::safe::peripherals::Peripheral;


pub fn enable_peripheral_clock_0(peripheral: &Peripheral) {
    unsafe {
        (*PMC).peripheral_clock_enable_0.write(peripheral.mask());
    }
}

pub fn main_clock_frequency_hz() -> u32 {
    let main_clock_frequency_within_16_slow_clock_cycles = unsafe {
        while (*PMC).main_clock_frequency.read() & MAINFRDY == 0 {}
        (*PMC).main_clock_frequency.read() & MAINF_MASK
    };

    main_clock_frequency_within_16_slow_clock_cycles
        * SLOW_CLOCK_FREQUENCY_HZ / 16
}
