// Peripheral identifiers for Atmel SAM3X.
// See data sheet, chapter 9.


pub const SUPC      : u32 = 0x1 << 0 ; // Supply Controller
pub const RSTC      : u32 = 0x1 << 1 ; // Reset Controller
pub const RTC       : u32 = 0x1 << 2 ; // Real-time Clock
pub const RTT       : u32 = 0x1 << 3 ; // Real-time Timer
pub const WDT       : u32 = 0x1 << 4 ; // Watchdog Timer
pub const PMC       : u32 = 0x1 << 5 ; // Power Management Controller
pub const EEFC0     : u32 = 0x1 << 6 ; // Enhanced Embedded Flash Controller 0
pub const EEFC1     : u32 = 0x1 << 7 ; // Enhanced Embedded Flash Controller 1
pub const UART      : u32 = 0x1 << 8 ; // Universal Asynchronous Receiver
                                       // Transceiver
pub const SMC_SDRAMC: u32 = 0x1 << 9 ; // Static Memory Controller /
                                       // Synchronous Dynamic RAM Controller
pub const SDRAMC    : u32 = 0x1 << 10; // Synchronous Dynamic RAM Controller
pub const PIOA      : u32 = 0x1 << 11; // Parallel I/O Controller A
pub const PIOB      : u32 = 0x1 << 12; // Parallel I/O Controller B
pub const PIOC      : u32 = 0x1 << 13; // Parallel I/O Controller C
pub const PIOD      : u32 = 0x1 << 14; // Parallel I/O Controller D
pub const PIOE      : u32 = 0x1 << 15; // Parallel I/O Controller E
pub const PIOF      : u32 = 0x1 << 16; // Parallel I/O Controller F
pub const USART0    : u32 = 0x1 << 17; // Universal Synchronous Asynchronous
                                       // Receiver Transmitter 0
pub const USART1    : u32 = 0x1 << 18; // Universal Synchronous Asynchronous
                                       // Receiver Transmitter 1
pub const USART2    : u32 = 0x1 << 19; // Universal Synchronous Asynchronous
                                       // Receiver Transmitter 2
pub const USART3    : u32 = 0x1 << 20; // Universal Synchronous Asynchronous
                                       // Receiver Transmitter 3
pub const HSMCI     : u32 = 0x1 << 21; // High Speed Multimedia Card Interface
pub const TWI0      : u32 = 0x1 << 22; // Two-Wire Interface 0
pub const TWI1      : u32 = 0x1 << 23; // Two-Wire Interface 1
pub const SPI0      : u32 = 0x1 << 24; // Serial Peripheral Interface 0
pub const SPI1      : u32 = 0x1 << 25; // Serial Peripheral Interface 1
pub const SSC       : u32 = 0x1 << 26; // Synchronous Serial Controller
pub const TC0       : u32 = 0x1 << 27; // Timer Counter Channel 0
pub const TC1       : u32 = 0x1 << 28; // Timer Counter Channel 1
pub const TC2       : u32 = 0x1 << 29; // Timer Counter Channel 2
pub const TC3       : u32 = 0x1 << 30; // Timer Counter Channel 3
pub const TC4       : u32 = 0x1 << 31; // Timer Counter Channel 4

pub const TC5   : u32 = 0x1 << 0 ; // Timer Counter Channel 5
pub const TC6   : u32 = 0x1 << 1 ; // Timer Counter Channel 6
pub const TC7   : u32 = 0x1 << 2 ; // Timer Counter Channel 7
pub const TC8   : u32 = 0x1 << 3 ; // Timer Counter Channel 8
pub const PWM   : u32 = 0x1 << 4 ; // Pulse Width Modulation Controller
pub const ADC   : u32 = 0x1 << 5 ; // ADC Controller
pub const DACC  : u32 = 0x1 << 6 ; // DAC Controller
pub const DMAC  : u32 = 0x1 << 7 ; // DMA Controller
pub const UOTGHS: u32 = 0x1 << 8 ; // USB OTG High Speed
pub const TRNG  : u32 = 0x1 << 9 ; // True Random Number Generator
pub const EMAC  : u32 = 0x1 << 10; // Ethernet MAC
pub const CAN0  : u32 = 0x1 << 11; // CAN Controller 0
pub const CAN1  : u32 = 0x1 << 12; // CAN Controller 1
