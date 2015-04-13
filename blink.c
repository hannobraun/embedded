// Addresses of several registers used to control parallel I/O.
volatile int *pb_pio_enable          = (int *)0x400E1000;
volatile int *pb_output_enable       = (int *)0x400E1010;
volatile int *pb_set_output_data     = (int *)0x400E1030;
volatile int *pb_clear_output_data   = (int *)0x400E1034;

// Bit mask for PB27. This is pin 13 (the built-in LED) on the Arduino Due.
int pb27_mask = 0x08000000;

// Addresses of several registers used to control the real-time timer.
volatile int *timer_mode_register  = (int *)0x400E1A30;
volatile int *timer_value_register = (int *)0x400E1A38;


// As the name suggests, this function sleeps for a given number of
// milliseconds. Our replacement for Arduino's delay function.
void sleep_ms(int milliseconds) {
	int sleep_until = *timer_value_register + milliseconds;
	while (*timer_value_register < sleep_until) {}
}

// The main function. A normal Arduino sketch would have setup and loop
// functions, which are normally called by Arduino's built-in main function. Our
// main here replaces all three of these.
int main() {
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
