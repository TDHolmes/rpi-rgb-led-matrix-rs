// public constants and modules
pub const ARGV_MAX_SIZE: usize = 64;

pub mod matrix;
pub mod canvas;

// internally public
pub(crate) mod c_api;
pub(crate) mod c_datatypes;
pub(crate) mod helper_functions;


#[cfg(test)]
mod tests {
    use super::*;
    use helper_functions;
    use libc::c_char;

    #[test]
    fn c_args() {
        let mut argv: [*const c_char; ARGV_MAX_SIZE] = [0 as *const c_char; ARGV_MAX_SIZE];
        let argc = helper_functions::get_c_argc_argv(&mut argv);
        unsafe {
            for i in 0..argc {
                println!("{:?}", *argv[i as usize]);
            }
        }
    }
}