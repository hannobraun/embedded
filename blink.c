#include <Arduino.h>


// Parallel I/O
volatile int *pb_pio_enable          = (int *)0x400E1000;
volatile int *pb_output_enable       = (int *)0x400E1010;
volatile int *pb_set_output_data     = (int *)0x400E1030;
volatile int *pb_clear_output_data   = (int *)0x400E1034;

int pb27_mask = 0x08000000;


// Real-time Timer
volatile int *timer_mode_register  = (int *)0x400E1A30;
volatile int *timer_value_register = (int *)0x400E1A38;


void sleep_ms(int milliseconds) {
	int sleep_until = *timer_value_register + milliseconds;
	while (*timer_value_register < sleep_until) {}
}


void setup() {
	// Parallel I/O
	*pb_pio_enable    = pb27_mask;
	*pb_output_enable = pb27_mask;

	// Real-time Timer
	*timer_mode_register = 0x00000020;
}

void loop() {
	*pb_set_output_data = pb27_mask;
	sleep_ms(200);
	*pb_clear_output_data = pb27_mask;
	sleep_ms(800);
}
