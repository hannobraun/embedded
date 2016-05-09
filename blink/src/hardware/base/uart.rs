// Universal Asynchronous Receiver Transceiver code for Atmel SAM3X.
// See data sheet, chapter 34.


use volatile::Volatile;

use super::pdc::Pdc;


/// UART user interface. See data sheet, chapter 34.6.
#[repr(C)]
pub struct Uart {
    pub control: Volatile<u32>,
    pub mode   : Volatile<u32>,
    pub interrupt_enable: Volatile<u32>,
    pub interrupt_disable: Volatile<u32>,
    pub interrupt_mask   : Volatile<u32>,
    pub status           : Volatile<u32>,
    pub receive_holding  : Volatile<u32>,
    pub transmit_holding : Volatile<u32>,
    pub baud_rate_generator: Volatile<u32>,

    _reserved: [Volatile<u32>; 55],

    pub pdc: Pdc,
}


pub const UART: *mut Uart = 0x400E0800 as *mut Uart;
