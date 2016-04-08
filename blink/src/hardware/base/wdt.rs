/// Watchdog Timer interface for Atmel SAM3X.
/// See Datasheet, chapter 15.


use volatile::Volatile;


/// User interface for the Watchdog Timer. See Datasheet, chapter 15.5.
#[repr(C)]
pub struct Wdt {
    pub control: Volatile<u32>,
    pub mode   : Volatile<u32>,
    pub status : Volatile<u32>,
}


pub const WDT: *mut Wdt = 0x400E1A50 as *mut Wdt;
