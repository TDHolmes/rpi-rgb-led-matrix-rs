use std::thread::sleep;
use std::time::Duration;

use ledmatrix::{Matrix, LEDMatrixOptions};
use ledmatrix::HardwareMapping::AdafruitHatPWM;


fn main() {
    let PANEL_ROWS: i32 = 16;
    let PANEL_COLS: i32 = 32;
    let NUM_PANELS: i32 = 2;

    let ROWS: i32 = PANEL_ROWS * NUM_PANELS;
    let COLS: i32 = 32;

    let options = LEDMatrixOptions::new(
        AdafruitHatPWM,
        PANEL_ROWS, PANEL_COLS,
        NUM_PANELS, 0,
        50
    );

    let mut matrix = Matrix::new_from_options(&options);
    let mut main_canvas = matrix.get_canvas();
    let mut aux_canvas = matrix.create_offscreen_canvas();

    while 1 == 1 {
        for x in 1..=COLS {
            aux_canvas.clear();
            aux_canvas.draw_line(
                x,        0,
                COLS - x, ROWS,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
        for y in 1..=ROWS {
            aux_canvas.clear();
            aux_canvas.draw_line(
                COLS, y,
                0,    ROWS - y,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }

        for x in 1..=COLS {
            aux_canvas.clear();
            aux_canvas.draw_line(
                COLS - x, ROWS,
                x,        0,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
        for y in 1..=ROWS {
            aux_canvas.clear();
            aux_canvas.draw_line(
                0,    ROWS - y,
                COLS, y,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
    }
}
