use std::fmt::format;
use std::process::Command;

use num::traits::Inv;
use num::{complex::Complex64, ToPrimitive};

use image::imageops::{brighten, invert};
use image::{ImageBuffer, RgbImage};

use rayon::prelude::*;

fn get_escape_value(c: Complex64, limit: u16) -> u16 {
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

fn generate_mandelbrot_grid(resolution: u32, offset_x: f64, offset_y: f64, zoom: f64) -> Vec<u16> {
    // disbale offset

    (0..=resolution.pow(2))
        .into_par_iter()
        .filter_map(|i| {
            let i = i.to_u32()?;
            let col = 2.0 * ((i % resolution).to_f64()? / resolution.to_f64()? - 0.5);
            let row = 2.0 * ((i / resolution).to_f64()? / resolution.to_f64()? - 0.5);

            let value = get_escape_value(
                // Convert in to row and column then zoom in
                Complex64 {
                    re: (col / zoom) - offset_x,
                    im: (row / zoom) - offset_y,
                },
                765,
            );
            Some(value)
        })
        .collect::<Vec<u16>>()
}

fn main() {
    let image_size: u32 = (2 as u32).pow(12);
    let grid = generate_mandelbrot_grid(image_size, 0.91, 0.234, 10000.0);
    let mut img: RgbImage = ImageBuffer::new(image_size + 1, image_size + 1);

    // Split number up to 768 into 3 buckets that flow into each other
    let spiller_over = |n: u16| -> [u8; 3] {
        //assert!(n <= 765);
        if n <= 255 {
            [n as u8, 0, 0]
        } else if (n >= 255) & (n <= 510) {
            [255, (n - 256) as u8, 0]
        } else {
            [255, 255, (n - 510) as u8]
        }
    };

    for (i, p) in grid.iter().enumerate() {
        let i = i as u32;
        img.put_pixel(i % image_size, i / image_size, image::Rgb(spiller_over(*p)))
    }

    img = brighten(&img, 10);

    img.save(format!("result_{}.png", image_size)).unwrap();

    Command::new("open").arg(format!("result_{}.png", image_size));
}
