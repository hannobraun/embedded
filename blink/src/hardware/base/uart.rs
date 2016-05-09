// Universal Asynchronous Receiver/Transmitter code for Atmel SAM3X.
// See data sheet, chapter 34.


use volatile::Volatile;

use super::pdc::Pdc;


/// UART user interface. See data sheet, chapter 34.6.
#[repr(C)]
pub struct Uart {
    pub control            : Volatile<u32>,
    pub mode               : Volatile<u32>,
    pub interrupt_enable   : Volatile<u32>,
    pub interrupt_disable  : Volatile<u32>,
    pub interrupt_mask     : Volatile<u32>,
    pub status             : Volatile<u32>,
    pub receive_holding    : Volatile<u32>,
    pub transmit_holding   : Volatile<u32>,
    pub baud_rate_generator: Volatile<u32>,

    _reserved: [Volatile<u32>; 55],

    pub pdc: Pdc,
}


// Control register bits. See data sheet, chapter 34.6.1.
pub const RSTRX : u32 = 0x1 << 2; // Reset Receiver
pub const RSTTX : u32 = 0x1 << 3; // Reset Transmitter
pub const RXEN  : u32 = 0x1 << 4; // Receiver Enable
pub const RXDIS : u32 = 0x1 << 5; // Receiver Disable
pub const TXEN  : u32 = 0x1 << 6; // Transmitter Enable
pub const TXDIS : u32 = 0x1 << 7; // Transmitter Disable
pub const RSTSTA: u32 = 0x1 << 8; // Reset Status Bits


// Parity configuration, to be written into the mode register. See data sheet,
// chapter 34.6.2.
pub const PARITY_EVEN : u32 = 0x0 << 9;
pub const PARITY_ODD  : u32 = 0x1 << 9;
pub const PARITY_SPACE: u32 = 0x2 << 9; // parity forced to 0
pub const PARITY_MARK : u32 = 0x3 << 9; // parity forced to 1
pub const PARITY_NO   : u32 = 0x4 << 9;


// UART modes, to be written into the mode register. See data sheet, chapter
// 34.6.2.
pub const MODE_NORMAL         : u32 = 0x0 << 14;
pub const MODE_AUTOMATIC_ECHO : u32 = 0x1 << 14;
pub const MODE_LOCAL_LOOPBACK : u32 = 0x2 << 14;
pub const MODE_REMOTE_LOOPBACK: u32 = 0x3 << 14;


pub const UART: *mut Uart = 0x400E0800 as *mut Uart;
