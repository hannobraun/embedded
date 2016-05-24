use core::fmt::Write;

use debug;
use hardware::safe::nvic::Nvic;
use hardware::safe::pio;
use hardware::safe::rtt::sleep_ms;
use hardware::safe::wdt::restart_watchdog;


pub fn start() {
    let mut nvic = unsafe { Nvic::new() };

    // Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
    // Arduino Due, which is the built-in LED (labelled "L").
    let led = unsafe { pio::b().pin_27() };
    let mut led = led
        .enable()
        .enable_output();

    let uart_tx = unsafe { pio::a().pin_9() };
    unsafe { debug::init(uart_tx) };

    loop {
        print!("Start main loop iteration\n");

        restart_watchdog();

        led.set_output();
        sleep_ms(200, &mut nvic);
        led.clear_output();
        sleep_ms(800, &mut nvic);
    }
}
