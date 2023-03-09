pub struct Port<T: num_traits::PrimInt> {
    raw: *mut T,
    mask: T,
}

impl<T: num_traits::PrimInt> Port<T> {
    pub fn new(raw: *mut T, msb: u32, lsb: u32) -> Self {
        Port {
            raw,
            mask: bitmask(msb, lsb),
        }
    }

    pub fn get(&self) -> T {
        self.masked(unsafe { *self.raw })
    }

    pub fn set(&mut self, v: T) {
        unsafe { *self.raw = self.masked(v) };
    }

    pub fn set_with<F>(&mut self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        self.set(f(self.get()));
    }

    fn masked(&self, v: T) -> T {
        v & self.mask
    }
}

fn bitmask<T: num_traits::PrimInt>(msb: u32, lsb: u32) -> T {
    let bitwidth = T::zero().count_zeros();
    let maxbit = bitwidth - 1;

    assert!(msb <= maxbit);
    assert!(lsb <= msb);

    let ones = T::max_value();
    ones.unsigned_shr(maxbit - msb) & ones.unsigned_shl(lsb)
}
