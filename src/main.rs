mod port;

use port::Port;

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
        pub fn vtop_port_out_quad(top: *mut Vtop) -> *mut u64;
        pub fn vtop_port_in_quad(top: *mut Vtop) -> *mut u64;
    }
}

pub struct Top {
    raw: *mut ffi::Vtop,
    clk: Port<u8>,
    reset_l: Port<u8>,
    out_small: Port<u8>,
    in_small: Port<u8>,
    out_quad: Port<u64>,
    in_quad: Port<u64>,
}

impl Top {
    pub fn new() -> Self {
        unsafe {
            let raw = ffi::vtop_new();

            Top {
                raw,
                clk: Port::new(ffi::vtop_port_clk(raw)),
                reset_l: Port::new(ffi::vtop_port_reset_l(raw)),
                out_small: Port::new(ffi::vtop_port_out_small(raw)),
                in_small: Port::new(ffi::vtop_port_in_small(raw)),
                out_quad: Port::new(ffi::vtop_port_out_quad(raw)),
                in_quad: Port::new(ffi::vtop_port_in_quad(raw)),
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

    for time in 0..20 {
        top.clk.set(if top.clk.get() == 0 { 1 } else { 0 });

        if top.clk.get() == 0 {
            if time < 10 {
                top.reset_l.set(0);
            } else {
                top.reset_l.set(1);
            }

            top.in_quad.set(top.in_quad.get() + 0x12);
        }

        top.eval();

        println!(
            "clk={:x} rstl={:x} iquad={:x} -> oquad={:x}",
            top.clk.get(),
            top.reset_l.get(),
            top.in_quad.get(),
            top.out_quad.get(),
        );
    }

    top.finish();
}
