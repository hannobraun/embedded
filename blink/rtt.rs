// Real-time Timer code for Atmel SAM3X.
// See data sheet, chapter 13.


// Real-time Timer user interface. See data sheet, chapter 13.5.
// TODO: I'm not sure what guarantees the compiler makes about the memory layout
//       of structs. Do I have to use #[repr(C)] or something similar?
pub struct Rtt {
	pub mode  : u32,
	pub alarm : u32,
	pub value : u32,
	pub status: u32,
}


pub const RTT: *mut Rtt = 0x400E1A30 as *mut Rtt;
