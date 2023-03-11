use std::ffi::{c_char, c_int, CString};

mod ffi {
    use std::ffi::{c_char, c_int};

    extern "C" {
        pub fn verilated_command_args(argc: c_int, argv: *const *const c_char);
        pub fn verilated_got_finish() -> bool;
        pub fn verilated_trace_ever_on(flag: bool);
        pub fn verilated_time() -> u64;
        pub fn verilated_time_inc(add: u64);
    }
}

pub fn command_args(args: Vec<String>) {
    let argc = args.len();

    let argv: Vec<*const c_char> = args
        .into_iter()
        .map(|s| CString::new(s).unwrap().into_raw() as _)
        .collect();

    unsafe { ffi::verilated_command_args(argc as c_int, argv.as_ptr()) };
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

pub fn trace_ever_on(flag: bool) {
    unsafe { ffi::verilated_trace_ever_on(flag) };
}
