/// Functions for enabling and disabling interrupts.
/// The function documentation refers to the ARM Cortex-M Programming Guide to
/// Memory Barrier Instructions (Application Note 321), available online at:
/// http://infocenter.arm.com/help/index.jsp?topic=/com.arm.doc.dai0321a/index.html


/// Enable interrupts.
pub fn enable() {
    unsafe {
        asm!("cpsie i" :::: "volatile")
    };
}

/// Disable interrupts.
/// According to Application Note 321, section 3.3, this instruction is self
/// synchronizing, which means the interrupts are disabled after this
/// instruction completes, and no explicit memory barrier is required.
pub fn disable() {
    unsafe {
        asm!("cpsid i" :::: "volatile");
    }
}
