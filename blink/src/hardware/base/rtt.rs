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


// Mode register flags. See data sheet, section 13.5.1.
pub const ALMIEN   : u32 = 0x1 << 16; // Alarm Interrupt Enable
pub const RTTINCIEN: u32 = 0x1 << 17; // RTT Increment Interrupt Enable
pub const RTTRST   : u32 = 0x1 << 18; // RTT Restart


pub const RTT: *mut Rtt = 0x400E1A30 as *mut Rtt;
