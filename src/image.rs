use derive_more::{Deref, DerefMut};
use image::{DynamicImage,ImageBuffer,Luma, Pixel};
use imageproc::drawing::Canvas;
use log::*;
use ndarray::{Array, Array2, ArrayView2};
use nshare::RefNdarray2;
use std::f32;

pub trait Kernel {
    fn get(&self, x: usize, y: usize) -> f32;
}

impl Kernel for Vec<f32> {
    fn get(&self, x: usize, y: usize) -> f32 {
        let size = (self.len() as f64).sqrt() as usize;
        self[y * size + x]
    }
}

impl Kernel for [[i32; 3]; 3] {
    fn get(&self, x: usize, y: usize) -> f32 {
        self[y][x] as f32
    }
}

// rgb , opacity ( 255, 255 )
type GrayImageBuffer = ImageBuffer<Luma<f32>, Vec<f32>>;

#[derive(Debug, Clone, Deref, DerefMut)]
pub struct GrayFloatImage(pub GrayImageBuffer);

impl GrayFloatImage {

    pub fn from_dynamic(input_image: &DynamicImage) -> Self {
        Self(match input_image.grayscale() {
            DynamicImage::ImageLuma8(gray_image) => {
                info!(
                    "Loaded a {} x {} 8-bit image",
                    input_image.width(),
                    input_image.height()
                );
                ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
                    Luma([f32::from(gray_image[(x, y)][0]) / 255f32])
                })
            }
            DynamicImage::ImageLuma16(gray_image) => {
                info!(
                    "Loaded a {} x {} 16-bit image",
                    input_image.width(),
                    input_image.height()
                );
                ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
                    Luma([f32::from(gray_image[(x, y)][0]) / 65535f32])
                })
            }
            DynamicImage::ImageLumaA8(gray_image) => {
                info!(
                    "Loaded a {} x {} 8-bit alpha image",
                    input_image.width(),
                    input_image.height()
                );
                ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
                    Luma([f32::from(gray_image[(x, y)][0]) / 255f32])
                })
            }
            DynamicImage::ImageLumaA16(gray_image) => {
                info!(
                    "Loaded a {} x {} 16-bit alpha image",
                    input_image.width(),
                    input_image.height()
                );
                ImageBuffer::from_fn(gray_image.width(), gray_image.height(), |x, y| {
                    Luma([f32::from(gray_image[(x, y)][0]) / 65535f32])
                })
            }
            DynamicImage::ImageRgb32F(float_image) => {
                info!(
                    "Loaded a {} x {} 32-bit RGB float image",
                    input_image.width(),
                    input_image.height()
                );
                ImageBuffer::from_fn(float_image.width(), float_image.height(), |x, y| {
                    Luma([float_image[(x, y)].to_luma()[0]])
                })
            }
            DynamicImage::ImageRgba32F(float_image) => {
                info!(
                    "Loaded a {} x {} 32-bit RGBA float image",
                    input_image.width(),
                    input_image.height()
                );
                ImageBuffer::from_fn(float_image.width(), float_image.height(), |x, y| {
                    Luma([float_image[(x, y)].to_luma()[0]])
                })
            }
            _ => panic!("DynamicImage::grayscale() returned unexpected type"),
        })
    }

    pub fn new(width: u32, height: u32) -> Self {
        let buffer = ImageBuffer::from_pixel(width, height, Luma([0.0]));
        GrayFloatImage(buffer)
    }

    pub fn ref_array(&self) -> ArrayView2<f32> {
        self.0.ref_ndarray2()
    }

    pub fn width(&self) -> usize {
        self.0.width() as usize
    }

    pub fn height(&self) -> usize {
        self.0.height() as usize
    }

    pub fn load_image(path: &str) -> Self {
        GrayFloatImage::from_dynamic(&image::open(path).unwrap())
    }   
    
    pub fn from_array2(array: Array2<f32>) -> Self {
        Self(
            ImageBuffer::from_fn(array.dim().1 as u32, array.dim().0 as u32, |x, y| {
                Luma([array[[y as usize, x as usize]]])
            })
        )
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        self.get_pixel(x as u32, y as u32)[0]
    }

    pub fn put(&mut self, x: usize, y: usize, pixel_value: f32) {
        self.put_pixel(x as u32, y as u32, Luma([pixel_value]));
    }

    pub fn to_u8_image(&self) -> ImageBuffer<Luma<u8>, Vec<u8>> {
        let (width, height) = (self.width() as u32, self.height() as u32);
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let value = (self[(x, y)][0] * 255.0).clamp(0.0, 255.0) as u8;
            *pixel = Luma([value]);
        }
        img
    }

    pub fn to_array2(&self) -> Array2<f32> {
        let (width, height) = (self.width() as usize, self.height() as usize);
        let mut array = Array2::<f32>::zeros((height, width));
        for y in 0..height {
            for x in 0..width {
                array[[y, x]] = self.0.get_pixel(x as u32, y as u32).0[0];
            }
        }
        array
    }
}


