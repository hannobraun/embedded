// Some basic stuff that is required to get Rust and its core library to work.


// These are a few language items that are required by the core library. The
// core library is completely platform agnostic and doesn't assume anything
// (besides very basic things like a stack) about the platform it is running
// on. Therefore it can't know how to handle program panics and the like.

#[lang = "panic_fmt"]
pub extern fn panic_fmt() {
    // TODO: Print a message to serial output, if available and reset the
    //       system.
    loop {}
}

#[lang = "eh_personality"]
pub extern fn eh_personality() {
    panic!("eh_personality function has been called")
}


// I'm not 100% sure what this function does, but references to it are compiled
// into the program by the Rust compiler. I think it would be called in the
// case of a program panic.
#[no_mangle] pub extern fn __aeabi_unwind_cpp_pr0() { loop {} }
