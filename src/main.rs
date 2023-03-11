use std::process::Command;

use num::{complex::Complex64, ToPrimitive};

use image::imageops::brighten;
use image::ImageBuffer;

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
                    re: (col / zoom) - (offset_x + 0.5),
                    im: (row / zoom) - offset_y,
                },
                765,
            );
            Some(value + (row.abs() + col.abs()).to_u16()?)
        })
        .collect::<Vec<u16>>()
}

fn main() {
    let image_size: u32 = (2 as u32).pow(12);
    let grid = generate_mandelbrot_grid(image_size, 0.0, 0.0, 1.0);
    let mut img = ImageBuffer::new(image_size + 1, image_size + 1);
    // Split number up to 768 into 3 buckets that flow into each other
    let spiller_over = |n: u16| -> [u8; 3] {
        //assert!(n <= 765);
        if n <= 255 {
            [0, n as u8, 0]
        } else if (n >= 255) & (n <= 510) {
            [0, (n - 256) as u8, 255]
        } else {
            [(n - 510) as u8, 255, 255]
        }
    };

    for (i, p) in grid.iter().enumerate() {
        let i = i as u32;
        img.put_pixel(i % image_size, i / image_size, image::Rgb(spiller_over(*p)));
    }

    img = brighten(&img, 10);

    img.save(format!("result_{}.png", image_size)).unwrap();

    Command::new("open").arg(format!("result_{}.png", image_size));
}
