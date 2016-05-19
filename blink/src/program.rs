use core::fmt::Write;

use hardware::safe::nvic::Nvic;
use hardware::safe::pio;
use hardware::safe::rtt::sleep_ms;
use hardware::safe::uart::Uart;
use hardware::safe::wdt::restart_watchdog;


pub fn start() {
    let mut nvic = unsafe { Nvic::new() };

    // Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
    // Arduino Due, which is the built-in LED (labelled "L").
    let led = unsafe { pio::b().pin_27() };
    let mut led = led
        .enable()
        .enable_output();

    let     uart_tx = unsafe { pio::a().pin_9() };
    let mut uart    = unsafe { Uart::new(uart_tx) };

    loop {
        // Ignore logging errors. It's not worth killing the program because of
        // failed debug output. It would be nicer to safe and report them
        // later, however.
        let _ = write!(uart, "Start main loop iteration\n");

        restart_watchdog();

        led.set_output();
        sleep_ms(200, &mut nvic);
        led.clear_output();
        sleep_ms(800, &mut nvic);
    }
}
