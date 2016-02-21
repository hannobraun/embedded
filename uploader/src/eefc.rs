use result::Result;
use sam_ba::SamBa;


/// Provides an interface to the Enhanced Embedded Flash Controller (EEFC).
/// See data sheet, chapter 18.
pub struct Eefc {
    command_register: u32,
    status_register : u32,
    result_register : u32,
}

impl Eefc {
    /// Returns an instance that interfaces with the controller for the first
    /// flash memory bank.
    /// See data sheet, chapter 18.5.
    pub fn eefc_0() -> Eefc {
        let base = 0x400e0a00;

        Eefc {
            command_register: base + 0x04,
            status_register : base + 0x08,
            result_register : base + 0x0c,
        }
    }

    /// Executes a flash command and returns its result. The commands and
    /// arguments that can be passed to this method are defined below.
    ///
    /// See data sheet chapter 18.4.3 for the list of commands, and chapter
    /// 18.5 (especially 18.5.2) for information about what's going on in this
    /// method.
    pub fn execute_command<C, A>(&self, sam_ba: &mut SamBa, argument: A)
        -> Result<u32>
        where
            C: Command<Argument=A>,
            A: Argument,
    {
        let command =
            0x5a << 24
            | (argument.value() as u32) << 8
            | C::value() as u32;

        try!(sam_ba.write_word(self.command_register, command));

        while try!(sam_ba.read_word(self.status_register)) & 1 == 0 {}
        assert_eq!(try!(sam_ba.read_word(self.status_register)), 1);

        sam_ba.read_word(self.result_register)
    }
}


pub trait Command {
    type Argument: Argument;

    fn value() -> u8;
}

pub trait Argument {
    fn value(self) -> u16;
}


pub struct ErasePageAndWritePage;

impl Command for ErasePageAndWritePage {
    type Argument = Page;

    fn value() -> u8 { 0x03 }
}

pub struct Page(pub u16);

impl Argument for Page {
    fn value(self) -> u16 {
        let Page(page) = self;
        page
    }
}


pub struct SetGpnvmBit;

impl Command for SetGpnvmBit {
    type Argument = GpnvmNumber;

    fn value() -> u8 { 0x0b }
}

pub enum GpnvmNumber {
    // Security          = 0,
    BootModeSelection = 1,
    // FlashSelection    = 2,
}

impl Argument for GpnvmNumber {
    fn value(self) -> u16 { self as u16 }
}
