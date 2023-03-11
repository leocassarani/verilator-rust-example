mod port;
mod top;
mod verilated;

use top::Top;

fn main() {
    verilated::trace_ever_on(true);
    verilated::command_args(std::env::args().collect());

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
