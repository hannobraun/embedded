// Power Management Controller code for Atmel SAM3X.
// See data sheet, chapter 28.


use volatile::Volatile;


/// Power Management Controller user interface. See data sheet, chapter 28.15.
#[repr(C)]
pub struct Pmc {
    pub system_clock_enable : Volatile<u32>,
    pub system_clock_disable: Volatile<u32>,
    pub system_clock_status : Volatile<u32>,

    pub _reserved_1: Volatile<u32>,

    pub peripheral_clock_enable_0 : Volatile<u32>,
    pub peripheral_clock_disable_0: Volatile<u32>,
    pub peripheral_clock_status_0 : Volatile<u32>,

    pub utmi_clock          : Volatile<u32>,
    pub main_oscillator     : Volatile<u32>,
    pub main_clock_frequency: Volatile<u32>,
    pub plla                : Volatile<u32>,

    pub _reserved_2: Volatile<u32>,

    pub master_clock: Volatile<u32>,

    pub _reserved_3: Volatile<u32>,

    pub usb_clock: Volatile<u32>,

    pub _reserved_4: Volatile<u32>,

    pub programmable_clock_0: Volatile<u32>,
    pub programmable_clock_1: Volatile<u32>,
    pub programmable_clock_2: Volatile<u32>,

    pub _reserved_5: [Volatile<u32>; 5],

    pub interrupt_enable : Volatile<u32>,
    pub interrupt_disable: Volatile<u32>,

    pub status: Volatile<u32>,

    pub interrupt_mask: Volatile<u32>,

    pub fast_startup_mode    : Volatile<u32>,
    pub fast_startup_polarity: Volatile<u32>,

    pub fault_output_clear: Volatile<u32>,

    pub _reserved_6: [Volatile<u32>; 26],

    pub write_protect_mode  : Volatile<u32>,
    pub write_protect_status: Volatile<u32>,

    pub _reserved_7: [Volatile<u32>; 5],

    pub peripheral_clock_enable_1 : Volatile<u32>,
    pub peripheral_clock_disable_1: Volatile<u32>,
    pub peripheral_clock_status_1 : Volatile<u32>,

    pub peripheral_control: Volatile<u32>,
}


pub const SLOW_CLOCK_FREQUENCY_HZ: u32 = 32_768;

// Constants for Clock Generator Main Clock Frequency Register. See data sheet,
// section 28.15.9.
pub const MAINFRDY  : u32 = 0x00010000;
pub const MAINF_MASK: u32 = 0x0000ffff;


pub const PMC: *mut Pmc = 0x400E0600 as *mut Pmc;
