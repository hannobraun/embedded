// Real-time Timer code for Atmel SAM3X.
// See data sheet, chapter 13.


// Addresses of several registers used to control the real-time timer.
pub const TIMER_MODE_REGISTER : *mut   u32 = 0x400E1A30 as *mut   u32;
pub const TIMER_VALUE_REGISTER: *const u32 = 0x400E1A38 as *const u32;
