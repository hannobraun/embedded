// Nested Vector Interrupt Controller code for Atmel SAM3X.
// See data sheet, chapters 10.20 and 9.1.


use volatile::Volatile;


pub type Reg = Volatile<u32>;


pub const ISER: *mut [Reg; 2] = 0xE000E100 as *mut [Reg; 2];
pub const ICER: *mut [Reg; 2] = 0xE000E180 as *mut [Reg; 2];
pub const ISPR: *mut [Reg; 2] = 0xE000E200 as *mut [Reg; 2];
pub const ICPR: *mut [Reg; 2] = 0xE000E280 as *mut [Reg; 2];
pub const IABR: *mut [Reg; 2] = 0xE000E300 as *mut [Reg; 2];
pub const IPR : *mut [Reg; 8] = 0xE000E400 as *mut [Reg; 8];
pub const STIR: *mut Reg      = 0xE000EF00 as *mut Reg;
