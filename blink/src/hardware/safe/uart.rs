use hardware::base::uart::{
    self,
    UART,
};
use hardware::safe::peripherals::Peripheral;
use hardware::safe::pio::{
    output_status,
    status,
    Pin,
};
use hardware::safe::pmc;


pub type UndefinedPin   = Pin<status::Undefined, output_status::Undefined>;
pub type InitializedPin = Pin<status::Disabled , output_status::Undefined>;


pub struct Uart {
    _tx_pin: InitializedPin,
}

impl Uart {
    /// Create a new `Uart` instance. There must only be one `Uart` instance at
    /// a time. Otherwise, the different instances might overwrite each others
    /// configuration.
    ///
    /// The argument `tx_pin` must be the specific pin that can be used by UART
    /// for sending data. One SAM3X8E this is pin 9 of PIO controller A. See
    /// data sheet, section 34.4.1.
    pub unsafe fn new(tx_pin: UndefinedPin) -> Self {
        let mut tx_pin = tx_pin
            // Disable UART pins to give control to the UART peripheral. See
            // data sheet, section 31.7.2.
            .disable()
            // Enable pull-up resistor for UART pins. See data sheet, sections
            // 31.5.1 and 31.7.22.
            .enable_pull_up();

        // Switch UART pins to be controlled by peripheral A, which is UART.
        // See data sheet, sections 31.7.24 and 9.3.1.
        tx_pin.select_peripheral_a();

        // Enable UART using the Power Management Controller. See data sheet,
        // sections 34.4.2, 28.7, 28.15.4, and 9.1.
        // This is a no-op, as the UART is always clocked. I'm leaving it in
        // because the documentation is contradictory on the matter. I'm
        // suspecting that this contradiction might be caused by
        // copy-and-paste mistakes, which may indicate that the UART is not
        // always clocked in other microcontrollers. Not having this line,
        // even though it is a no-op for SAM3X8E, might cause a problem when
        // porting to other microcontrollers, and I'd rather waste a cycle
        // here than pulling out my hair searching for problems later.
        pmc::enable_peripheral_clock_0(&Peripheral::Uart);

        // Configure UART baud rate. See data sheet, sections 34.5.1 and
        // 34.6.9.
        const BAUD_RATE: u32 = 9600;
        let clock_divisor = pmc::main_clock_frequency_hz() / BAUD_RATE / 16;
        (*UART).baud_rate_generator.write(clock_divisor);

        // Set parity to no parity. See data sheet, section 34.6.2.
        (*UART).mode.write(
            uart::MODE_NORMAL | uart::PARITY_EVEN
        );

        // Enable receiver and transmitter. See data sheet, sections 34.5.2.1,
        // 34.5.3.1, and 34.6.1.
        (*UART).control.write(uart::RXEN | uart::TXEN);

        Uart {
            _tx_pin: tx_pin,
        }
    }

    pub fn send(&mut self, data: u8) {
        unsafe {
            // Wait until transmitter is ready. See data sheet, sections
            // 34.5.3.3 and 34.6.6.
            while (*UART).status.read() & uart::TXRDY == 0 {}

            // Send character. See data sheet, sections 34.5.3.3 and 34.6.8.
            (*UART).transmit_holding.write(data as u32);

            // Wait until character has been sent. See data sheet, sections
            // 34.5.3.3 and 34.6.6.
            while (*UART).status.read() & uart::TXEMPTY == 0 {}
        }
    }
}