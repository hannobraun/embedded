// Initialization code and entry point for the program.


#![feature(asm, intrinsics, lang_items, stmt_expr_attributes)]

#![no_main]
#![no_std]


use core::ptr;


#[macro_use]
pub mod debug;

mod program;

pub mod hardware {
    // Basic definition of the hardware's features
    pub mod base {
        pub mod nvic;
        pub mod pdc;
        pub mod peripherals;
        pub mod pio;
        pub mod pmc;
        pub mod rtt;
        pub mod uart;
        pub mod wdt;
    }

    // Safe API to the hardware's features
    pub mod safe {
        pub mod nvic;
        pub mod peripherals;
        pub mod pio;
        pub mod pmc;
        pub mod rtt;
        pub mod uart;
        pub mod wdt;
    }
}

pub mod interrupts;
pub mod rust_base;
pub mod volatile;


// These are various memory addresses provided by the linker.
//
// I'm not sure why it works that way, but we need to access those by
// reference to get the address. See:
// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dui0803b/CHDBIJJD.html
extern {
    static _estack: u32;

    static mut _etext    : u32;
    static mut _srelocate: u32;
    static mut _erelocate: u32;
}


// The data structure for the vector table. See data sheet, chapter 10.6.4.
#[repr(C)]
pub struct VectorTable {
    pub initial_stack_pointer: &'static u32,

    pub on_reset: fn(),

    pub _reserved_1: [u32; 1],

    pub on_hard_fault             : fn(),
    pub on_memory_management_fault: fn(),
    pub on_bus_fault              : fn(),
    pub on_usage_fault            : fn(),

    pub _reserved_2: [u32; 4],

    pub on_sv_call: fn(),

    pub _reserved_3: [u32; 2],

    pub on_pend_sv: fn(),
    pub on_systick: fn(),

    pub on_irq0 : fn(),
    pub on_irq1 : fn(),
    pub on_irq2 : fn(),
    pub on_irq3 : fn(),
    pub on_irq4 : fn(),
    pub on_irq5 : fn(),
    pub on_irq6 : fn(),
    pub on_irq7 : fn(),
    pub on_irq8 : fn(),
    pub on_irq9 : fn(),
    pub on_irq10: fn(),
    pub on_irq11: fn(),
    pub on_irq12: fn(),
    pub on_irq13: fn(),
    pub on_irq14: fn(),
    pub on_irq15: fn(),
    pub on_irq16: fn(),
    pub on_irq17: fn(),
    pub on_irq18: fn(),
    pub on_irq19: fn(),
    pub on_irq20: fn(),
    pub on_irq21: fn(),
    pub on_irq22: fn(),
    pub on_irq23: fn(),
    pub on_irq24: fn(),
    pub on_irq25: fn(),
    pub on_irq26: fn(),
    pub on_irq27: fn(),
    pub on_irq28: fn(),
    pub on_irq29: fn(),
}

unsafe impl Sync for VectorTable {}


// The vector table. We're telling the compiler to place this into .vectors
// section, not where it would normally go (I suppose .rodata). The linker
// script makes sure that the .vectors section is at the right place.
//
// The `#[no_mangle]` is surprisingly important. Without it, LLVM will optimize
// the vector table away, if compiler optimizations are enabled. If that
// happens, the resulting executable is only a few bytes large and no longer
// does anything.
#[link_section=".vectors"]
#[no_mangle]
pub static VECTOR_TABLE: VectorTable = VectorTable {
    initial_stack_pointer: &_estack,

    on_reset: on_reset,

    _reserved_1: [0; 1],

    on_hard_fault             : on_hard_fault,
    on_memory_management_fault: on_memory_management_fault,
    on_bus_fault              : on_bus_fault,
    on_usage_fault            : on_usage_fault,

    _reserved_2: [0; 4],

    on_sv_call: abort,

    _reserved_3: [0; 2],

    on_pend_sv: abort,
    on_systick: abort,

    on_irq0 : abort,
    on_irq1 : abort,
    on_irq2 : abort,
    on_irq3 : handle_rtt,
    on_irq4 : abort,
    on_irq5 : abort,
    on_irq6 : abort,
    on_irq7 : abort,
    on_irq8 : abort,
    on_irq9 : abort,
    on_irq10: abort,
    on_irq11: abort,
    on_irq12: abort,
    on_irq13: abort,
    on_irq14: abort,
    on_irq15: abort,
    on_irq16: abort,
    on_irq17: abort,
    on_irq18: abort,
    on_irq19: abort,
    on_irq20: abort,
    on_irq21: abort,
    on_irq22: abort,
    on_irq23: abort,
    on_irq24: abort,
    on_irq25: abort,
    on_irq26: abort,
    on_irq27: abort,
    on_irq28: abort,
    on_irq29: abort,
};


fn on_reset() {
    // Copy mutable global variables from the flash memory into RAM, where they
    // can be used. Please note that while I tested this when I wrote it, there
    // is, as I'm writing this, no code in this program that actually uses
    // global variables. If you make changes here, please make sure you test
    // thoroughly.
    // Also be aware that the compiler might optimize away global variables, so
    // having a `static mut` in the code doesn't actually mean this code here
    // is going to be needed. The best approach I found is to test with a debug
    // build.
    unsafe {
        let mut src = &mut _etext     as *const u32;
        let mut dst = &mut _srelocate as *mut u32;

        while dst < &mut _erelocate as *mut u32 {
            ptr::write_volatile(
                dst,
                ptr::read_volatile(src),
            );

            src = src.offset(1);
            dst = dst.offset(1);
        }
    }

    // TODO: We should initialize the .bss segment here by writing zeros into
    //       it. The .bss segment contains uninitialized global variables, and
    //       while I don't actually think that Rust itself uses the .bss
    //       segment (I might be totally wrong about this), we might want to
    //       link to C libraries that require that segment.
    //       Since I don't have a test case right now to verify if that the
    //       code I would write for this is correct, I opted not to do this at
    //       the moment. Once there is a test case, this should be a pretty
    //       straight-forward thing to do.

    program::start()
}

fn on_hard_fault() {
    panic!("Unhandled hard fault");
}

fn on_memory_management_fault() {
    panic!("Unhandled memory management fault");
}

fn on_bus_fault() {
    panic!("Unhandled bus fault");
}

fn on_usage_fault() {
    panic!("Unhandled usage fault");
}

// Used as a handler function for all interrupts we don't want to handle yet.
fn abort() {
    panic!("Unhandled interrupt");
}

fn handle_rtt() {
    use hardware::base::rtt::{
        self,
        RTT,
    };

    // I don't know why, but unless we explicitely disable it, the alarm
    // interrupt just keeps repeating, blocking the whole program.
    unsafe {
        let mode = (*RTT).mode.read();
        (*RTT).mode.write(mode & !rtt::ALMIEN);
    }
}
