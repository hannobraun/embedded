// Parallel I/O code for Atmel SAM3X.
// See Datasheet, chapter 31.

// TODO: Remove, once this module is moved into a library.
#![allow(dead_code, unused)]


// PIO registers. See Datasheet, chapter 31.7.
// TODO: I'm not sure what guarantees the compiler makes about the memory layout
//       of structs. Do I have to use #[repr(C)] or something similar?
pub struct Controller {
	pub pio_enable : u32,
	pub pio_disable: u32,
	pub pio_status : u32,

	pub _reserved_1: u32,

	pub output_enable : u32,
	pub output_disable: u32,
	pub output_status : u32,

	pub _reserved_2: u32,

	pub glitch_input_filter_enable : u32,
	pub glitch_input_filter_disable: u32,
	pub glitch_input_filter_status : u32,

	pub _reserved_3: u32,

	pub set_output_data   : u32,
	pub clear_output_data : u32,
	pub output_data_status: u32,
	pub pin_data_status   : u32,

	pub interrupt_enable : u32,
	pub interrupt_disable: u32,
	pub interrupt_mask   : u32,
	pub interrupt_status : u32,

	pub multi_driver_enable : u32,
	pub multi_driver_disable: u32,
	pub multi_driver_status : u32,

	pub _reserved_4: u32,

	pub pull_up_disable   : u32,
	pub pull_up_enable    : u32,
	pub pad_pull_up_status: u32,

	pub _reserved_5: u32,

	pub peripheral_ab_select: u32,

	pub _reserved_6: [u32; 3],

	pub system_clock_glitch_input_filter_select                 : u32,
	pub debouncing_input_filter_select                          : u32,
	pub glitch_or_debouncing_input_filter_clock_selection_status: u32,
	pub slow_clock_divider_debouncing                           : u32,

	pub _reserved_7: [u32; 4],

	pub output_write_enable : u32,
	pub output_write_disable: u32,
	pub output_write_status : u32,

	pub _reserved_8: u32,

	pub additional_interrupt_modes_enable : u32,
	pub additional_interrupt_modes_disable: u32,
	pub additional_interrupt_modes_mask   : u32,

	pub _reserved_9: u32,

	pub edge_select      : u32,
	pub level_select     : u32,
	pub edge_level_status: u32,

	pub _reserved_a: u32,

	pub falling_edge_low_level_select: u32,
	pub rising_edge_high_level_select: u32,
	pub fall_rise_low_high_status    : u32,

	pub _reserved_b: u32,

	pub lock_status         : u32,
	pub write_protect_mode  : u32,
	pub write_protect_status: u32,
}


// Addresses of the PIO controllers. See chapters 31.7 and 31.7.1.
pub const PIO_A: *mut Controller = 0x400E0E00 as *mut Controller;
pub const PIO_B: *mut Controller = 0x400E1000 as *mut Controller;
pub const PIO_C: *mut Controller = 0x400E1200 as *mut Controller;
pub const PIO_D: *mut Controller = 0x400E1400 as *mut Controller;
pub const PIO_E: *mut Controller = 0x400E1600 as *mut Controller;
pub const PIO_F: *mut Controller = 0x400E1800 as *mut Controller;


// Bit flags for the various I/O pins of each controller. This is described in
// various chapters, e.g. chapter 31.7.1.
pub const P0 : u32 = 0x00000001;
pub const P1 : u32 = 0x00000002;
pub const P2 : u32 = 0x00000004;
pub const P3 : u32 = 0x00000008;
pub const P4 : u32 = 0x00000010;
pub const P5 : u32 = 0x00000020;
pub const P6 : u32 = 0x00000040;
pub const P7 : u32 = 0x00000080;
pub const P8 : u32 = 0x00000100;
pub const P9 : u32 = 0x00000200;
pub const P10: u32 = 0x00000400;
pub const P11: u32 = 0x00000800;
pub const P12: u32 = 0x00001000;
pub const P13: u32 = 0x00002000;
pub const P14: u32 = 0x00004000;
pub const P15: u32 = 0x00008000;
pub const P16: u32 = 0x00010000;
pub const P17: u32 = 0x00020000;
pub const P18: u32 = 0x00040000;
pub const P19: u32 = 0x00080000;
pub const P20: u32 = 0x00100000;
pub const P21: u32 = 0x00200000;
pub const P22: u32 = 0x00400000;
pub const P23: u32 = 0x00800000;
pub const P24: u32 = 0x01000000;
pub const P25: u32 = 0x02000000;
pub const P26: u32 = 0x04000000;
pub const P27: u32 = 0x08000000;
pub const P28: u32 = 0x10000000;
pub const P29: u32 = 0x20000000;
pub const P30: u32 = 0x40000000;
pub const P31: u32 = 0x80000000;
