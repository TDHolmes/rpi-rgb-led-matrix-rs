use std::thread::sleep;
use std::time::Duration;

use ledmatrix::{Matrix, LEDMatrixOptions};
use ledmatrix::HardwareMapping::AdafruitHatPWM;


fn main() {
    let options = LEDMatrixOptions::new(
        AdafruitHatPWM,
        16, 32,
        2, 0,
        50
    );

    let mut matrix = Matrix::new_from_options(&options);
    let mut main_canvas = matrix.get_canvas();
    let mut aux_canvas = matrix.create_offscreen_canvas();

    while 1 == 1 {
        for x in (0..32).step_by(2) {
            aux_canvas.clear();
            aux_canvas.draw_line(
                x,      0,
                32 - x, 16,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
        for y in 0..16 {
            aux_canvas.clear();
            aux_canvas.draw_line(
                32, y,
                0, 16 - y,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }

        for x in (0..32).step_by(2) {
            aux_canvas.clear();
            aux_canvas.draw_line(
                32 - x, 16,
                x,      0,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
        for y in 0..16 {
            aux_canvas.clear();
            aux_canvas.draw_line(
                0, 16 - y,
                32, y,
                0x80, 0x80, 0x80);
            matrix.swap_canvas_on_vsync(&mut main_canvas, &mut aux_canvas);
            sleep(Duration::from_millis(50));
        }
    }
}
