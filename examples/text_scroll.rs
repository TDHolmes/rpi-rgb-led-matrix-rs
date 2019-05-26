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
    let file_path = Path::new(file!()).parent().unwrap();
    let font_path_buf = file_path.join(Path::new("../rpi-rgb-led-matrix/fonts/5x8.bdf"));
    let mut font = Font::new(font_path_buf.as_path()).unwrap();

    loop {
        for x in -3*COLS..COLS*2 {
            let y = 8;
            let (r, g, b) = (50, 50, 50);

            main_canvas.clear();
            main_canvas.draw_text(&mut font, x, y, r, g, b, "Hello, World!", 0);
            aux_canvas.vertical_draw_text(&mut font, 4, x*2 + COLS / 2, 25, 25, 25, "Goodbye", 0);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
    }
}
