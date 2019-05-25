extern crate libc;

use libc::{c_char, c_int};


/*
 * The C LED Matrix API
 */

#[repr(C)]
pub struct RGBLedMatrix {
    _private: [u8; 0],
}

#[repr(C)]
pub struct LedCanvas {
    _private: [u8; 0],
}

#[repr(C)]
pub struct LedFont {
    _private: [u8; 0],
}

/**
 * Parameters to create a new matrix.
 *
 * To get the defaults, non-set values have to be initialized to zero, so you
 * should zero out this struct before setting anything.
 */
#[repr(C)]
pub struct RGBLedMatrixOptions {
    /*
     * Name of the hardware mapping used. If passed NULL here, the default
     * is used.
     */
    pub hardware_mapping: *const c_char,

    /* The "rows" are the number of rows supported by the display, so 32 or 16.
     * Default: 32.
     * Corresponding flag: --led-rows
     */
    pub rows: c_int,

    /* The "cols" are the number of columns per panel. Typically something
     * like 32, but also 64 is possible. Sometimes even 40.
     * cols * chain_length is the total length of the display, so you can
     * represent a 64 wide display as cols=32, chain=2 or cols=64, chain=1;
     * same thing.
     * Flag: --led-cols
     */
    pub cols: c_int,

    /* The chain_length is the number of displays daisy-chained together
     * (output of one connected to input of next). Default: 1
     * Corresponding flag: --led-chain
     */
    pub chain_length: c_int,

    /* The number of parallel chains connected to the Pi; in old Pis with 26
     * GPIO pins, that is 1, in newer Pis with 40 interfaces pins, that can
     * also be 2 or 3. The effective number of pixels in vertical direction is
     * then thus rows * parallel. Default: 1
     * Corresponding flag: --led-parallel
     */
    pub parallel: c_int,

    /* Set PWM bits used for output. Default is 11, but if you only deal with
     * limited comic-colors, 1 might be sufficient. Lower require less CPU and
     * increases refresh-rate.
     * Corresponding flag: --led-pwm-bits
     */
    pub pwm_bits: c_int,

    /* Change the base time-unit for the on-time in the lowest
     * significant bit in nanoseconds.
     * Higher numbers provide better quality (more accurate color, less
     * ghosting), but have a negative impact on the frame rate.
     * Corresponding flag: --led-pwm-lsb-nanoseconds
     */
    pub pwm_lsb_nanoseconds: c_int,

    /* The lower bits can be time-dithered for higher refresh rate.
     * Corresponding flag: --led-pwm-dither-bits
     */
    pub pwm_dither_bits: c_int,

    /* The initial brightness of the panel in percent. Valid range is 1..100
     * Corresponding flag: --led-brightness
     */
    pub brightness: c_int,

    /* Scan mode: 0=progressive, 1=interlaced
     * Corresponding flag: --led-scan-mode
     */
    pub scan_mode: c_int,

    /* Default row address type is 0, corresponding to direct setting of the
     * row, while row address type 1 is used for panels that only have A/B,
     * typically some 64x64 panels
     * Corresponding flag: --led-row-addr-type
     */
    pub row_address_type: c_int,

    /*  Type of multiplexing. 0 = direct, 1 = stripe, 2 = checker (typical 1:8)
     */
    pub multiplexing: c_int,

    /* In case the internal sequence of mapping is not "RGB", this contains the
     * real mapping. Some panels mix up these colors.
     * Corresponding flag: --led-rgb-sequence
     */
    pub led_rgb_sequence: *const c_char,

    /* A string describing a sequence of pixel mappers that should be applied
     * to this matrix. A semicolon-separated list of pixel-mappers with optional
     * parameter.
     * Corresponding flag: --led-pixel-mapper
     */
    pub pixel_mapper_config: *const c_char,

    /** The following are boolean flags, all off by default **/
    /* Allow to use the hardware subsystem to create pulses. This won't do
     * anything if output enable is not connected to GPIO 18.
     * Corresponding flag: --led-hardware-pulse
     */
    pub various_bitfield_options: u8, // unsigned disable_hardware_pulsing:1;
                                      // unsigned show_refresh_rate:1;  /* Corresponding flag: --led-show-refresh    */
                                      // // unsigned swap_green_blue:1; /* deprecated, use led_sequence instead */
                                      // unsigned inverse_colors:1;     /* Corresponding flag: --led-inverse         */
}

#[link(name = "rgbmatrix")]
extern "C" {
    // matrix functions

    pub fn led_matrix_create_from_options(
        options: *mut RGBLedMatrixOptions,
        argc: *const c_int,
        argv: *const *const c_char,
    ) -> *mut RGBLedMatrix;
    pub fn led_matrix_create(rows: c_int, chained: c_int, parallel: c_int) -> *mut RGBLedMatrix;
    pub fn led_matrix_delete(matrix: *mut RGBLedMatrix);
    // pub fn led_matrix_print_flags(out: *mut FILE);

    pub fn led_matrix_get_brightness(matrix: *mut RGBLedMatrix) -> u8;
    pub fn led_matrix_set_brightness(matrix: *mut RGBLedMatrix, brightness: u8);

    // get canvas from matrix
    pub fn led_matrix_get_canvas(matrix: *mut RGBLedMatrix) -> *mut LedCanvas;
    pub fn led_matrix_create_offscreen_canvas(matrix: *mut RGBLedMatrix) -> *mut LedCanvas;
    // double buffer swap of canvas
    pub fn led_matrix_swap_on_vsync(
        matrix: *mut RGBLedMatrix,
        canvas: *mut LedCanvas,
    ) -> *mut LedCanvas;

    // canvas functions

    pub fn led_canvas_get_size(canvas: *const LedCanvas, width: *mut c_int, height: *mut c_int);
    pub fn led_canvas_set_pixel(canvas: *mut LedCanvas, x: c_int, y: c_int, r: u8, g: u8, b: u8);
    pub fn led_canvas_clear(canvas: *mut LedCanvas);
    pub fn led_canvas_fill(canvas: *mut LedCanvas, r: u8, g: u8, b: u8);

    pub fn draw_text(
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

    pub fn vertical_draw_text(
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

    pub fn draw_circle(c: *mut LedCanvas, xx: c_int, y: c_int, radius: c_int, r: u8, g: u8, b: u8);
    pub fn draw_line(
        c: *mut LedCanvas,
        x0: c_int,
        y0: c_int,
        x1: c_int,
        y1: c_int,
        r: u8,
        g: u8,
        b: u8,
    );

    // other

    pub fn load_font(bdf_font_file: *const c_char) -> *mut LedFont;
    pub fn delete_font(font: *mut LedFont);
}
