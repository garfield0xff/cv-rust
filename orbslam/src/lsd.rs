use std::f32::consts::PI;
use image::imageops::FilterType;
use imageproc::drawing::Canvas;
use ndarray::Array2;
use crate::image::{gaussian_blur, sobel_filter_x, sobel_filter_y, GrayFloatImage};

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: usize,
    pub y: usize
}

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

pub fn new_lsd_detector(image: &GrayFloatImage, threshold: f32) -> Vec<(Point, Point)> {
    let gaussian_image = gaussian_blur(image, 2.0);

    let scaled_image = scale_image(&gaussian_image, 0.8); 

    let i_x = sobel_filter_x(&scaled_image);
    let i_y = sobel_filter_y(&scaled_image);

    let (magnitude, angle) = gradient_magnitude_direction(&i_x, &i_y);

    let (rows, cols) = (magnitude.shape()[0] ,magnitude.shape()[1]) ;

    let mut clusters = Array2::<i32>::zeros((rows, cols));

    let mut cluster_id = 1;
    for y in 0..rows {
        for x in 0..cols {
            if magnitude[[y, x]] > threshold && clusters[[y, x]] == 0 {
                cluster_by_gradient_direction_single(&angle, &mut clusters, x as i32, y as i32, cluster_id, 8_f32.to_radians());
                cluster_id += 1;
            }

        }
    }

    let lines = extract_line_candidates(&clusters);

    lines
}

fn cluster_by_gradient_direction_single(
    angle: &Array2<f32>, 
    clusters: &mut Array2<i32>, 
    x: i32, 
    y: i32, 
    cluster_id: i32, 
    tolerance: f32
) {
    let rows = angle.shape()[0] as i32;
    let cols = angle.shape()[1] as i32;
    
    let mut region = vec![(x, y)];
    let pixel_angle = angle[[y as usize, x as usize]];

    while let Some((px, py)) = region.pop() {
        if clusters[[py as usize, px as usize]] == 0 {
            clusters[[py as usize, px as usize]] = cluster_id;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx != 0 || dy != 0 {
                        let nx = px + dx;
                        let ny = py + dy;
                        if nx >= 0 && nx < cols && ny >= 0 && ny < rows {
                            let neighbor_angle = angle[[ny as usize, nx as usize]];
                            if (neighbor_angle - pixel_angle).abs() <= tolerance && clusters[[ny as usize, nx as usize]] == 0 {
                                region.push((nx, ny));
                            }
                        }
                    }
                }
            }
        }
    }
}


fn gradient_magnitude_direction(i_x: &Array2<f32>, i_y: &Array2<f32>) -> (Array2<f32>, Array2<f32>) {
    let (height, width) = (i_x.shape()[0], i_x.shape()[1]);
    let mut magnitude = Array2::<f32>::zeros((height, width));
    let mut direction = Array2::<f32>::zeros((height, width));

    for y in 0..height {
        for x in 0..width {
            let gx = i_x[[y, x]];
            let gy = i_y[[y, x]];
            magnitude[[y, x]] = (gx * gx + gy * gy).sqrt();
            direction[[y, x]] = (gy.atan2(gx) * 180.0 / PI).abs();
        }
    }

    (magnitude, direction)
}

fn scale_image(image: &GrayFloatImage, scale: f32) -> GrayFloatImage {
    let scaled_width = (image.width() as f32 * scale) as u32;
    let scaled_height = (image.height() as f32 * scale) as u32;
    let resized_image = image::imageops::resize(&image.0, scaled_width, scaled_height, FilterType::Lanczos3);
    GrayFloatImage(resized_image)
}

fn fit_line(points: &[Point]) -> (Point, Point) {
    let start = points[0];
    let end = points[points.len() - 1];
    (start, end)
}

fn extract_line_candidates(clusters: &Array2<i32>) -> Vec<(Point, Point)> {
    let rows = clusters.shape()[0];
    let cols = clusters.shape()[1];
    let mut lines = Vec::new();

    let max_cluster_id = *clusters.iter().max().unwrap();

    for cluster_id in 1..=max_cluster_id {
        let mut points = Vec::new();
        for y in 0..rows {
            for x in 0..cols {
                if clusters[[y, x]] == cluster_id {
                    points.push(Point { x, y });
                }
            }
        }

        if points.len() > 1 {
            let line = fit_line(&points);
            println!("fit line for cluster_id {}: {:?}", cluster_id, line);
            lines.push(line);
        }
    }
    lines
}
