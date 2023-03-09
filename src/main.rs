mod port;

use port::{Port, WidePort};

mod ffi {
    pub enum Vtop {}

    extern "C" {
        pub fn vtop_new() -> *mut Vtop;
        pub fn vtop_delete(top: *mut Vtop);
        pub fn vtop_eval(top: *mut Vtop);
        pub fn vtop_finish(top: *mut Vtop);

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
    clk: Port<u8>,
    reset_l: Port<u8>,
    out_small: Port<u8>,
    in_small: Port<u8>,
    out_wide: WidePort<3>,
    in_wide: WidePort<3>,
    out_quad: Port<u64>,
    in_quad: Port<u64>,
}

impl Top {
    pub fn new() -> Self {
        unsafe {
            let raw = ffi::vtop_new();

            Top {
                raw,
                clk: Port::new(ffi::vtop_port_clk(raw), 0, 0),
                reset_l: Port::new(ffi::vtop_port_reset_l(raw), 0, 0),
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

    pub fn finish(&mut self) {
        unsafe { ffi::vtop_finish(self.raw) };
    }
}

impl Drop for Top {
    fn drop(&mut self) {
        unsafe { ffi::vtop_delete(self.raw) };
    }
}

fn main() {
    let mut top = Top::new();

    top.reset_l.set(1);
    top.clk.set(0);
    top.in_small.set(1);
    top.in_quad.set(0x1234);
    top.in_wide.set([0x11111111, 0x22222222, 0x3]);

    for time in 0..20 {
        top.clk.set_with(|v| if v == 0 { 1 } else { 0 });

        if top.clk.get() == 0 {
            if time < 10 {
                top.reset_l.set(0);
            } else {
                top.reset_l.set(1);
            }

            top.in_quad.set_with(|v| v + 0x12);
        }

        top.eval();

        println!(
            "clk={:x} rstl={:x} iquad={:x} -> oquad={:x} owide={:x}_{:08x}_{:08x}",
            top.clk.get(),
            top.reset_l.get(),
            top.in_quad.get(),
            top.out_quad.get(),
            top.out_wide.get()[2],
            top.out_wide.get()[1],
            top.out_wide.get()[0],
        );
    }

    top.finish();
}
