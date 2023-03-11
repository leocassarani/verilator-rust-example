use crate::port::{Port, SinglePort, WidePort};

mod ffi {
    pub enum Vtop {}

    extern "C" {
        pub fn vtop_new() -> *mut Vtop;
        pub fn vtop_delete(top: *mut Vtop);
        pub fn vtop_eval(top: *mut Vtop);
        pub fn vtop_final(top: *mut Vtop);

        pub fn vtop_port_clk(top: *mut Vtop) -> *mut u8;
        pub fn vtop_port_reset_l(top: *mut Vtop) -> *mut u8;
        pub fn vtop_port_out_small(top: *mut Vtop) -> *mut u8;
        pub fn vtop_port_in_small(top: *mut Vtop) -> *mut u8;
        pub fn vtop_port_out_wide(top: *mut Vtop) -> *mut [u32; 3];
        pub fn vtop_port_in_wide(top: *mut Vtop) -> *mut [u32; 3];
        pub fn vtop_port_out_quad(top: *mut Vtop) -> *mut u64;
        pub fn vtop_port_in_quad(top: *mut Vtop) -> *mut u64;
    }
}

#[allow(dead_code)]
pub struct Top {
    raw: *mut ffi::Vtop,

    pub clk: SinglePort,
    pub reset_l: SinglePort,
    pub out_small: Port<u8>,
    pub in_small: Port<u8>,
    pub out_wide: WidePort<3>,
    pub in_wide: WidePort<3>,
    pub out_quad: Port<u64>,
    pub in_quad: Port<u64>,
}

impl Top {
    pub fn new() -> Self {
        unsafe {
            let raw = ffi::vtop_new();

            Top {
                raw,
                clk: SinglePort::new(ffi::vtop_port_clk(raw)),
                reset_l: SinglePort::new(ffi::vtop_port_reset_l(raw)),
                out_small: Port::new(ffi::vtop_port_out_small(raw), 1, 0),
                in_small: Port::new(ffi::vtop_port_in_small(raw), 1, 0),
                out_wide: WidePort::new(ffi::vtop_port_out_wide(raw), 69, 0),
                in_wide: WidePort::new(ffi::vtop_port_in_wide(raw), 69, 0),
                out_quad: Port::new(ffi::vtop_port_out_quad(raw), 39, 0),
                in_quad: Port::new(ffi::vtop_port_in_quad(raw), 39, 0),
            }
        }
    }

    pub fn eval(&mut self) {
        unsafe { ffi::vtop_eval(self.raw) };
    }

    pub fn finalize(&mut self) {
        unsafe { ffi::vtop_final(self.raw) };
    }
}

impl Drop for Top {
    fn drop(&mut self) {
        unsafe { ffi::vtop_delete(self.raw) };
    }
}
