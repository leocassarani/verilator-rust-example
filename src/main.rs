mod port;
mod verilated;

use port::{Port, SinglePort, WidePort};

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
    clk: SinglePort,
    reset_l: SinglePort,
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

fn main() {
    let mut top = Top::new();

    top.reset_l.set(true);
    top.clk.set(false);
    top.in_small.set(1);
    top.in_quad.set(0x1234);
    top.in_wide.set([0x11111111, 0x22222222, 0x3]);

    while !verilated::got_finish() {
        top.clk.toggle();

        verilated::time_inc(1);

        if !top.clk.get() {
            if verilated::time() > 1 && verilated::time() < 10 {
                top.reset_l.set(false);
            } else {
                top.reset_l.set(true);
            }

            top.in_quad.set_with(|v| v + 0x12);
        }

        top.eval();

        println!(
            "[{:x}] clk={:x} rstl={:x} iquad={:x} -> oquad={:x} owide={:x}_{:08x}_{:08x}",
            verilated::time(),
            top.clk.get() as u8,
            top.reset_l.get() as u8,
            top.in_quad.get(),
            top.out_quad.get(),
            top.out_wide.get()[2],
            top.out_wide.get()[1],
            top.out_wide.get()[0],
        );
    }

    top.finalize();
}
