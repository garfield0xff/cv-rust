
pub mod image;
pub mod descriptors;
pub mod detectors;
pub mod harris;

use ::image::{Luma};
use crate::{image::GrayFloatImage};

fn main() {


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


