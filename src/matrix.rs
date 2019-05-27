use super::c_api;
use super::c_datatypes;
use super::ARGV_MAX_SIZE;
use super::helper_functions;
use super::canvas;

use std::ffi::CString;
use libc::{c_int, c_char};

pub enum HardwareMapping {
    Regular = 0,
    AdafruitHat = 1,
    AdafruitHatPWM = 2,
}

impl HardwareMapping {
    pub fn from_string(string: &str) -> HardwareMapping {
        match string {
            "adafruit-hat" => HardwareMapping::AdafruitHat,
            "adafruit-hat-pwm" => HardwareMapping::AdafruitHatPWM,
            _ => HardwareMapping::Regular
        }
    }

    pub fn to_string(&self) -> *const c_char {
        match self {
            HardwareMapping::Regular => CString::new("regular").unwrap().into_raw(),
            HardwareMapping::AdafruitHat => CString::new("adafruit-hat").unwrap().into_raw(),
            HardwareMapping::AdafruitHatPWM => CString::new("adafruit-hat-pwm").unwrap().into_raw(),
        }
    }
}

pub enum RGBSequence {
    RGB = 0,
    RBG = 1,
    GRB = 2,
    GBR = 3,
    BGR = 4,
    BRG = 5,
}

impl RGBSequence {
    pub fn from_string(string: &str) -> RGBSequence {
        match string {
            "RBG" => RGBSequence::RBG,
            "GRB" => RGBSequence::GRB,
            "GBR" => RGBSequence::GBR,
            "BGR" => RGBSequence::BGR,
            "BRG" => RGBSequence::BRG,
            _ => RGBSequence::RGB,
        }
    }

    pub fn to_string(&self) -> *const c_char {
        match self {
            RGBSequence::RGB => CString::new("RGB").unwrap().into_raw(),
            RGBSequence::RBG => CString::new("RBG").unwrap().into_raw(),
            RGBSequence::GRB => CString::new("GRB").unwrap().into_raw(),
            RGBSequence::GBR => CString::new("GBR").unwrap().into_raw(),
            RGBSequence::BGR => CString::new("BGR").unwrap().into_raw(),
            RGBSequence::BRG => CString::new("BRG").unwrap().into_raw(),
        }
    }
}

/*
 * Matrix Options
 */

pub struct LEDMatrixOptions {
    pub mapping: HardwareMapping,
    pub rows: i32,
    pub cols: i32,
    pub chain_length: i32,
    pub parallel: i32,
    pub(crate) pwm_bits: i32,
    pub(crate) pwm_lsb_nanoseconds: i32,
    pub(crate) pwm_dither_bits: i32,
    pub(crate) brightness: u8,
    pub(crate) scan_mode: i32,
    pub(crate) row_address_type: i32,
    // TODO: add enum of multitplexing types?
    //   0=direct; 1=strip; 2=checker; 3=spiral; 4=Z-strip
    pub(crate) multiplexing: i32,
    pub(crate) led_rgb_sequence: RGBSequence,
}

impl LEDMatrixOptions {
    pub fn new(
        mapping: HardwareMapping,
        rows: i32,
        cols: i32,
        chain_length: i32,
        parallel: i32,
        brightness: u8
    ) -> LEDMatrixOptions {
        LEDMatrixOptions {
            mapping: mapping,
            rows: rows,
            cols: cols,
            chain_length: chain_length,
            parallel: parallel,
            brightness: brightness,
            pwm_bits: 11,
            pwm_lsb_nanoseconds: 130,
            pwm_dither_bits: 0,
            scan_mode: 0,
            row_address_type: 0,
            multiplexing: 0,
            led_rgb_sequence: RGBSequence::RGB,
        }
    }

