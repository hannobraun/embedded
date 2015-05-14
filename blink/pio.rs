// Parallel I/O code for Atmel SAM3X8E.
// See Datasheet, chapter 31.

// TODO: Remove, once this module is moved into a library.
#![allow(dead_code, unused)]


// PIO registers. See Datasheet, chapter 31.7.
pub struct Pio {
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


pub const PIO_A: *mut Pio = 0x400E0E00 as *mut Pio;
pub const PIO_B: *mut Pio = 0x400E1000 as *mut Pio;
pub const PIO_C: *mut Pio = 0x400E1200 as *mut Pio;
pub const PIO_D: *mut Pio = 0x400E1400 as *mut Pio;
pub const PIO_E: *mut Pio = 0x400E1600 as *mut Pio;
pub const PIO_F: *mut Pio = 0x400E1800 as *mut Pio;


// Bit mask for PB27. This is pin 13 (the built-in LED) on the Arduino Due.
pub const PB27_MASK: u32 = 0x08000000;
