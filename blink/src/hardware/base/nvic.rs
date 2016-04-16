// Nested Vector Interrupt Controller code for Atmel SAM3X.
// See data sheet, chapters 10.20 and 9.1.


use volatile::Volatile;


pub type Reg = Volatile<u32>;


pub const ISER0: *mut Reg = 0xE000E100 as *mut Reg;
pub const ISER1: *mut Reg = 0xE000E104 as *mut Reg;

pub const ICER0: *mut Reg = 0xE000E180 as *mut Reg;
pub const ICER1: *mut Reg = 0xE000E184 as *mut Reg;

pub const ISPR0: *mut Reg = 0xE000E200 as *mut Reg;
pub const ISPR1: *mut Reg = 0xE000E204 as *mut Reg;

pub const ICPR0: *mut Reg = 0xE000E280 as *mut Reg;
pub const ICPR1: *mut Reg = 0xE000E284 as *mut Reg;

pub const IABR0: *mut Reg = 0xE000E300 as *mut Reg;
pub const IABR1: *mut Reg = 0xE000E304 as *mut Reg;

pub const IPR0: *mut Reg = 0xE000E400 as *mut Reg;
pub const IPR1: *mut Reg = 0xE000E404 as *mut Reg;
pub const IPR2: *mut Reg = 0xE000E408 as *mut Reg;
pub const IPR3: *mut Reg = 0xE000E40C as *mut Reg;
pub const IPR4: *mut Reg = 0xE000E410 as *mut Reg;
pub const IPR5: *mut Reg = 0xE000E414 as *mut Reg;
pub const IPR6: *mut Reg = 0xE000E418 as *mut Reg;
pub const IPR7: *mut Reg = 0xE000E41C as *mut Reg;

pub const STIR: *mut Reg = 0xE000EF00 as *mut Reg;


// Bit masks for the various system peripherals. The following masks can be
// used with `ISER0`, `ICER0`, `ISPR0`, `ICPR0`, and `IABR0`.
pub const SUPC      : u32 = 0x00000001; // Supply Controller
pub const RSTC      : u32 = 0x00000002; // Reset Controller
pub const RTC       : u32 = 0x00000004; // Real-time Clock
pub const RTT       : u32 = 0x00000008; // Real-time Timer
pub const WDG       : u32 = 0x00000010; // Watchdog Timer
pub const PMC       : u32 = 0x00000020; // Power Management Controller
pub const EEFC0     : u32 = 0x00000040; // Enhanced Embedded Flash Controller 0
pub const EEFC1     : u32 = 0x00000080; // Enhanced Embedded Flash Controller 1
pub const UART      : u32 = 0x00000100; // Universal Asynchronous Receiver
                                        // Transceiver
pub const SMC_SDRAMC: u32 = 0x00000200; // Static Memory Controller /
                                        // Synchronous Dynamic RAM Controller
pub const SDRAMC    : u32 = 0x00000400; // Synchronous Dynamic RAM Controller
pub const PIOA      : u32 = 0x00000800; // Parallel I/O Controller A
pub const PIOB      : u32 = 0x00001000; // Parallel I/O Controller B
pub const PIOC      : u32 = 0x00002000; // Parallel I/O Controller C
pub const PIOD      : u32 = 0x00004000; // Parallel I/O Controller D
pub const PIOE      : u32 = 0x00008000; // Parallel I/O Controller E
pub const PIOF      : u32 = 0x00010000; // Parallel I/O Controller F
pub const USART0    : u32 = 0x00020000; // Universal Synchronous Asynchronous
                                        // Receiver Transmitter 0
pub const USART1    : u32 = 0x00040000; // Universal Synchronous Asynchronous
                                        // Receiver Transmitter 1
pub const USART2    : u32 = 0x00080000; // Universal Synchronous Asynchronous
                                        // Receiver Transmitter 2
pub const USART3    : u32 = 0x00100000; // Universal Synchronous Asynchronous
                                        // Receiver Transmitter 3
pub const HSMCI     : u32 = 0x00200000; // High Speed Multimedia Card Interface
pub const TWI0      : u32 = 0x00400000; // Two-Wire Interface 0
pub const TWI1      : u32 = 0x00800000; // Two-Wire Interface 1
pub const SPI0      : u32 = 0x01000000; // Serial Peripheral Interface 0
pub const SPI1      : u32 = 0x02000000; // Serial Peripheral Interface 1
pub const SSC       : u32 = 0x04000000; // Synchronous Serial Controller
pub const TC0       : u32 = 0x08000000; // Timer Counter Channel 0
pub const TC1       : u32 = 0x10000000; // Timer Counter Channel 1
pub const TC2       : u32 = 0x20000000; // Timer Counter Channel 2
pub const TC3       : u32 = 0x40000000; // Timer Counter Channel 3
pub const TC4       : u32 = 0x80000000; // Timer Counter Channel 4

// Bit masks for the various system peripherals. The following masks can be
// used with `ISER1`, `ICER1`, `ISPR1`, `ICPR1`, and `IABR1`.
pub const TC5   : u32 = 0x00000001; // Timer Counter Channel 5
pub const TC6   : u32 = 0x00000002; // Timer Counter Channel 6
pub const TC7   : u32 = 0x00000004; // Timer Counter Channel 7
pub const TC8   : u32 = 0x00000008; // Timer Counter Channel 8
pub const PWM   : u32 = 0x00000010; // Pulse Width Modulation Controller
pub const ADC   : u32 = 0x00000020; // ADC Controller
pub const DACC  : u32 = 0x00000040; // DAC Controller
pub const DMAC  : u32 = 0x00000080; // DMA Controller
pub const UOTGHS: u32 = 0x00000100; // USB OTG High Speed
pub const TRNG  : u32 = 0x00000200; // True Random Number Generator
pub const EMAC  : u32 = 0x00000400; // Ethernet MAC
pub const CAN0  : u32 = 0x00000800; // CAN Controller 0
pub const CAN1  : u32 = 0x00001000; // CAN Controller 1
