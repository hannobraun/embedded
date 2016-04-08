use hardware::base::wdt::WDT;


pub fn restart_watchdog() {
    unsafe {
        (*WDT).control.write(0xA5000001);
    }
}
