// Addresses of several registers used to control parallel I/O.
static volatile int * const pb_pio_enable          = (int *)0x400E1000;
static volatile int * const pb_output_enable       = (int *)0x400E1010;
static volatile int * const pb_set_output_data     = (int *)0x400E1030;
static volatile int * const pb_clear_output_data   = (int *)0x400E1034;

// Bit mask for PB27. This is pin 13 (the built-in LED) on the Arduino Due.
static const int pb27_mask = 0x08000000;

// Addresses of several registers used to control the real-time timer.
static volatile int * const timer_mode_register  = (int *)0x400E1A30;
static volatile int * const timer_value_register = (int *)0x400E1A38;


// As the name suggests, this function sleeps for a given number of
// milliseconds. Our replacement for Arduino's delay function.
void sleep_ms(int milliseconds) {
	int sleep_until = *timer_value_register + milliseconds;
	while (*timer_value_register < sleep_until) {}
}

// This function is the entry point for our application and the handler function
// for the reset interrupt.
void start() {
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

	// Enable PB27 (pin 13) and configure it for output.
	*pb_pio_enable    = pb27_mask;
	*pb_output_enable = pb27_mask;

	// Set the timer to a resolution of a millisecond.
	*timer_mode_register = 0x00000020;

	// Continuously set and clear output on PB27 (pin 13). This blinks the Due's
	// built-in LED, which is the single purpose of this program.
	while (1) {
		*pb_set_output_data = pb27_mask;
		sleep_ms(200);
		*pb_clear_output_data = pb27_mask;
		sleep_ms(800);
	}
}
