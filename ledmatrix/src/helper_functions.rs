extern crate libc;

use std::env;
use std::ffi::CString;
use libc::c_char;

const ARGV_MAX_SIZE: usize = 128;


pub fn get_c_argc_argv() -> (usize, *const*const c_char) {
    // TODO: make this argument parsing not fixed length.
    let mut argv: [*const c_char; ARGV_MAX_SIZE] = [CString::new("").unwrap().into_raw(); ARGV_MAX_SIZE];
    let mut argc = 0;

    println!("args from env::args():");
    for  argument in env::args() {
        if argc >= ARGV_MAX_SIZE {
            panic!("Too many command line options!");
        }
        println!("\t{:?}", argument);
        argv[argc] = CString::new(argument).unwrap().into_raw();
        argc += 1;
    }

    (argc, argv.as_ptr())
}
