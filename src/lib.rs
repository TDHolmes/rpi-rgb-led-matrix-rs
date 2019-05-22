mod c_api;

extern crate libc;
use libc::{c_void, c_char, c_int};
use std::ffi::CString;


pub enum HardwareMapping {
    Regular = 0,
    AdafruitHat = 1,
    AdafruitHatPWM = 2
}

fn mapping_to_string(mapping: &HardwareMapping) -> *const c_char {
    match mapping {
        HardwareMapping::Regular        => CString::new("regular").unwrap().as_ptr(),
        HardwareMapping::AdafruitHat    => CString::new("adafruit-hat").unwrap().as_ptr(),
        HardwareMapping::AdafruitHatPWM => CString::new("adafruit-hat-pwm").unwrap().as_ptr(),
    }
}

pub enum RGBSequence {
    RGB = 0, RBG = 1,
    GRB = 2, GBR = 3,
    BGR = 4, BRG = 5
}

fn sequence_to_string(sequence: &RGBSequence) -> *const c_char {
    match sequence {
        RGBSequence::RGB => CString::new("RGB").unwrap().as_ptr(),
        RGBSequence::RBG => CString::new("RBG").unwrap().as_ptr(),
        RGBSequence::GRB => CString::new("GRB").unwrap().as_ptr(),
        RGBSequence::GBR => CString::new("GBR").unwrap().as_ptr(),
        RGBSequence::BGR => CString::new("BGR").unwrap().as_ptr(),
        RGBSequence::BRG => CString::new("BRG").unwrap().as_ptr(),
    }
}

pub struct Matrix {
    matrix: *mut c_api::RGBLedMatrix,
    pub options: LEDMatrixOptions
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
    brightness: i32,
    scan_mode: i32,
    row_address_type: i32,
    multiplexing: i32,
    led_rgb_sequence: RGBSequence,
}


impl Matrix {
    pub fn new_from_options(options: &LEDMatrixOptions) -> Matrix {
        // build up the C struct of options from our options
        let mut c_options = c_api::RGBLedMatrixOptions {
            hardware_mapping : mapping_to_string(&options.mapping),
            rows : options.rows,
            cols : options.cols,
            chain_length : options.chain_length,
            parallel : options.parallel,
            pwm_bits : options.pwm_bits,
            pwm_lsb_nanoseconds : options.pwm_lsb_nanoseconds,
            pwm_dither_bits : options.pwm_dither_bits,
            brightness : options.brightness,
            scan_mode : options.scan_mode,
            row_address_type : options.row_address_type,
            multiplexing : options.multiplexing,
            led_rgb_sequence : sequence_to_string(&options.led_rgb_sequence),
            pixel_mapper_config : CString::new("").unwrap().as_ptr(),
            various_bitfield_options: 0
        };

        // put in dummy values for these unused variables. argv isn't even the right format,
        //   so don't even try.
        let argc: c_int = 0;
        let argv: *const c_char = CString::new("").unwrap().as_ptr();

        unsafe {
            let m = c_api::led_matrix_create_from_options(&mut c_options, &argc, argv);

            Matrix {
                matrix : m,
                options: *options
            }
        }
    }

    pub fn new(rows: i32, chained: i32, parallel: i32) -> Matrix {
        unsafe {
            let m: *mut c_api::RGBLedMatrix = c_api::led_matrix_create(rows, chained, parallel);

            Matrix {
                matrix: m
            }
        }
    }

    pub fn get_brightness(&mut self) -> u8 {
        unsafe {
            c_api::led_matrix_get_brightness(self.matrix)
        }
    }

    pub fn set_brightness(&mut self, brightness: u8) {
        unsafe {
            c_api::led_matrix_set_brightness(self.matrix, brightness);
        }
    }

    pub fn get_canvas(&mut self) -> Canvas {
        unsafe {
            Canvas::new(c_api::led_matrix_get_canvas(self.matrix))
        }
    }

    pub fn create_offscreen_canvas(&mut self) -> Canvas {
        unsafe {
            Canvas::new(c_api::led_matrix_create_offscreen_canvas(self.matrix))
        }
    }

    pub fn swap_canvas_on_vsync(&mut self, canvas_to_draw: &mut Canvas, new_offscreen_canvas: &mut Canvas) {
        unsafe {
            new_offscreen_canvas.canvas = c_api::led_matrix_swap_on_vsync(self.matrix, canvas_to_draw.canvas);
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

impl Canvas {
    fn new(canvas_ref: *mut c_api::LedCanvas) -> Canvas {
        Canvas {
            canvas: canvas_ref
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        let width: i32 = 0;
        let height: i32 = 0;

        unsafe {
            c_api::led_canvas_get_size(self.canvas, width as *mut c_int, height as *mut c_int);
        }

        return (width, height)
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

    // pub fn draw_text(
    //     c: *mut LedCanvas, font: *mut LedFont, x: c_int, y: c_int,
    //     r: u8, g: u8, b: u8,
    //     utf8_text: *const c_char, kerning_offset: c_int) -> c_int;

    // pub fn vertical_draw_text(
    //     c: *mut LedCanvas, font: *mut LedFont, x: c_int, y: c_int,
    //     r: u8, g: u8, b: u8,
    //     utf8_text: *const c_char, kerning_offset: c_int) -> c_int;

}