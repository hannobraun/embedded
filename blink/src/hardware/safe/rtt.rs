use hardware::base::rtt::{
    self,
    RTT,
};
use hardware::safe::nvic::Nvic;
use hardware::safe::peripherals::Peripheral;


/// Sleep for a period of time roughly corresponding to the given number
/// of milliseconds.
///
/// The timer is set to a frequency of 1024 Hz, not 1000 Hz, which means
/// the waiting time is off by about 2.4%. Please don't use this method for
/// serious timekeeping.
///
/// This function configures the RTT and the NVIC as required, but doesn't
/// currently reset the configuration. This doesn't matter in the context
/// of the current program, but it might be nicer to reset everything to
/// the previous value, if this were used in a larger program where other
/// parts of the code might also sleep.
pub fn sleep_ms(milliseconds: u32, nvic: &mut Nvic) {
    let prescaler_value = 0x00000020; // millisecond resolution (roughly)
    unsafe {
        (*RTT).mode.write(
            rtt::RTTRST | rtt::ALMIEN | prescaler_value
        );

        // The reset is only effective after two slow clock cycles. Let's
        // just wait until that has happened.
        // See data sheet, section 13.4.
        while (*RTT).value.read() != 0 {}

        (*RTT).alarm.write(milliseconds);
    }

    nvic.enable(&Peripheral::Rtt);

    unsafe {
        while (*RTT).status.read() & rtt::ALMS == 0 {
            // Wait for interrupt.
            // Interrupts don't actually need to be enabled for this to work.
            // The processor will wake up again, even if the interrupt is just
            // pending and doesn't actually fire.
            // The data synchronization barrier before the interrupt is added
            // to ensure all memory accesses have completed before putting the
            // processor to sleep. This might not be strictly necessary in this
            // case, but it's safer in general. The wake-up condition might be
            // modified before sleeping, which could cause the program to not
            // wake up as expected.
            asm!(
                "
                    dsb
                    wfi
                "
                :::: "volatile"
            );
        }
    }
}
