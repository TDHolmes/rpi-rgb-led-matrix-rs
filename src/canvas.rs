use super::c_api;
use super::c_datatypes;

use std::path::Path;
use libc::c_int;
use std::ffi::CString;


/*
 * Canvas
 */

pub struct Canvas {
    pub(crate) canvas: *mut c_datatypes::LedCanvas,
}

impl Canvas {
    pub(crate) fn new(canvas_ref: *mut c_datatypes::LedCanvas) -> Canvas {
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
        let cstr = CString::new(utf8_text).unwrap().into_raw();
        unsafe {
            c_api::draw_text(self.canvas, font.font, x, y, r, g, b, cstr, kerning_offset);
            let _ = CString::from_raw(cstr);  // free the raw string
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
        let cstr = CString::new(utf8_text).unwrap().into_raw();
        unsafe {
            c_api::vertical_draw_text(self.canvas, font.font, x, y, r, g, b, cstr, kerning_offset);
            let _ = CString::from_raw(cstr);  // free the raw string
        }
    }
}

/*
 * Font
 */

pub struct Font {
    font: *mut c_datatypes::LedFont,
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