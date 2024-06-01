use std::f32::consts::PI;
use ndarray::{Array, Array2, ArrayView2};
use crate::image::{sobel_filter_x, sobel_filter_y, GrayFloatImage};

pub fn lsd_detector(image: &GrayFloatImage, threshold: f32) -> Vec<(usize, usize, f32, f32)>{
    let i_x = sobel_filter_x(&image);
    let i_y  = sobel_filter_y(&image);

    let (magnitude, direction) = gradient_magnitude_direction(&i_x, &i_y);

    let mut lines = Vec::new();

    for y in 0..magnitude.shape()[0] {
        for x in 0..magnitude.shape()[1] {
            let mag = magnitude[[y, x]];
            if mag > threshold {
                let dir = direction[[y, x]];
                lines.push((x, y, mag, dir));
            }
        }
     }
    
    lines
    
}

fn gradient_magnitude_direction(i_x: &Array2<f32>, i_y: &Array2<f32>) -> (Array2<f32>, Array2<f32>) {
    let (height, width) = (i_x.shape()[0], i_x.shape()[1]);
    let mut magnitue = Array2::<f32>::zeros((height, width));
    let mut direction = Array2::<f32>::zeros((height, width));

    for y in 0..height {
        for x in 0..width {
            let gx = i_x[[y, x]];
            let gy = i_y[[y, x]];
            magnitue[[y, x]] = (gx * gx + gy * gy).sqrt();
            direction[[y, x]] = (gy.atan2(gx) * 180.0 / PI).abs();
        }
    }

    (magnitue, direction)
}