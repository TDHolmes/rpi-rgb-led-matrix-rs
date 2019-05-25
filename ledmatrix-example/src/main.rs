use std::thread::sleep;
use std::time::Duration;

use ledmatrix::{Matrix, LEDMatrixOptions};
use ledmatrix::HardwareMapping::AdafruitHatPWM;


fn main() {
    let options = LEDMatrixOptions::new(
        AdafruitHatPWM,
        16, 32,
        1, 0,
        50
    );

    let mut matrix = Matrix::new_from_options(&options);
    let mut main_canvas = matrix.get_canvas();

    while 1 == 1 {
        for x in 0..16 {
            main_canvas.clear();
            main_canvas.draw_line(
                x*2,      0,
                32 - x*2, 16,
                0x80, 0x80, 0x80);
            sleep(Duration::new(0, 100000000));  // 0.1s
        }
        for y in 0..16 {
            main_canvas.clear();
            main_canvas.draw_line(
                32, y,
                0, 16 - y,
                0x80, 0x80, 0x80);
            sleep(Duration::new(0, 100000000));  // 0.1s
        }

        for x in 0..16 {
            main_canvas.clear();
            main_canvas.draw_line(
                32 - x*2, 16,
                x*2, 0,
                0x80, 0x80, 0x80);
            sleep(Duration::new(0, 100000000));  // 0.1s
        }
        for y in 0..16 {
            main_canvas.clear();
            main_canvas.draw_line(
                0, 16 - y,
                32, y,
                0x80, 0x80, 0x80);
            sleep(Duration::new(0, 100000000));  // 0.1s
        }
    }
}
