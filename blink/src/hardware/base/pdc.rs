// Peripheral DMA Controller code for Atmel SAM3X.
// See data sheet, chapter 26.


use volatile::Volatile;


/// PDC user interface. See data sheet, chapter 26.5.
#[repr(C)]
pub struct Pdc {
    pub receive_pointer      : Volatile<u32>,
    pub receive_counter      : Volatile<u32>,
    pub transmit_pointer     : Volatile<u32>,
    pub transmit_counter     : Volatile<u32>,
    pub receive_next_pointer : Volatile<u32>,
    pub receive_next_counter : Volatile<u32>,
    pub transmit_next_pointer: Volatile<u32>,
    pub transmit_next_counter: Volatile<u32>,
    pub transfer_control     : Volatile<u32>,
    pub transfer_status      : Volatile<u32>,
}
