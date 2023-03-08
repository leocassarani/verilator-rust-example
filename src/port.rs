pub struct Port<T: Copy> {
    raw: *mut T,
}

impl<T: Copy> Port<T> {
    pub fn new(raw: *mut T) -> Self {
        Port { raw }
    }

    pub fn get(&self) -> T {
        unsafe { *self.raw }
    }

    pub fn set(&mut self, v: T) {
        unsafe { *self.raw = v };
    }
}
