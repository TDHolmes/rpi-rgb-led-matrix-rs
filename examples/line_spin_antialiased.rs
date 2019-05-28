use std::thread::sleep;
use std::time::Duration;
use rgb::RGB8;

use ledmatrix::matrix::{Matrix, LEDMatrixOptions};
use ledmatrix::matrix::HardwareMapping::AdafruitHatPWM;
use ledmatrix::canvas::PixelLocation;


fn main() {
    const PANEL_ROWS: i32 = 16;
    const PANEL_COLS: i32 = 32;
    const NUM_PANELS: i32 = 2;

    const ROWS: i32 = PANEL_ROWS * NUM_PANELS;
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

    let mut p0 = PixelLocation {x: 0, y: 0};
    let mut p1 = PixelLocation {x: 0, y: 0};
    let rgb = RGB8 {r: 0x80, g: 0x80, b: 0x80};

    p0.y = 0;
    p1.y = ROWS;
    while 1 == 1 {
        for x in 1..=COLS {
            // moves lines
            p0.x = x;
            p1.x = COLS - x;

            // Draw and sleep
            aux_canvas.clear();
            aux_canvas.draw_line_antialiased(&p0, &p1, &rgb);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(25));
        }

        for y in 1..=ROWS {
            // moves lines
            p0.y = y;
            p1.y = ROWS - y;

            // Draw and sleep
            aux_canvas.clear();
            aux_canvas.draw_line_antialiased(&p0, &p1, &rgb);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(25));
        }

        for x in 1..=COLS {
            // moves lines
            p0.x = COLS - x;
            p1.x = x;

            // Draw and sleep
            aux_canvas.clear();
            aux_canvas.draw_line_antialiased(&p0, &p1, &rgb);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(25));
        }
        for y in 1..=ROWS {
            // moves lines
            p0.y = ROWS - y;
            p1.y = y;

            // Draw and sleep
            aux_canvas.clear();
            aux_canvas.draw_line_antialiased(&p0, &p1, &rgb);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(25));
        }
    }
}
