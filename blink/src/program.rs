use hardware::base::wdt::{
    WDT,
    Wdt,
};
use hardware::safe::pio;
use hardware::safe::rtt::Timer;


extern {
    fn WDT_Restart(wdt: *mut Wdt);
}


pub fn start() {
    // Pin 27 of the PIOB parallel I/O controller corresponds to pin 13 on the
    // Arduino Due, which is the built-in LED (labelled "L").
    let led = unsafe { pio::b().pin_27() };
    let led = led
        .enable()
        .enable_output();

    let timer = Timer::new();

    loop {
        unsafe { WDT_Restart(WDT) };

        led.set_output();
        timer.sleep_ms(200);
        led.clear_output();
        timer.sleep_ms(800);
    }
}
