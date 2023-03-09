mod ffi {
    extern "C" {
        pub fn verilated_got_finish() -> bool;
        pub fn verilated_time() -> u64;
        pub fn verilated_time_inc(add: u64);
    }
}

pub fn got_finish() -> bool {
    unsafe { ffi::verilated_got_finish() }
}

pub fn time() -> u64 {
    unsafe { ffi::verilated_time() }
}

pub fn time_inc(add: u64) {
    unsafe { ffi::verilated_time_inc(add) };
}
