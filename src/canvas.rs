use super::c_api;
use super::c_datatypes;

use std::path::Path;
use libc::c_int;
use std::ffi::CString;

use rgb::*;


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

    pub fn draw_line_antialiased(&mut self, pp0: &PixelLocation, pp1: &PixelLocation, rgb: &RGB8) {
        // integer part of x
        fn ipart(x: f32) -> f32 {
            x.floor()
        }

        fn round(x: f32) -> f32 {
            ipart(x + 0.5)
        }

        // fractional part of x
        fn fpart(x: f32) -> f32 {
            x - x.floor()
        }

        fn rfpart(x: f32) -> f32 {
            1.0 - fpart(x)
        }

        let mut p0: PixelLocation = pp0.clone();
        let mut p1: PixelLocation = pp1.clone();
        let mut pswap = PixelLocation{x: 0, y: 0};

        let steep = (p1.y as f32 - p0.y as f32).abs() > (p1.x as f32 - p0.x as f32).abs();

        if steep {
            // swap x & y on p0
            pswap.x = p0.x;
            p0.x = p0.y;
            p0.y = pswap.x;

            // swap x & y on p1
            pswap.x = p1.x;
            p1.x = p1.y;
            p1.y = pswap.x;
        }

        if p0.x > p1.x {
            // swap x cords for p0 & p1
            pswap.x = p0.x;
            p0.x = p1.x;
            p0.x = pswap.x;

            // swap y cords for p0 & p1
            pswap.y = p0.y;
            p0.y = p1.y;
            p0.y = pswap.y;
        }

        let dx = (p1.x - p0.x) as f32;
        let dy = (p1.y - p0.y) as f32;
        let mut gradient = dy / dx;
        if dx == 0.0 {
            gradient = 1.0;
        }

        // handle first endpoint
        let xend = round(p0.x as f32);
        let yend = p0.y as f32 + gradient * (xend - p0.x as f32);
        let xgap = rfpart(p0.x as f32 + 0.5);
        let xpxl1 = xend as i32; // this will be used in the main loop;
        let ypxl1 = yend.floor() as i32;
        print!("({},{}) -> ", xpxl1, ypxl1);
        if steep {
            let pixel = PixelLocation{x: ypxl1, y: xpxl1};
            let _rgb = rgb.map(|px| (px as f32 * rfpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);

            let pixel = PixelLocation{x: ypxl1 + 1, y: xpxl1};
            let _rgb = rgb.map(|px| (px as f32 * fpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);
        } else {
            let pixel = PixelLocation{x: xpxl1, y: ypxl1};
            let _rgb = rgb.map(|px| (px as f32 * rfpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);

            let pixel = PixelLocation{x: xpxl1, y: ypxl1 + 1};
            let _rgb = rgb.map(|px| (px as f32 * fpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);
        }
        let mut intery = yend + gradient; // first y-intersection for the main loop;

        // handle second endpoint
        let xend = p1.x as f32;
        let yend = p1.y as f32 + gradient * (xend - p1.x as f32);
        let xgap = fpart(p1.x as f32 + 0.5);
        let xpxl2 = xend as i32;         // this will be used in the main loop;
        let ypxl2 = yend.floor() as i32;
        println!("({},{})", xpxl2, ypxl2);
        if steep {
            let pixel = PixelLocation{x: ypxl2, y: xpxl2};
            let _rgb = rgb.map(|px| (px as f32 * rfpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);

            let pixel = PixelLocation{x: ypxl2 + 1, y: xpxl2};
            let _rgb = rgb.map(|px| (px as f32 * fpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);
        } else {
            let pixel = PixelLocation{x: xpxl2, y: ypxl2};
            let _rgb = rgb.map(|px| (px as f32 * rfpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);

            let pixel = PixelLocation{x: xpxl2, y: ypxl2 + 1};
            let _rgb = rgb.map(|px| (px as f32 * fpart(yend) * xgap) as u8);
            self.set_pixel(&pixel, &_rgb);
        }

        // main loop
        let mut pix = PixelLocation{ x: 0, y: 0 };
        if steep {
            for x in xpxl1 + 1..xpxl2 - 1 {
                pix.y = x;
                pix.x = intery.floor() as i32;
                self.set_pixel(&pix, &rgb.map(|pix| ((pix as f32) * rfpart(intery)) as u8));

                pix.x = (intery.floor() as i32) + 1;
                self.set_pixel(&pix, &rgb.map(|pix| ((pix as f32) * fpart(intery)) as u8));
                intery = intery + gradient;
            }
        } else {
            for x in xpxl1 + 1..xpxl2 - 1 {
                pix.x = x;
                pix.y = intery.floor() as i32;
                self.set_pixel(&pix, &rgb.map(|pix| ((pix as f32) * rfpart(intery)) as u8));

                pix.y = (intery.floor() as i32) + 1;
                self.set_pixel(&pix, &rgb.map(|pix| ((pix as f32) * fpart(intery)) as u8));
                intery = intery + gradient;
            }
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

#[derive(Copy, Clone)]
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