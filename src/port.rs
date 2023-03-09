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

pub struct WidePort<const N: usize> {
    raw: *mut [u32; N],
    mask: [u32; N],
}

impl<const N: usize> WidePort<N> {
    pub fn new(raw: *mut [u32; N], msb: u32, lsb: u32) -> Self {
        WidePort {
            raw,
            mask: wide_bitmask(msb, lsb),
        }
    }

    pub fn get(&self) -> [u32; N] {
        unsafe { self.masked(*self.raw) }
    }

    pub fn set(&mut self, v: [u32; N]) {
        unsafe { *self.raw = self.masked(v) };
    }

    fn masked(&self, mut v: [u32; N]) -> [u32; N] {
        for i in 0..N {
            v[i] &= self.mask[i];
        }
        v
    }
}

fn wide_bitmask<const N: usize>(msb: u32, lsb: u32) -> [u32; N] {
    let mut mask = [0; N];

    assert!(msb < 32 * N as u32);
    assert!(lsb <= msb);

    let from = lsb / 32;
    let to = msb / 32;

    for byte in from..=to {
        let floor = 32 * byte;
        let msb = (msb - floor).clamp(0, 31);
        let lsb = lsb.saturating_sub(floor);
        mask[byte as usize] = bitmask(msb, lsb);
    }

    mask
}

fn bitmask<T: num_traits::PrimInt>(msb: u32, lsb: u32) -> T {
    let bitwidth = T::zero().count_zeros();
    let maxbit = bitwidth - 1;

    assert!(msb <= maxbit);
    assert!(lsb <= msb);

    let ones = T::max_value();
    ones.unsigned_shr(maxbit - msb) & ones.unsigned_shl(lsb)
}
