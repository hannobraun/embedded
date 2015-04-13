#include <Arduino.h>

volatile int *pb_pio_enable          = (int *)0x400E1000;
volatile int *pb_output_enable       = (int *)0x400E1010;
volatile int *pb_set_output_data     = (int *)0x400E1030;
volatile int *pb_clear_output_data   = (int *)0x400E1034;

int pb27_mask = 0x08000000;

void setup() {
	*pb_pio_enable    = pb27_mask;
	*pb_output_enable = pb27_mask;
}

void loop() {
	*pb_set_output_data = pb27_mask;
	delay(200);
	*pb_clear_output_data = pb27_mask;
	delay(800);
}
