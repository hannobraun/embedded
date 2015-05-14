// Addresses of several registers used to control parallel I/O.
pub const PB_PIO_ENABLE       : *mut u32 = 0x400E1000 as *mut u32;
pub const PB_OUTPUT_ENABLE    : *mut u32 = 0x400E1010 as *mut u32;
pub const PB_SET_OUTPUT_DATA  : *mut u32 = 0x400E1030 as *mut u32;
pub const PB_CLEAR_OUTPUT_DATA: *mut u32 = 0x400E1034 as *mut u32;

// Bit mask for PB27. This is pin 13 (the built-in LED) on the Arduino Due.
pub const PB27_MASK: u32 = 0x08000000;
