use super::c_api;
use super::c_datatypes;

use std::path::Path;
use libc::c_int;
use std::ffi::CString;
use rgb::RGB8;


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

    /// Gets the total size of the canvas, taking into account the number
    /// of parallel and series panels you have.
    pub fn get_size(&self) -> (i32, i32) {
        let mut width: c_int = 0;
        let mut height: c_int = 0;

        unsafe {
            c_api::led_canvas_get_size(self.canvas, &mut width as *mut c_int, &mut height as *mut c_int);
        }

        return (width as i32, height as i32);
    }

    pub fn clear(&mut self) {
        unsafe {
            c_api::led_canvas_clear(self.canvas);
        }
    }

    pub fn fill(&mut self, rgb: &RGB8) {
        unsafe {
            c_api::led_canvas_fill(self.canvas, rgb.r, rgb.g, rgb.b);
        }
    }

    pub fn set_pixel(&mut self, pixel: &PixelLocation, rgb: &RGB8) {
        unsafe {
            c_api::led_canvas_set_pixel(self.canvas, pixel.x, pixel.y, rgb.r, rgb.g, rgb.b);
        }
    }

    pub fn draw_circle(&mut self, pixel: &PixelLocation, radius: i32, rgb: &RGB8) {
        unsafe {
            c_api::draw_circle(self.canvas, pixel.x, pixel.y, radius, rgb.r, rgb.g, rgb.b);
        }
    }

    pub fn draw_line(&mut self, p0: &PixelLocation, p1: &PixelLocation, rgb: &RGB8) {
        unsafe {
            c_api::draw_line(self.canvas, p0.x, p0.y, p1.x, p1.y, rgb.r, rgb.g, rgb.b);
        }
    }

    pub fn draw_text(
        &mut self,
        font: &mut Font,
        pixel_start: &PixelLocation,
        rgb: &RGB8,
        utf8_text: &str,
        kerning_offset: i32,
    ) {
        let cstr = CString::new(utf8_text).unwrap().into_raw();
        unsafe {
            c_api::draw_text(
                self.canvas,
                font.font,
                pixel_start.x,
                pixel_start.y,
                rgb.r,
                rgb.g,
                rgb.b,
                cstr,
                kerning_offset
            );
            let _ = CString::from_raw(cstr);  // free the raw string
        }
    }

    pub fn vertical_draw_text(
        &mut self,
        font: &mut Font,
        pixel_start: &PixelLocation,
        rgb: &RGB8,
        utf8_text: &str,
        kerning_offset: i32,
    ) {
        let cstr = CString::new(utf8_text).unwrap().into_raw();
        unsafe {
            c_api::vertical_draw_text(
                self.canvas,
                font.font,
                pixel_start.x,
                pixel_start.y,
                rgb.r,
                rgb.g,
                rgb.b,
                cstr,
                kerning_offset
            );
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

/*
 * Pixel Location
 */

pub struct PixelLocation {
    pub x: i32,
    pub y: i32,
}

impl PixelLocation {
    pub fn from_relative(x: f32, y: f32, canvas: &Canvas) -> PixelLocation {
        let (xsize, ysize) = canvas.get_size();

        PixelLocation {
            x: (xsize as f32 * x) as i32,
            y: (ysize as f32 * y) as i32,
        }
    }
}