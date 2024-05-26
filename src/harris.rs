use ndarray::{Array2};

use crate::image::{sobel_filter_x, sobel_filter_y, GrayFloatImage};

pub struct Harris();

impl Harris {
    pub fn corner_detector(image: &GrayFloatImage, window_size: usize, k: f32, threshold: f32) -> Vec<(usize, usize)> {
    
        let i_x = sobel_filter_x(image);
        let i_y = sobel_filter_y(image);

        let i_xx = &i_x * &i_x;
        let i_yy = &i_y * &i_y;
        let i_xy = &i_x * &i_y;

        let integral_xx = integral_image(&i_xx);
        let integral_xy = integral_image(&i_xy);
        let integral_yy = integral_image(&i_yy);
        

        let mut r = Array2::<f32>::zeros((image.height(), image.width()));


        for y in 0..image.height() {
            for x in 0..image.width() {
                
                let sum_xx = sum_rect(&integral_xx, x, y, window_size);
                let sum_yy = sum_rect(&integral_yy, x, y, window_size);
                let sum_xy = sum_rect(&integral_xy, x, y, window_size);

                let det = sum_xx * sum_yy - sum_xy * sum_xy;
                let trace = sum_xx + sum_yy;

                r[[y, x]] = det - k * (trace * trace);

             }
        }

        non_maximum_suppression(&r, image.width(), image.height(), threshold)
    }    
}

fn integral_image(img: &Array2<f32>) -> Array2<f32> {
    let (height, width) = img.dim();
    let mut integral = Array2::<f32>::zeros((height, width));

    for y in 0..height {
        for x in 0..width {
            let sum_above = if y > 0 { integral[[y - 1, x]] } else { 0.0 };
            let sum_left = if x > 0 { integral[[y, x - 1]] } else { 0.0 };
            let sum_above_left = if y > 0 && x > 0 { integral[[y - 1, x - 1]] } else { 0.0 };

            integral[[y, x]] = img[[y, x]] + sum_above + sum_left - sum_above_left;
        }
    }
    integral
}

fn sum_rect(integral: &Array2<f32>, x: usize, y: usize, window_size: usize) -> f32 {
    let half_size = window_size / 2;
    let x1 = if x >= half_size { x - half_size } else { 0 };
    let y1 = if y >= half_size { y - half_size } else { 0 };
    let x2 = if x + half_size < integral.dim().1 { x + half_size } else { integral.dim().1 - 1 };
    let y2 = if y + half_size < integral.dim().0 { y + half_size } else { integral.dim().0 - 1 };

    let sum = integral[[y2, x2]]
        - if x1 > 0 { integral[[y2, x1 - 1]] } else { 0.0 }
        - if y1 > 0 { integral[[y1 - 1, x2]] } else { 0.0 }
        + if x1 > 0 && y1 > 0 { integral[[y1 - 1, x1 - 1]] } else { 0.0 };

    sum
}

fn non_maximum_suppression(
    r: &Array2<f32>,
    width: usize,
    height: usize,
    threshold: f32,
) -> Vec<(usize, usize)> {
    let mut corners = Vec::new();

    for y in 1..height-1 {
        for x in 1..width-1 {
            if r[[y, x]] > threshold &&
               r[[y, x]] > r[[y-1, x]] &&
               r[[y, x]] > r[[y+1, x]] &&
               r[[y, x]] > r[[y, x-1]] &&
               r[[y, x]] > r[[y, x+1]] {
                corners.push((x, y));
            }
        }
    }
    println!("corner is : {:?}", corners);
    println!("num maximum supression succeeed");
    corners
}


