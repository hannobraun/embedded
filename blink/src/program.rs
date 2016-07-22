use core::u8;

use hardware::safe::nvic::Nvic;
use hardware::safe::pio;
use hardware::safe::rtt::sleep_ms;
use hardware::safe::uart::{
    self,
    Uart,
};
use hardware::safe::wdt::restart_watchdog;
use interrupts;


pub fn start() {
    // Disable interrupts in general. They will only be enabled where they are
    // actually needed.
    interrupts::disable();

    let mut nvic = unsafe { Nvic::new() };

    let uart_tx = unsafe { pio::a().pin_9() };
    let mut uart = unsafe { Uart::new(uart_tx) };

    let mut value    = 0;
    let mut count_up = true;
    loop {
        restart_watchdog();

        uart.write_byte(value);
        if let Err(error) = uart.check_for_errors() {
            match error {
                uart::Error::Overrun => blink(nvic, 100, 900),
                uart::Error::Framing => blink(nvic, 900, 100),
                uart::Error::Parity  => blink(nvic, 500, 500),
            }
        }

        if count_up {
            value += 1;
        }
        else {
            value -= 1;
        }
        if value == 0 || value == u8::MAX {
            count_up = !count_up;
        }


        sleep_ms(100, &mut nvic);
    }
}

fn blink(mut nvic: Nvic, v1: u32, v2: u32) -> ! {
    // Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
    // Arduino Due, which is the built-in LED (labelled "L").
    let led = unsafe { pio::b().pin_27() };
    let mut led = led
        .enable()
        .enable_output();

    loop {
        led.set_output();
        sleep_ms(v1, &mut nvic);
        led.clear_output();
        sleep_ms(v2, &mut nvic);
    }
}
