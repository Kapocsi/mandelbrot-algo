use num::{
    complex::{self, Complex64},
    traits::Pow,
};
use std::f64::consts;

use image::imageops::brighten;
use image::{GrayImage, ImageBuffer, Luma, RgbImage};

const GRID_X: u32 = 4096;
const GRID_Y: u32 = GRID_X;
const scale_factore: f64 = GRID_X as f64 / 4.0;

fn progress_bar(current: i32, max: i32) {
    let progress: f64 = (current as f64 / max as f64) * 100.0;
    print!(
        "\r{}{}",
        (0..(progress as i32 / 2)).map(|_| "=").collect::<String>(),
        (0..(50 - (progress as i32 / 2)))
            .map(|_| " ")
            .collect::<String>()
    );
    print!(" {}%", progress);
    if progress == 100.0 {
        println!("");
    }
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(GRID_X, GRID_Y);

    for cmp_cord in 0..GRID_Y {
        let cmp_cord_v: f64 = (cmp_cord as f64 - (GRID_Y as f64 / 2.0)) / (GRID_Y / 2) as f64;
        for real_cord in 0..GRID_X {
            let real_cord_v: f64 =
                ((real_cord as f64 - (GRID_X as f64 / 2.0)) / (GRID_X as f64 / 2.0)) - 0.5;
            let mut val = complex::Complex64::new(0.0, 0.0);
            let mut result: u8 = 0;
            for i in 0..=55 {
                if val.norm_sqr() > 4.0 {
                    result = i;
                    break;
                }
                val = val.powu(2) + Complex64::new(real_cord_v, cmp_cord_v);
            }

            // Create a colored pixel based on a gradient
            let red = (result as f64 / 40.0 * 255.0) as u8;
            let green = ((result as f64 / 40.0) * (result as f64 / 40.0) * 255.0) as u8;
            let blue =
                ((result as f64 / 40.0) * (result as f64 / 40.0) * (result as f64 / 40.0) * 255.0)
                    as u8;

            img.put_pixel(real_cord, cmp_cord, image::Rgb([red, green, blue]));
        }
        progress_bar(cmp_cord as i32, GRID_Y as i32)
    }

    img = brighten(&img, 10);

    img.save(format!("result_{}x{}.png", GRID_X, GRID_Y))
        .unwrap();
}
