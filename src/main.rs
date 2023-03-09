use num::{
    complex::{self, Complex64},
    traits::Pow,
};
use std::f64::consts;

use image::{imageops::brighten, Rgb};
use image::{GrayImage, ImageBuffer, Luma, RgbImage};

use rayon::prelude::*;

const GRID_X: u32 = 16384;
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
#[inline]
fn get_escape_value(c: Complex64, limit: u8) -> u8 {
    let mut value = Complex64 { re: 0.0, im: 0.0 };
    {
        for i in 0..=limit {
            if value.norm_sqr() > 4.0 {
                return i;
            }
            value = value.powu(2) + c;
        }
        0
    }
}

fn main() {
    let mut img: RgbImage = ImageBuffer::new(GRID_X, GRID_Y);

    for cmp_cord in 0..GRID_Y {
        let cmp_cord_v: f64 = (cmp_cord as f64 - (GRID_Y as f64 / 2.0)) / (GRID_Y / 2) as f64;
        //((real_cord as f64 - (GRID_X as f64 / 2.0)) / (GRID_X as f64 / 2.0)) - 0.5;

        let completion_steps: Vec<u8> = (0..GRID_X)
            .into_par_iter()
            .enumerate()
            .map(|(index, _)| {
                let real_cord_val =
                    ((index as f64 - (GRID_X as f64 / 2.0)) / (GRID_X as f64 / 2.0)) - 0.5;
                return get_escape_value(
                    Complex64 {
                        re: real_cord_val,
                        im: cmp_cord_v,
                    },
                    255,
                );
            })
            .collect();

        for (i, val) in completion_steps.iter().enumerate() {
            img.put_pixel(i as u32, cmp_cord, image::Rgb([*val, *val, *val]))
        }

        progress_bar(cmp_cord as i32, GRID_Y as i32)
    }

    img = brighten(&img, 10);

    img.save(format!("result_{}x{}.png", GRID_X, GRID_Y))
        .unwrap();
}
