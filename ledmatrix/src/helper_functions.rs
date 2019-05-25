extern crate libc;

use std::env;
use std::ffi::CString;
use libc::c_char;

const ARGV_MAX_SIZE: usize = 128;


pub fn get_c_argc_argv() -> (i32, *const*const c_char) {
    // TODO: make this argument parsing not fixed length.
    let mut argv: [*const c_char; ARGV_MAX_SIZE] = [CString::new("").unwrap().into_raw(); ARGV_MAX_SIZE];
    let mut argc: i32 = 0;

    for (ind, argument) in env::args().enumerate() {
        argc += 1;
        if argc >= ARGV_MAX_SIZE as i32 {
            panic!("Too many command line options!");
        }
        argv[ind] = CString::new(argument).unwrap().into_raw();
    }

    (argc, argv.as_ptr())
}
