use std::thread::sleep;
use std::time::Duration;
use std::path::Path;
use rgb::*;

use ledmatrix::matrix::{Matrix, LEDMatrixOptions};
use ledmatrix::canvas::{Font, PixelLocation};
use ledmatrix::matrix::HardwareMapping::AdafruitHatPWM;


fn main() {
    const PANEL_ROWS: i32 = 16;
    const PANEL_COLS: i32 = 32;
    const NUM_PANELS: i32 = 2;

    let rgb_hello = RGB8::new(0x80, 0x80, 0x80);
    let rgb_goodbye = rgb_hello.clone().map(|px| px / 2);

    let options = LEDMatrixOptions::new(
        AdafruitHatPWM,
        PANEL_ROWS, PANEL_COLS,
        NUM_PANELS, 0,
        50
    );

    let mut matrix = Matrix::new_from_options(&options);
    let mut main_canvas = matrix.get_canvas();
    let mut aux_canvas = matrix.create_offscreen_canvas();

    let file_path = Path::new(file!()).parent().unwrap();
    let font_path_buf = file_path.join(Path::new("../rpi-rgb-led-matrix/fonts/5x8.bdf"));
    let mut font = Font::new(font_path_buf.as_path()).unwrap();

    loop {
        for step_micro in -300..200 {
            let step: f32 = (step_micro as f32) / 100.0;

            let goodbye_pos = PixelLocation::from_relative(0.25, step * 2.0 + 0.5, &main_canvas);
            let hello_pos = PixelLocation::from_relative(step, 0.5, &main_canvas);

            main_canvas.clear();
            main_canvas.draw_text(&mut font, &hello_pos, &rgb_hello, "Hello, World!", 0);
            aux_canvas.vertical_draw_text(&mut font, &goodbye_pos, &rgb_goodbye, "Goodbye", 0);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
    }
}
