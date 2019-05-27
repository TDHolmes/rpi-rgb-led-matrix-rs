use libc::{c_char, c_int};
use std::ffi::CString;
// use std::path::Path;

use super::matrix;

/*
 * The C LED Matrix API
 */

#[repr(C)]
pub(crate) struct RGBLedMatrix {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct LedCanvas {
    _private: [u8; 0],
}

#[repr(C)]
pub(crate) struct LedFont {
    _private: [u8; 0],
}

/**
 * Parameters to create a new matrix.
 *
 * To get the defaults, non-set values have to be initialized to zero, so you
 * should zero out this struct before setting anything.
 */
#[repr(C)]
pub(crate) struct LEDMatrixOptions {
    /*
     * Name of the hardware mapping used. If passed NULL here, the default
     * is used.
     */
    pub(crate) hardware_mapping: *const c_char,

    /* The "rows" are the number of rows supported by the display, so 32 or 16.
     * Default: 32.
     * Corresponding flag: --led-rows
     */
    pub(crate) rows: c_int,

    /* The "cols" are the number of columns per panel. Typically something
     * like 32, but also 64 is possible. Sometimes even 40.
     * cols * chain_length is the total length of the display, so you can
     * represent a 64 wide display as cols=32, chain=2 or cols=64, chain=1;
     * same thing.
     * Flag: --led-cols
     */
    pub(crate) cols: c_int,

    /* The chain_length is the number of displays daisy-chained together
     * (output of one connected to input of next). Default: 1
     * Corresponding flag: --led-chain
     */
    pub(crate) chain_length: c_int,

    /* The number of parallel chains connected to the Pi; in old Pis with 26
     * GPIO pins, that is 1, in newer Pis with 40 interfaces pins, that can
     * also be 2 or 3. The effective number of pixels in vertical direction is
     * then thus rows * parallel. Default: 1
     * Corresponding flag: --led-parallel
     */
    pub(crate) parallel: c_int,

    /* Set PWM bits used for output. Default is 11, but if you only deal with
     * limited comic-colors, 1 might be sufficient. Lower require less CPU and
     * increases refresh-rate.
     * Corresponding flag: --led-pwm-bits
     */
    pub(crate) pwm_bits: c_int,

    /* Change the base time-unit for the on-time in the lowest
     * significant bit in nanoseconds.
     * Higher numbers provide better quality (more accurate color, less
     * ghosting), but have a negative impact on the frame rate.
     * Corresponding flag: --led-pwm-lsb-nanoseconds
     */
    pub(crate) pwm_lsb_nanoseconds: c_int,

    /* The lower bits can be time-dithered for higher refresh rate.
     * Corresponding flag: --led-pwm-dither-bits
     */
    pub(crate) pwm_dither_bits: c_int,

    /* The initial brightness of the panel in percent. Valid range is 1..100
     * Corresponding flag: --led-brightness
     */
    pub(crate) brightness: c_int,

    /* Scan mode: 0=progressive, 1=interlaced
     * Corresponding flag: --led-scan-mode
     */
    pub(crate) scan_mode: c_int,

    /* Default row address type is 0, corresponding to direct setting of the
     * row, while row address type 1 is used for panels that only have A/B,
     * typically some 64x64 panels
     * Corresponding flag: --led-row-addr-type
     */
    pub(crate) row_address_type: c_int,

    /*  Type of multiplexing. 0 = direct, 1 = stripe, 2 = checker (typical 1:8)
     */
    pub(crate) multiplexing: c_int,

    /* In case the internal sequence of mapping is not "RGB", this contains the
     * real mapping. Some panels mix up these colors.
     * Corresponding flag: --led-rgb-sequence
     */
    pub(crate) led_rgb_sequence: *const c_char,

    /* A string describing a sequence of pixel mappers that should be applied
     * to this matrix. A semicolon-separated list of pixel-mappers with optional
     * parameter.
     * Corresponding flag: --led-pixel-mapper
     */
    pub(crate) pixel_mapper_config: *const c_char,

    /** The following are boolean flags, all off by default **/
    /* Allow to use the hardware subsystem to create pulses. This won't do
     * anything if output enable is not connected to GPIO 18.
     * Corresponding flag: --led-hardware-pulse
     */
    pub(crate) various_bitfield_options: u8, // unsigned disable_hardware_pulsing:1;
                                      // unsigned show_refresh_rate:1;  /* Corresponding flag: --led-show-refresh    */
                                      // // unsigned swap_green_blue:1; /* deprecated, use led_sequence instead */
                                      // unsigned inverse_colors:1;     /* Corresponding flag: --led-inverse         */
}

impl LEDMatrixOptions {
    pub(crate) fn new_from_options(rust_options: &matrix::LEDMatrixOptions) -> LEDMatrixOptions {
        LEDMatrixOptions {
            hardware_mapping: rust_options.mapping.to_string(),

            rows: rust_options.rows as c_int,
            cols: rust_options.cols as c_int,
            chain_length: rust_options.chain_length as c_int,
            parallel: rust_options.parallel as c_int,

            pwm_bits: rust_options.pwm_bits as c_int,
            pwm_lsb_nanoseconds: rust_options.pwm_lsb_nanoseconds as c_int,
            pwm_dither_bits: rust_options.pwm_dither_bits as c_int,

            brightness: rust_options.brightness as c_int,
            scan_mode: rust_options.scan_mode as c_int,
            row_address_type: rust_options.row_address_type as c_int,
            multiplexing: rust_options.multiplexing as c_int,

            led_rgb_sequence: rust_options.led_rgb_sequence.to_string(),
            pixel_mapper_config: CString::new("").unwrap().into_raw(),
            various_bitfield_options: 0,
        }
    }
}