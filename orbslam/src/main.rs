
pub mod image;
pub mod descriptors;
pub mod detectors;
pub mod harris;
pub mod lsd;

use harris::Harris;
use lsd::lsd_detector;

use std::time::Instant;
use ::image::{open, Rgb, RgbImage};
use log::info;
use crate::{image::GrayFloatImage};

fn main() {
    
        pretty_env_logger::init();
    
        let img_path = "img_path";

        let mut gray_image = open(&img_path).expect("failed to load image").to_luma8();
        let gray_float_image = GrayFloatImage::load_image(&img_path);

        let start = Instant::now();
        let lines = lsd_detector(&gray_float_image,  1.5);
        draw_lines(&gray_float_image, lines, "output.png");
        info!("lsd detector response in : {:?}", start.elapsed());

        let mut rgb_img = RgbImage::new(gray_image.width(), gray_image.height());
        for (x, y, pixel) in gray_image.enumerate_pixels() {
            let rgb_pixel = Rgb([pixel[0], pixel[0], pixel[0]]);
            rgb_img.put_pixel(x, y, rgb_pixel);
        }
    
}

fn draw_lines(image: &GrayFloatImage, lines: Vec<(usize, usize, f32, f32)>, output_path: &str) {
    let mut rgb_image = RgbImage::new(image.width() as u32, image.height() as u32);

    for (x, y, _mag, _dir) in lines {
        let color = Rgb([255, 0, 0]); // 빨간색
        if x < rgb_image.width() as usize && y < rgb_image.height() as usize {
            rgb_image.put_pixel(x as u32, y as u32, color);
        }
    }

    rgb_image.save(output_path).unwrap();
}

fn draw_corners(img: &mut GrayFloatImage, corners: &[(usize, usize)]) -> RgbImage{
    let corner_size = 5;

    let mut rgb_image = RgbImage::new(img.width() as u32, img.height() as u32);
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel_value = img.get(x, y);
            let rgb_value = (pixel_value * 255.0) as u8;
            rgb_image.put_pixel(x as u32, y as u32, Rgb([0, 0, 0]));
        }
    }


    for &(x, y) in corners {
        if x < img.width() && y < img.height() {
            for dy in 0..corner_size {
                for dx in 0..corner_size {
                    let nx = x as u32 + dx - corner_size / 2;
                    let ny = y as u32 + dy - corner_size / 2;
                    if nx < img.width() as u32 && ny < img.height() as u32 {
                        rgb_image.put_pixel(nx, ny, Rgb([255, 0, 0]));
                    }
                }
            }
        }
    }
    

    rgb_image
}



