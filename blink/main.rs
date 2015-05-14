#![feature(core, intrinsics, lang_items, no_std)]

#![no_main]
#![no_std]


extern crate core;

mod pio;


use core::prelude::*;

use pio::{
	P27,
	PIO_B,
};


// These are a few language items that are required by the core library. The
// core library is completely platform agnostic and doesn't assume anything
// (besides very basic things like a stack) about the platform it is running on.
// Therefore it can't know how to handle program panics and the like.

#[lang = "panic_fmt"]
pub extern fn panic_fmt() { loop {} }

#[lang = "stack_exhausted"]
pub extern fn stack_exhausted() { loop {} }

#[lang = "eh_personality"]
pub extern fn eh_personality() { loop {} }


// I'm not 100% sure what this function does, but references to it are compiled
// into the program by the Rust compiler. I think it would be called in the case
// of a program panic.
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() { loop {} }


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
	pub initial_stack_pointer_value: &'static u32,
	pub reset_handler              : fn(),

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
	initial_stack_pointer_value: &_estack,
	reset_handler              : start,
	other_interrupt_vectors    : [0; 44],
};


// Addresses of several registers used to control the real-time timer.
const TIMER_MODE_REGISTER : *mut   u32 = 0x400E1A30 as *mut   u32;
const TIMER_VALUE_REGISTER: *const u32 = 0x400E1A38 as *const u32;


// As the name suggests, this function sleeps for a given number of
// milliseconds. Our replacement for Arduino's delay function.
fn sleep_ms(milliseconds: u32) {
	unsafe {
		let sleep_until = *TIMER_VALUE_REGISTER + milliseconds;
		while *TIMER_VALUE_REGISTER < sleep_until {}
	}
}

// This function is the entry point for our application and the handler function
// for the reset interrupt.
fn start() {
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

	unsafe {
		// Enable pin 27 of PIO_B (pin 13 on the Arduino Due) and configure it
		// for output.
		(*PIO_B).pio_enable    = P27;
		(*PIO_B).output_enable = P27;

		// Set the timer to a resolution of a millisecond.
		*TIMER_MODE_REGISTER = 0x00000020;

		// Continuously set and clear output on pin 27 of PIO_B (pin 13 on the
		// Arduino Due). This blinks the Due's built-in LED, which is the single
		// purpose of this program.
		loop {
			(*PIO_B).set_output_data = P27;
			sleep_ms(200);
			(*PIO_B).clear_output_data = P27;
			sleep_ms(800);
		}
	}
}
