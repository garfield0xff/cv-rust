
pub mod image;
pub mod descriptors;
pub mod detectors;
pub mod harris;

use harris::Harris;
use ::image::{Luma};
use log::info;
use pretty_env_logger::init;
use crate::{image::GrayFloatImage};

fn main() {
    pretty_env_logger::init();


    let img_path = "image_path";
    let mut gray_image = GrayFloatImage::load_image(&img_path);
    let output_path = "harris_output__test.png";

    let corners = Harris::corner_detector(&gray_image, 2, 0.04, 20.0);
    draw_corners(&mut gray_image, &corners);

    let u8_image = gray_image.to_u8_image();
    u8_image.save(output_path).unwrap();


}

fn draw_corners(img: &mut GrayFloatImage, corners: &[(usize, usize)]) -> GrayFloatImage{

    for y in 0..img.height() {
        for x in 0..img.width() {
            img.put_pixel(x as u32, y as u32, Luma([0.0]));
        }
    }


    for &(x, y) in corners {
        if x < img.width() as usize && y < img.height() as usize {
            img.put_pixel(x as u32, y as u32, Luma([1.0]));
        }
    }

    img.clone()
}


