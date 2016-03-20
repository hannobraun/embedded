use hardware::safe::pio;
use hardware::safe::rtt::Timer;


pub fn start() {
    // Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
    // Arduino Due, which is the built-in LED (labelled "L").
    let led = unsafe { pio::b().pin_27() };
    let led = led
        .enable()
        .enable_output();

    let timer = Timer::new();

    // TODO: Since we're not doing anything about the watchdog, the program is
    //       being restarted every 17-18 seconds. This messes up our nice
    //       blinking pattern.
    loop {
        led.set_output();
        timer.sleep_ms(200);
        led.clear_output();
        timer.sleep_ms(800);
    }
}
