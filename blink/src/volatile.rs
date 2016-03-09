use core::ptr;


pub struct Volatile<T> {
    value: T,
}

impl<T> Volatile<T> {
    pub unsafe fn read(&self) -> T {
        ptr::read_volatile(&self.value)
    }

    pub unsafe fn write(&mut self, value: T) {
        ptr::write_volatile(&mut self.value, value)
    }
}
