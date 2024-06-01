use crate::image::GrayFloatImage;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Corner {
    pub x: u32,
    pub y: u32,
    pub score: f32    
}

impl Corner {
    pub fn new(x: u32, y: u32, score: f32) -> Corner {
        Corner { x, y, score }
    }
}

pub enum Fast {
    /// Corners require a section of length as least nine.
    Nine,
}


unsafe fn get_circle(
    image: &GrayFloatImage,
    x: u32,
    y: u32,
    p0: i16,
    p4: i16,
    p8: i16,
    p12: i16,
) -> [i16; 16] {
    [
        p0,
        image.unsafe_get_pixel(x + 1, y - 3)[0] as i16,
        image.unsafe_get_pixel(x + 2, y - 2)[0] as i16,
        image.unsafe_get_pixel(x + 3, y - 1)[0] as i16,
        p4,
        image.unsafe_get_pixel(x + 3, y + 1)[0] as i16,
        image.unsafe_get_pixel(x + 2, y + 2)[0] as i16,
        image.unsafe_get_pixel(x + 1, y + 3)[0] as i16,
        p8,
        image.unsafe_get_pixel(x - 1, y + 3)[0] as i16,
        image.unsafe_get_pixel(x - 2, y + 2)[0] as i16,
        image.unsafe_get_pixel(x - 3, y + 1)[0] as i16,
        p12,
        image.unsafe_get_pixel(x - 3, y - 1)[0] as i16,
        image.unsafe_get_pixel(x - 2, y - 2)[0] as i16,
        image.unsafe_get_pixel(x - 1, y - 3)[0] as i16,
    ]
}

fn is_corner_fast9(image: &GrayFloatImage, threshold: u8, x: u32, y: u32) -> bool {
    let (width, height) = image.dimensions();
    if x >= u32::MAX -3 
    || y >= u32::MAX -3 
    || x < 3
    || y < 3
    || width <= x + 3
    || height <= y + 3
    {
        return false
    }

    let c = unsafe { image.unsafe_get_pixel(x, y)[0] };
    let low_thresh: i16 = c as i16 - threshold as i16;
    let high_thresh: i16 = c as i16 + threshold as i16;

    let (p0, p4, p8, p12) = unsafe {
        (
            image.unsafe_get_pixel(x, y - 3)[0] as i16,
            image.unsafe_get_pixel(x, y + 3)[0] as i16,
            image.unsafe_get_pixel(x + 3, y)[0] as i16,
            image.unsafe_get_pixel(x - 3, y)[0] as i16,
        )
    };

    

    let above = (p0 > high_thresh && p4 > high_thresh)
        || (p4 > high_thresh && p8 > high_thresh)
        || (p8 > high_thresh && p12 > high_thresh)
        || (p12 > high_thresh && p0 > high_thresh);

    let below = (p0 < low_thresh && p4 < low_thresh)
        || (p4 < low_thresh && p8 < low_thresh)
        || (p8 < low_thresh && p12 < low_thresh)
        || (p12 < low_thresh && p0 < low_thresh);

    if !above && !below {
        return false;
    }
    
        // JUSTIFICATION - see comment at the start of this function
    let pixels = unsafe { get_circle(image, x, y, p0, p4, p8, p12) };

    (above && has_bright_span(&pixels, 9, high_thresh))
        || (below && has_dark_span(&pixels, 9, low_thresh))

}


fn has_bright_span(circle: &[i16; 16], length: u8, threshold: i16) -> bool {
    search_span(circle, length, |c| *c > threshold)
}

/// True if the circle has a contiguous section of at least the given length, all
/// of whose pixels have intensities strictly less than the threshold.
fn has_dark_span(circle: &[i16; 16], length: u8, threshold: i16) -> bool {
    search_span(circle, length, |c| *c < threshold)
}

fn search_span<F>(circle: &[i16; 16], length: u8, f: F) -> bool
where
    F: Fn(&i16) -> bool,
{
    if length > 16 {
        return false;
    }

    let mut nb_ok = 0u8;
    let mut nb_ok_start = None;

    for c in circle.iter() {
        if f(c) {
            nb_ok += 1;
            if nb_ok == length {
                return true;
            }
        } else {
            if nb_ok_start.is_none() {
                nb_ok_start = Some(nb_ok);
            }
            nb_ok = 0;
        }
    }

    nb_ok + nb_ok_start.unwrap() >= length
}

pub fn float_corners_fast9(image: &GrayFloatImage, threshold: u8) -> Vec<Corner> {
    let (width, height) = (image.width(), image.height());
    let mut corners = vec![];

    for y in 0..height {
        for x in 0..width {
            if is_corner_fast9(image, threshold, x as u32, y as u32) {
                let score = fast_corner_score(image, threshold, x as u32, y as u32, Fast::Nine);
                corners.push(Corner::new(x as u32, y as u32, score as f32));
            }
        }
    }

    corners

}

pub fn fast_corner_score(image: &GrayFloatImage, threshold: u8, x: u32, y: u32, variant: Fast) -> u8 {
    let mut max = 255u8;
    let mut min = threshold;

    loop {
        if max == min {
            return max;
        }

        let mean = ((max as u16 + min as u16) / 2u16) as u8;
        let probe = if max == min + 1 { max } else { mean };

        let is_corner = match variant {
            Fast::Nine => is_corner_fast9(image, probe, x, y),
        };

        println!("is_corner : {}", is_corner);

        if is_corner {
            min = probe;
        } else {
            max = probe - 1;
        }
    }
}