pub fn sobel_filter_x(image: &GrayFloatImage) -> Array2<f32>{
    let kernel: [[i32; 3]; 3] = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    // let kernel_vec: Vec<f32> = kernel.iter().flat_map(|&row| row.iter().map(|&val| val as f32).collect::<Vec<_>>()).collect();
    // let kernel_array: Array2<f32> = Array2::from_shape_vec((3, 3), kernel_vec).expect("Error converting to Array2");
    convolve(image, &kernel, kernel.len())
}

pub fn sobel_filter_y(image: &GrayFloatImage) -> Array2<f32>{
    let kernel:[[i32; 3]; 3]  = [[-1, -2, -1], [0, 0, 0], [1, 2, 1]];
    // let kernel_vec: Vec<f32> = kernel.iter().flat_map(|&row| row.iter().map(|&val| val as f32).collect::<Vec<_>>()).collect();
    // let kernel_array: Array2<f32> = Array2::from_shape_vec((3, 3), kernel_vec).expect("Error converting to Array2");
    convolve(image, &kernel, kernel.len())
}

fn gaussian(x: f32, r: f32) -> f32 {
    ((2.0 * f32::consts::PI).sqrt() * r).recip() * (-x.powi(2) / (2.0 * r.powi(2))).exp()
}

pub fn gaussian_kernel(r: f32, kernel_size: usize) -> Vec<f32> {
    assert!(kernel_size % 2 == 1, "kernel_size must be odd");
    let mut kernel = vec![0f32; kernel_size];
    let half_width = (kernel_size / 2) as i32;
    let mut sum = 0f32;
    for i in -half_width..=half_width {
        let val = gaussian(i as f32, r);
        kernel[(i + half_width) as usize] = val;
        sum += val;
    }
    for val in kernel.iter_mut() {
        *val /= sum;
    }
    kernel
}

pub fn gaussian_blur(image: &GrayFloatImage, r: f32) -> GrayFloatImage  {
    let kernel_radius = (2.0 * r).ceil() as usize;
    let kernel_size = kernel_radius * 2 + 1;
    let kernel = gaussian_kernel(r, kernel_size);
    
    let blurred_array = convolve(image, &kernel, kernel_size);
    
    let mut blurred_image = GrayFloatImage::new(image.width() as u32, image.height() as u32);
    for y in 0..image.height() {
        for x in 0..image.width() {
            let pixel_value = blurred_array[[y as usize, x as usize]];
            blurred_image.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    blurred_image
}


fn convolve<T: Kernel>(image: &GrayFloatImage, kernel: &T, kernel_size: usize) -> Array2<f32> {
    let (width, height) = (image.width() as usize, image.height() as usize);
    let mut result = Array2::<f32>::zeros((height, width));
    let half_k = kernel_size / 2;

    for y in half_k..height - half_k {
        for x in half_k..width - half_k {
            let mut sum = 0.0;
            for ky in 0..3 {
                for kx in 0..3 {
                    let px = x + kx - half_k;
                    let py = y + ky - half_k;
                    sum += image.get(px, py) * kernel.get(kx, ky)
                }
            }
            result[[y, x]] = sum;
        }
    }
    result
}
