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
        for i in 0..16 {
            main_canvas.clear();
            main_canvas.draw_line(i*2, 0, 32 - i*2, 16, 0x80, 0x80, 0x80);
            sleep(Duration::new(0, 250000000));  // 0.25s
        }
    }
}
