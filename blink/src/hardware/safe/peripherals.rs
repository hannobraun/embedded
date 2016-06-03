use hardware::base::peripherals;


#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Peripheral {
    Supc      = peripherals::SUPC,
    Rstc      = peripherals::RSTC,
    Rtc       = peripherals::RTC,
    Rtt       = peripherals::RTT,
    Wdt       = peripherals::WDT,
    Pmc       = peripherals::PMC,
    Eefc0     = peripherals::EEFC0,
    Eefc1     = peripherals::EEFC1,
    Uart      = peripherals::UART,
    SmcSdramc = peripherals::SMC_SDRAMC,
    Sdramc    = peripherals::SDRAMC,
    PioA      = peripherals::PIOA,
    PioB      = peripherals::PIOB,
    PioC      = peripherals::PIOC,
    PioD      = peripherals::PIOD,
    PioE      = peripherals::PIOE,
    PioF      = peripherals::PIOF,
    Usart0    = peripherals::USART0,
    Usart1    = peripherals::USART1,
    Usart2    = peripherals::USART2,
    Usart3    = peripherals::USART3,
    Hsmci     = peripherals::HSMCI,
    Twi0      = peripherals::TWI0,
    Twi1      = peripherals::TWI1,
    Spi0      = peripherals::SPI0,
    Spi1      = peripherals::SPI1,
    Ssc       = peripherals::SSC,
    Tc0       = peripherals::TC0,
    Tc1       = peripherals::TC1,
    Tc2       = peripherals::TC2,
    Tc3       = peripherals::TC3,
    Tc4       = peripherals::TC4,
    Tc5       = peripherals::TC5,
    Tc6       = peripherals::TC6,
    Tc7       = peripherals::TC7,
    Tc8       = peripherals::TC8,
    Pwm       = peripherals::PWM,
    Adc       = peripherals::ADC,
    Dacc      = peripherals::DACC,
    Dmac      = peripherals::DMAC,
    UtogHs    = peripherals::UOTGHS,
    Trng      = peripherals::TRNG,
    Emac      = peripherals::EMAC,
    Can0      = peripherals::CAN0,
    Can1      = peripherals::CAN1,
}

impl Peripheral {
    pub fn id(&self) -> u32 {
        *self as u32
    }

    pub fn mask(&self) -> u32 {
        0x1 << self.id() % 32
    }
}
