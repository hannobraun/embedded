use hardware::safe::nvic::Nvic;
use hardware::safe::pio;
use hardware::safe::rtt::Timer;
use hardware::safe::wdt::restart_watchdog;


pub fn start() {
    let mut nvic = unsafe { Nvic::new() };
    nvic.enable_rtt();

    // Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
    // Arduino Due, which is the built-in LED (labelled "L").
    let led = unsafe { pio::b().pin_27() };
    let led = led
        .enable()
        .enable_output();

    let timer = unsafe { Timer::new() };

    loop {
        restart_watchdog();

        led.set_output();
        timer.sleep_ms(200);
        led.clear_output();
        timer.sleep_ms(800);
    }
}
