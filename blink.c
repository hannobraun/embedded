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

void start() {
	Reset_Handler();
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


// Those are interrupt handlers. They are set up by remaining Arduino code
// during initialization. All of these just halt execution.
void        NMI_Handler() { while (1) {} }
void  HardFault_Handler() { while (1) {} }
void  MemManage_Handler() { while (1) {} }
void   BusFault_Handler() { while (1) {} }
void UsageFault_Handler() { while (1) {} }
void        SVC_Handler() { while (1) {} }
void   DebugMon_Handler() { while (1) {} }
void     PendSV_Handler() { while (1) {} }
void    SysTick_Handler() { while (1) {} }
void       SUPC_Handler() { while (1) {} }
void       RSTC_Handler() { while (1) {} }
void        RTC_Handler() { while (1) {} }
void        RTT_Handler() { while (1) {} }
void        WDT_Handler() { while (1) {} }
void        PMC_Handler() { while (1) {} }
void       EFC0_Handler() { while (1) {} }
void       EFC1_Handler() { while (1) {} }
void       UART_Handler() { while (1) {} }
void        SMC_Handler() { while (1) {} }
void       PIOA_Handler() { while (1) {} }
void       PIOB_Handler() { while (1) {} }
void       PIOC_Handler() { while (1) {} }
void       PIOD_Handler() { while (1) {} }
void     USART0_Handler() { while (1) {} }
void     USART1_Handler() { while (1) {} }
void     USART2_Handler() { while (1) {} }
void     USART3_Handler() { while (1) {} }
void      HSMCI_Handler() { while (1) {} }
void       TWI0_Handler() { while (1) {} }
void       TWI1_Handler() { while (1) {} }
void       SPI0_Handler() { while (1) {} }
void        SSC_Handler() { while (1) {} }
void        TC0_Handler() { while (1) {} }
void        TC1_Handler() { while (1) {} }
void        TC2_Handler() { while (1) {} }
void        TC3_Handler() { while (1) {} }
void        TC4_Handler() { while (1) {} }
void        TC5_Handler() { while (1) {} }
void        TC6_Handler() { while (1) {} }
void        TC7_Handler() { while (1) {} }
void        TC8_Handler() { while (1) {} }
void        PWM_Handler() { while (1) {} }
void        ADC_Handler() { while (1) {} }
void       DACC_Handler() { while (1) {} }
void       DMAC_Handler() { while (1) {} }
void       TRNG_Handler() { while (1) {} }
void       EMAC_Handler() { while (1) {} }
void       CAN0_Handler() { while (1) {} }
void       CAN1_Handler() { while (1) {} }
void     UOTGHS_Handler() { while (1) {} }
