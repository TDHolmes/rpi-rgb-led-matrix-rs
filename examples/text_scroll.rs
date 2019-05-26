use std::thread::sleep;
use std::time::Duration;
use std::path::Path;

use ledmatrix::{Matrix, LEDMatrixOptions, Font};
use ledmatrix::HardwareMapping::AdafruitHatPWM;


fn main() {
    const PANEL_ROWS: i32 = 16;
    const PANEL_COLS: i32 = 32;
    const NUM_PANELS: i32 = 2;

    const COLS: i32 = 32;

    let options = LEDMatrixOptions::new(
        AdafruitHatPWM,
        PANEL_ROWS, PANEL_COLS,
        NUM_PANELS, 0,
        50
    );

    let mut matrix = Matrix::new_from_options(&options);
    let mut main_canvas = matrix.get_canvas();
    let mut aux_canvas = matrix.create_offscreen_canvas();
    let mut font = Font::new(Path::new("../rpi-rgb-led-matrix/fonts/5x8.bdf")).unwrap();

    loop {
        for x in 0..COLS {
            let y = 1;
            let (r, g, b) = (50, 50, 50);

            aux_canvas.clear();
            aux_canvas.draw_text(&mut font, x, y, r, g, b, "Hello, World!", 1);
            aux_canvas.vertical_draw_text(&mut font, 4, x, 25, 25, 25, "Goodbye", 1);
            matrix.swap_canvas_on_vsync(&mut aux_canvas, &mut main_canvas);
            sleep(Duration::from_millis(50));
        }
    }
}
