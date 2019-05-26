extern crate libc;

use std::env;
use std::ffi::CString;
use libc::c_char;

const ARGV_MAX_SIZE: usize = 64;


pub fn get_c_argc_argv() -> (isize, *const*const c_char) {
    // TODO: make this argument parsing not fixed length.
    let mut argv: [*const c_char; ARGV_MAX_SIZE] = [0 as *const c_char; ARGV_MAX_SIZE];
    let mut argc: isize = 0;

    for argument in env::args() {
        if argc >= ARGV_MAX_SIZE as isize {
            panic!("Too many command line options!");
        }
        argv[argc as usize] = CString::new(argument).unwrap().into_raw();
        argc += 1;
        print_c_string(argv[argc as usize]);
    }

    (argc, argv.as_ptr())
}

pub fn print_c_string(string_ptr: *const c_char) {
    if string_ptr == 0 as *const c_char {
        println!("ERROR: String given to print_c_string is NULL");
        return;
    }

    print!("'");
    let mut offs = 0;
    unsafe {
        while *string_ptr.offset(offs) != 0 {
            print!("{}", *string_ptr.offset(offs) as u8 as char);
            offs += 1;
        }
    }
    println!("'");
}