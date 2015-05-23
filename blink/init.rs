// Initialization code and entry point for the program.


#![feature(core, intrinsics, lang_items, no_std)]

#![no_main]
#![no_std]


extern crate core;


mod main;

mod pio;
mod rtt;

pub mod rust_base;


use core::prelude::*;


// This is the top of the stack, as provided to us by the linker.
extern {
	static _estack: u32;
}


// The data structure for the vector table. See data sheet, chapter 10.6.4.
pub struct VectorTable {
	pub initial_stack_pointer: &'static u32,

	pub on_reset: fn(),

	pub _reserved_1: [u32; 1],

	pub on_hard_fault             : fn(),
	pub on_memory_management_fault: fn(),
	pub on_bus_fault              : fn(),
	pub on_usage_fault            : fn(),

	pub _reserved_2: [u32; 4],

	pub on_sv_call: fn(),

	pub _reserved_3: [u32; 2],

	pub on_pend_sv: fn(),
	pub on_systick: fn(),

	pub on_irq0 : fn(),
	pub on_irq1 : fn(),
	pub on_irq2 : fn(),
	pub on_irq3 : fn(),
	pub on_irq4 : fn(),
	pub on_irq5 : fn(),
	pub on_irq6 : fn(),
	pub on_irq7 : fn(),
	pub on_irq8 : fn(),
	pub on_irq9 : fn(),
	pub on_irq10: fn(),
	pub on_irq11: fn(),
	pub on_irq12: fn(),
	pub on_irq13: fn(),
	pub on_irq14: fn(),
	pub on_irq15: fn(),
	pub on_irq16: fn(),
	pub on_irq17: fn(),
	pub on_irq18: fn(),
	pub on_irq19: fn(),
	pub on_irq20: fn(),
	pub on_irq21: fn(),
	pub on_irq22: fn(),
	pub on_irq23: fn(),
	pub on_irq24: fn(),
	pub on_irq25: fn(),
	pub on_irq26: fn(),
	pub on_irq27: fn(),
	pub on_irq28: fn(),
	pub on_irq29: fn(),
}

unsafe impl Sync for VectorTable {}


// The vector table. We're telling the compiler to place this into .vectors
// section, not where it would normally go (I suppose .rodata). The linker
// script makes sure that the .vectors section is at the right place.
#[link_section=".vectors"]
pub static VECTOR_TABLE: VectorTable = VectorTable {
	// TODO: Find out why this is &_estack and not just _estack. I was able to
	//       find documentation stating this fact, but offering no explanation.
	//       See http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0803b/CHDBIJJD.html
	initial_stack_pointer: &_estack,

	on_reset: on_reset,

	_reserved_1: [0; 1],

	on_hard_fault             : abort,
	on_memory_management_fault: abort,
	on_bus_fault              : abort,
	on_usage_fault            : abort,

	_reserved_2: [0; 4],

	on_sv_call: abort,

	_reserved_3: [0; 2],

	on_pend_sv: abort,
	on_systick: abort,

	on_irq0 : abort,
	on_irq1 : abort,
	on_irq2 : abort,
	on_irq3 : abort,
	on_irq4 : abort,
	on_irq5 : abort,
	on_irq6 : abort,
	on_irq7 : abort,
	on_irq8 : abort,
	on_irq9 : abort,
	on_irq10: abort,
	on_irq11: abort,
	on_irq12: abort,
	on_irq13: abort,
	on_irq14: abort,
	on_irq15: abort,
	on_irq16: abort,
	on_irq17: abort,
	on_irq18: abort,
	on_irq19: abort,
	on_irq20: abort,
	on_irq21: abort,
	on_irq22: abort,
	on_irq23: abort,
	on_irq24: abort,
	on_irq25: abort,
	on_irq26: abort,
	on_irq27: abort,
	on_irq28: abort,
	on_irq29: abort,
};


fn on_reset() {
	// TODO: This function doesn't copy the .relocate segment into RAM, as init
	//       code would normally do. We're getting away with this, because this
	//       program doesn't use any global variables (or more generally,
	//       doesn't have anything that would go into the .data section). Please
	//       be aware that what might be mistaken for global variables in this
	//       file are actually global constants, which go into the .rodata
	//       section. The problem is that if there were global variables, their
	//       initial value would not be set and the program would just fail
	//       silently. I see two solutions for this:
	//       1. Decide that we're never going to use global variables and remove
	//          support for the .relocate section from the linker script. I
	//          think if that were done, an attempted write to a global variable
	//          might fail the program outright, because the global variable
	//          would reside in ROM. This is just speculation, however, so more
	//          research is required before implementing this solution.
	//       2. Just copy the .relocate segment like any sane microcontroller
	//          program would do. This would definitely be a safe solution, and
	//          the only reason I'm not doing it right now is that it reeks of
	//          cargo cult. I'd rather be bitten from not doing it and gain a
	//          better have understanding of why I'm doing it afterwards, than
	//          just do it from the start without really understanding the
	//          reason.
	// TODO: This function doesn't initialize the .bss segment to zero, as init
	//       code would normally do. This doesn't make any difference right now,
	//       because there are no uninitialized global variables in this
	//       program. I'm wary of just doing it, for two reasons:
	//       1. I'm not sure why it needs to be done at all. According to my
	//          understanding, C doesn't guarantee that variables are
	//          initialized with any given value, so why should global variables
	//          be different?
	//       2. Even if there is a good reasons (as there probably is), I don't
	//          think global variables are such a hot idea, so I don't want to
	//          do anything that supports them, out of pure stubbornness.

	main::start()
}

// Used as a handler function for all interrupts we don't want to handle yet.
fn abort() {
	loop {}
}
