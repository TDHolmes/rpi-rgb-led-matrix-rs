extern crate libc;

use std::env;
use std::ffi::CString;
use libc::c_char;

use super::ARGV_MAX_SIZE;  // TODO: DISGUSTING


pub fn get_c_argc_argv(argv: &mut [*const c_char; ARGV_MAX_SIZE]) -> isize {
    let mut argc: isize = 0;

    for argument in env::args() {
        if argc >= ARGV_MAX_SIZE as isize {
            panic!("Too many command line options!");
        }
        argv[argc as usize] = CString::new(argument).unwrap().into_raw();
        argc += 1;
    }

    argc
}

pub fn print_c_string(string_ptr: *const c_char) {
    if string_ptr == 0 as *const c_char {
        panic!("ERROR: String given to print_c_string is NULL");
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