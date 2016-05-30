pub fn enable() {
    unsafe {
        asm!("cpsie i" :::: "volatile")
    };
}

pub fn disable() {
    unsafe {
        asm!("cpsid i" :::: "volatile");
    }
}
