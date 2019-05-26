mod c_api;
mod helper_functions;

extern crate libc;
use libc::{c_char, c_int};
use std::ffi::CString;
use std::path::Path;

pub const ARGV_MAX_SIZE: usize = 64;

pub enum HardwareMapping {
    Regular = 0,
    AdafruitHat = 1,
    AdafruitHatPWM = 2,
}

impl HardwareMapping {
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

pub struct Matrix {
    matrix: *mut c_api::RGBLedMatrix,
    // pub options: LEDMatrixOptions,
}

pub struct Canvas {
    canvas: *mut c_api::LedCanvas,
}

pub struct LEDMatrixOptions {
    pub mapping: HardwareMapping,
    pub rows: i32,
    pub cols: i32,
    pub chain_length: i32,
    pub parallel: i32,
    pwm_bits: i32,
    pwm_lsb_nanoseconds: i32,
    pwm_dither_bits: i32,
    brightness: u8,
    scan_mode: i32,
    row_address_type: i32,
    // TODO: add enum of multitplexing types?
    //   0=direct; 1=strip; 2=checker; 3=spiral; 4=Z-strip
    multiplexing: i32,
    led_rgb_sequence: RGBSequence,
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
}

/*
 * Matrix
 */

impl Matrix {
    pub fn new_from_options(options: &LEDMatrixOptions) -> Matrix {
        // build up the C struct of options from our options
        let mut c_options = c_api::RGBLedMatrixOptions {
            hardware_mapping: options.mapping.to_string(),
            rows: options.rows,
            cols: options.cols,
            chain_length: options.chain_length,
            parallel: options.parallel,
            pwm_bits: options.pwm_bits,
            pwm_lsb_nanoseconds: options.pwm_lsb_nanoseconds,
            pwm_dither_bits: options.pwm_dither_bits,
            brightness: options.brightness as i32,
            scan_mode: options.scan_mode,
            row_address_type: options.row_address_type,
            multiplexing: options.multiplexing,
            led_rgb_sequence: options.led_rgb_sequence.to_string(),
            pixel_mapper_config: CString::new("").unwrap().into_raw(),
            various_bitfield_options: 0,
        };

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

            Matrix {
                matrix: m,
            }
        }
    }

    pub fn new(rows: i32, chained: i32, parallel: i32) -> Matrix {
        unsafe {
            let m: *mut c_api::RGBLedMatrix = c_api::led_matrix_create(rows, chained, parallel);

            Matrix { matrix: m }
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

    pub fn get_canvas(&mut self) -> Canvas {
        unsafe { Canvas::new(c_api::led_matrix_get_canvas(self.matrix)) }
    }

    pub fn create_offscreen_canvas(&mut self) -> Canvas {
        unsafe { Canvas::new(c_api::led_matrix_create_offscreen_canvas(self.matrix)) }
    }

    pub fn swap_canvas_on_vsync(
        &mut self,
        canvas_to_draw: &mut Canvas,
        new_offscreen_canvas: &mut Canvas,
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

/*
 * Canvas
 */

impl Canvas {
    fn new(canvas_ref: *mut c_api::LedCanvas) -> Canvas {
        Canvas { canvas: canvas_ref }
    }

    pub fn get_size(&self) -> (i32, i32) {
        let width: i32 = 0;
        let height: i32 = 0;

        unsafe {
            c_api::led_canvas_get_size(self.canvas, width as *mut c_int, height as *mut c_int);
        }

        return (width, height);
    }

    pub fn clear(&mut self) {
        unsafe {
            c_api::led_canvas_clear(self.canvas);
        }
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8) {
        unsafe {
            c_api::led_canvas_fill(self.canvas, r, g, b);
        }
    }

    pub fn set_pixel(&mut self, x: i32, y: i32, r: u8, g: u8, b: u8) {
        unsafe {
            c_api::led_canvas_set_pixel(self.canvas, x, y, r, g, b);
        }
    }

    pub fn draw_circle(&mut self, x: i32, y: i32, radius: i32, r: u8, g: u8, b: u8) {
        unsafe {
            c_api::draw_circle(self.canvas, x, y, radius, r, g, b);
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, r: u8, g: u8, b: u8) {
        unsafe {
            c_api::draw_line(self.canvas, x0, y0, x1, y1, r, g, b);
        }
    }

    pub fn draw_text(
        &mut self,
        font: &mut Font,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: &str,
        kerning_offset: i32,
    ) {
        let cstr = CString::new(utf8_text).unwrap().as_ptr();
        unsafe {
            c_api::draw_text(self.canvas, font.font, x, y, r, g, b, cstr, kerning_offset);
        }
    }

    pub fn vertical_draw_text(
        &mut self,
        font: &mut Font,
        x: i32,
        y: i32,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: &str,
        kerning_offset: i32,
    ) {
        let cstr = CString::new(utf8_text).unwrap().as_ptr();
        unsafe {
            c_api::vertical_draw_text(self.canvas, font.font, x, y, r, g, b, cstr, kerning_offset);
        }
    }
}

/*
 * Font
 */

pub struct Font {
    font: *mut c_api::LedFont,
}

impl Font {
    pub fn new(bdf_filepath: &Path) -> Result<Font, &'static str> {
        // validate path
        let abs_path = bdf_filepath.canonicalize().unwrap();
        if !abs_path.exists() {
            return Err("Filepath does not appear to exist!");
        }

        if let Some(ext) = abs_path.extension() {
            if !(ext == "bdf") {
                return Err("Given filepath does not appear to be a .bdf file!");
            }
        } else {
            return Err("Given filepath doesn't even have a file extension!");
        }

        // make the object
        let string = CString::new(abs_path.to_str().unwrap()).unwrap();
        unsafe {
            return Ok(Font {
                font: c_api::load_font(string.as_ptr()),
            });
        }
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            c_api::delete_font(self.font);
        }
    }
}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn c_args() {
//         let (argc, argv) = helper_functions::get_c_argc_argv();
//         unsafe {
//             for i in 0..argc {
//                 println!("{:?}", *argv[i as usize]);
//             }
//         }
//     }
// }