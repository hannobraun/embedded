// Real-time Timer code for Atmel SAM3X.
// See data sheet, chapter 13.


use volatile::Volatile;


// Real-time Timer user interface. See data sheet, chapter 13.5.
#[repr(C)]
pub struct Rtt {
	pub mode  : Volatile<u32>,
	pub alarm : Volatile<u32>,
	pub value : Volatile<u32>,
	pub status: Volatile<u32>,
}


pub const RTT: *mut Rtt = 0x400E1A30 as *mut Rtt;
