// build.rs

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    // Move to the rpi-rgb-led-matrix library
    let rpi_rgb_path_root = Path::new("src/rpi-rgb-led-matrix/");
    let rpi_rgb_out = rpi_rgb_path_root.join("lib");
    assert!(env::set_current_dir(&rpi_rgb_path_root).is_ok());

    // Make it!
    eprintln!("Making...");
    Command::new("make").status().unwrap();

    println!("cargo:rustc-link-search=native={}", rpi_rgb_out.as_path().display());
    println!("cargo:rustc-link-lib=static=rgbmatrix");
}