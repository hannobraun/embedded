// Nested Vector Interrupt Controller code for Atmel SAM3X.
// See data sheet, chapter 10.20.


use volatile::Volatile;


pub type Reg = Volatile<u32>;


pub const ISER0: *mut Reg = 0xE000E100 as *mut Reg;
pub const ISER1: *mut Reg = 0xE000E104 as *mut Reg;

pub const ICER0: *mut Reg = 0xE000E180 as *mut Reg;
pub const ICER1: *mut Reg = 0xE000E184 as *mut Reg;

pub const ISPR0: *mut Reg = 0xE000E200 as *mut Reg;
pub const ISPR1: *mut Reg = 0xE000E204 as *mut Reg;

pub const ICPR0: *mut Reg = 0xE000E280 as *mut Reg;
pub const ICPR1: *mut Reg = 0xE000E284 as *mut Reg;

pub const IABR0: *mut Reg = 0xE000E300 as *mut Reg;
pub const IABR1: *mut Reg = 0xE000E304 as *mut Reg;

pub const IPR0: *mut Reg = 0xE000E400 as *mut Reg;
pub const IPR1: *mut Reg = 0xE000E404 as *mut Reg;
pub const IPR2: *mut Reg = 0xE000E408 as *mut Reg;
pub const IPR3: *mut Reg = 0xE000E40C as *mut Reg;
pub const IPR4: *mut Reg = 0xE000E410 as *mut Reg;
pub const IPR5: *mut Reg = 0xE000E414 as *mut Reg;
pub const IPR6: *mut Reg = 0xE000E418 as *mut Reg;
pub const IPR7: *mut Reg = 0xE000E41C as *mut Reg;

pub const STIR: *mut Reg = 0xE000EF00 as *mut Reg;
