extern crate libc;

use std::env;
use std::ffi::CString;
use libc::c_char;


pub fn get_c_argc_argv() -> (i32, [*const c_char; 128]) {
    // Prints each argument on a separate line
    let mut argv: [*const c_char; 128] = [CString::new("").unwrap().into_raw(); 128];
    let mut argc = 0;

    for (ind, argument) in env::args().enumerate() {
        argc += 1;
        println!("{:?}", argument);
        argv[ind] = CString::new(argument).unwrap().into_raw();
    }

    (argc, argv)
}