    fn from_c_options(c_options: &c_datatypes::LEDMatrixOptions) -> LEDMatrixOptions {
        // All this work to convert a C string to a rust string... wow
        let sequence_ptr: *mut c_char = c_options.led_rgb_sequence as *mut c_char;
        let mapping_ptr: *mut c_char = c_options.hardware_mapping as *mut c_char;

        unsafe {
            // these functions are unsafe
            let sequence_str = CString::from_raw(sequence_ptr).into_string().unwrap();
            let mapping_str = CString::from_raw(mapping_ptr).into_string().unwrap();

            // the rest needs to be in this block because of those functions :(
            let sequence = RGBSequence::from_string(&sequence_str);
            let mapping = HardwareMapping::from_string(&mapping_str);

            LEDMatrixOptions {
                mapping: mapping,
                rows: c_options.rows as i32,
                cols: c_options.cols as i32,
                chain_length: c_options.chain_length as i32,

                parallel: c_options.parallel as i32,
                brightness: c_options.brightness as u8,

                pwm_bits: c_options.pwm_bits as i32,
                pwm_lsb_nanoseconds: c_options.pwm_lsb_nanoseconds as i32,
                pwm_dither_bits: c_options.pwm_dither_bits as i32,

                scan_mode: c_options.scan_mode as i32,
                row_address_type: c_options.row_address_type as i32,
                multiplexing: c_options.multiplexing as i32,
                led_rgb_sequence: sequence,
            }
        }
    }
}

/*
 * Matrix
 */

pub struct Matrix {
    matrix: *mut c_datatypes::RGBLedMatrix,
    pub options: LEDMatrixOptions,
}

impl Matrix {
    pub fn new_from_options(options: &LEDMatrixOptions) -> Matrix {
        // build up the C struct of options from our options
        let mut c_options = c_datatypes::LEDMatrixOptions::new_from_options(options);

        // TODO: Try to make this not static size?
        let mut argv: [*const c_char; ARGV_MAX_SIZE] = [0 as *const c_char; ARGV_MAX_SIZE];
        let argc = helper_functions::get_c_argc_argv(&mut argv);
        let argv_raw = argv.as_ptr() as *const*const c_char;

        unsafe {
            println!("Arguments:");
            for i in 0..argc {
                let val: *const c_char = *argv_raw.offset(i as isize);
                print!("\t"); helper_functions::print_c_string(val);
            }

            let m = c_api::led_matrix_create_from_options(
                &mut c_options,
                &(argc as c_int),
                &argv_raw as *const*const*const c_char
            );

            // our options might have changed from command line options. update and store
            let updated_options = LEDMatrixOptions::from_c_options(&c_options);

            Matrix {
                matrix: m,
                options: updated_options
            }
        }
    }

    pub fn new(rows: i32, chained: i32, parallel: i32) -> Matrix {
        unsafe {
            let m: *mut c_datatypes::RGBLedMatrix = c_api::led_matrix_create(rows, chained, parallel);
            let options = LEDMatrixOptions::new(
                HardwareMapping::Regular,
                rows, -1,
                chained, parallel,
                100
            );
            Matrix { matrix: m, options: options }
        }
    }

    pub fn get_brightness(&mut self) -> u8 {
        unsafe { c_api::led_matrix_get_brightness(self.matrix) }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe {
            c_api::led_matrix_set_brightness(self.matrix, brightness);
        }
    }

    pub fn get_canvas(&mut self) -> canvas::Canvas {
        unsafe { canvas::Canvas::new(c_api::led_matrix_get_canvas(self.matrix)) }
    }

    pub fn create_offscreen_canvas(&mut self) -> canvas::Canvas {
        unsafe { canvas::Canvas::new(c_api::led_matrix_create_offscreen_canvas(self.matrix)) }
    }

    pub fn swap_canvas_on_vsync(
        &mut self,
        canvas_to_draw: &mut canvas::Canvas,
        new_offscreen_canvas: &mut canvas::Canvas,
    ) {
        unsafe {
            new_offscreen_canvas.canvas =
                c_api::led_matrix_swap_on_vsync(self.matrix, canvas_to_draw.canvas);
        }
    }
}

impl Drop for Matrix {
    fn drop(&mut self) {
        unsafe {
            c_api::led_matrix_delete(self.matrix);
        }
    }
}
