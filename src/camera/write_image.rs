use exr::prelude::*;
use std::path::Path;
use crate::common::*;

use image::{RgbImage, Rgb};

pub fn write_png_image(file_path_str: &str, rgb: &Vec<Float>, width: usize, height: usize) {
    let mut img = RgbImage::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            // let r = gamma_correct(rgb[3 * idx + 0]);
            // let g = gamma_correct(rgb[3 * idx + 1]);
            // let b = gamma_correct(rgb[3 * idx + 2]);

            let r = (rgb[3 * idx + 0] * 255.0) as u8;
            let g = (rgb[3 * idx + 1] * 255.0) as u8;
            let b = (rgb[3 * idx + 2] * 255.0) as u8;
            img.put_pixel(x as u32, (height - 1 - y) as u32, Rgb([r, g, b]));
        }
    }

    img.save(file_path_str).expect("Failed to write PNG");
}

pub fn write_exr_image(file_path_str: &str, rgb: Vec<Float>, width: usize, height: usize) {
    // EXR expects the image data as Vec<(f32, f32, f32)> in scanline order (top to bottom)
    let rgb_pixels: Vec<(f32, f32, f32)> = rgb
        .chunks(3)
        .map(|chunk| (chunk[0], chunk[1], chunk[2]))
        .collect();
    
    let file_path = Path::new(file_path_str);
    
    match write_rgb_file(
        file_path,
        width,
        height,
        |x, y| {
            // EXR expects top-down order, so flip y
            let flipped_y = height - 1 - y;
            let index = flipped_y * width + x;
            rgb_pixels[index]
        },
    ) {
        Ok(_) => { println!("Wrote image to file: {}!", file_path_str); },
        Err(e) =>  { println!("Error writing image to file: {:?}", e); }
    }
}
