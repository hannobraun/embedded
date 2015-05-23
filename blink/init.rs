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


// This is a partial definition of the vector table. It only defines the first
// two entries which, as far as I can tell, are the minimum needed for a program
// to work at all.
// Space for the other interrupt handlers is reserved. I'm not sure if this is
// necessary, but I can imagine that the vector table not having the right
// length could cause all kinds of problems (imagine if it was too short, and
// the linker would place something else directly after it).
pub struct VectorTable {
	pub initial_stack_pointer: &'static u32,
	pub on_reset             : fn(),

	pub other_interrupt_vectors: [u32; 44],
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
	initial_stack_pointer  : &_estack,
	on_reset               : on_reset,
	other_interrupt_vectors: [0; 44],
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
	//          cargo cult. I'd rather be bitten from not doing it and then have
	//          a good understanding of why I'm doing it afterwards, than just
	//          do it from the start without really understanding the reason.
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
