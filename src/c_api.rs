extern crate libc;

use libc::{c_char, c_int};
use super::c_datatypes::*;


#[link(name = "rgbmatrix")]
extern "C" {

    // --- matrix functions --- //

    pub(crate) fn led_matrix_create_from_options(
        options: *mut LEDMatrixOptions,
        argc: *const c_int,
        argv: *const *const *const c_char,
    ) -> *mut RGBLedMatrix;
    pub(crate) fn led_matrix_create(rows: c_int, chained: c_int, parallel: c_int) -> *mut RGBLedMatrix;
    pub(crate) fn led_matrix_delete(matrix: *mut RGBLedMatrix);
    // pub(crate) fn led_matrix_print_flags(out: *mut FILE);

    pub(crate) fn led_matrix_get_brightness(matrix: *mut RGBLedMatrix) -> u8;
    pub(crate) fn led_matrix_set_brightness(matrix: *mut RGBLedMatrix, brightness: u8);

    pub(crate) fn led_matrix_get_canvas(matrix: *mut RGBLedMatrix) -> *mut LedCanvas;
    pub(crate) fn led_matrix_create_offscreen_canvas(matrix: *mut RGBLedMatrix) -> *mut LedCanvas;
    pub(crate) fn led_matrix_swap_on_vsync(
        matrix: *mut RGBLedMatrix,
        canvas: *mut LedCanvas,
    ) -> *mut LedCanvas;

    // --- canvas functions --- //

    pub(crate) fn led_canvas_get_size(canvas: *const LedCanvas, width: *mut c_int, height: *mut c_int);
    pub(crate) fn led_canvas_set_pixel(canvas: *mut LedCanvas, x: c_int, y: c_int, r: u8, g: u8, b: u8);
    pub(crate) fn led_canvas_clear(canvas: *mut LedCanvas);
    pub(crate) fn led_canvas_fill(canvas: *mut LedCanvas, r: u8, g: u8, b: u8);

    pub(crate) fn draw_text(
        c: *mut LedCanvas,
        font: *mut LedFont,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const c_char,
        kerning_offset: c_int,
    );

    pub(crate) fn vertical_draw_text(
        c: *mut LedCanvas,
        font: *mut LedFont,
        x: c_int,
        y: c_int,
        r: u8,
        g: u8,
        b: u8,
        utf8_text: *const c_char,
        kerning_offset: c_int,
    );

    pub(crate) fn draw_circle(c: *mut LedCanvas, xx: c_int, y: c_int, radius: c_int, r: u8, g: u8, b: u8);
    pub(crate) fn draw_line(
        c: *mut LedCanvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );

    // --- other --- //

    pub(crate) fn load_font(bdf_font_file: *const c_char) -> *mut LedFont;
    pub(crate) fn delete_font(font: *mut LedFont);
}
