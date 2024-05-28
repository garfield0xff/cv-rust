
pub mod image;
pub mod descriptors;
pub mod detectors;
pub mod harris;

use std::time::Instant;

use harris::Harris;
use ::image::{open, GrayImage, Luma, Rgb, RgbImage};
use imageproc::{corners::{corners_fast12, corners_fast9}, drawing::{draw_filled_circle_mut, Canvas}, point::Point};
use log::info;
use pretty_env_logger::init;
use rand::Rng;
use crate::{image::GrayFloatImage};

const PATCH_SIZE: usize = 31;
const BRIEF_SIZE: usize = 256;


fn brief_descriptor(image: &GrayImage, x: u32, y: u32) -> Vec<u8> {
    let mut descriptor = vec![0u8; BRIEF_SIZE];
    let mut rng = rand::thread_rng();

    for i in 0..BRIEF_SIZE {
        let p_x = rng.gen_range(-(PATCH_SIZE as i32 / 2)..(PATCH_SIZE as i32 / 2));
        let p_y = rng.gen_range(-(PATCH_SIZE as i32 / 2)..(PATCH_SIZE as i32 / 2));
        let q_x = rng.gen_range(-(PATCH_SIZE as i32 / 2)..(PATCH_SIZE as i32 / 2));
        let q_y = rng.gen_range(-(PATCH_SIZE as i32 / 2)..(PATCH_SIZE as i32 / 2));

        let p = (x as i32 + p_x, y as i32 + p_y);
        let q = (x as i32 + q_x, y as i32 + q_y);

        if is_within_bounds(image, p) && is_within_bounds(image, q) {
            let p_value = image.get_pixel(p.0 as u32, p.1 as u32)[0];
            let q_value = image.get_pixel(q.0 as u32, q.1 as u32)[0];
            if p_value < q_value {
                descriptor[i] = 1;
            }
        }
    }

    descriptor
}

fn is_within_bounds(image: &GrayImage, point: (i32, i32)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < image.width() as i32 && point.1 < image.height() as i32
}


fn main() {
    
        pretty_env_logger::init();
    
        let img_path = "/Users/gyujinkim/Desktop/Github/cv-rust/src/harris_input_test2.png";
        let mut gray_image = open(&img_path).expect("failed to load image").to_luma8();
        let output_path_fast9 = "output_fast9.png";
        let output_path_fast12 = "output_fast12.png";


        // FAST9 코너 검출
        let start = Instant::now();
        let corners_fast9 = corners_fast9(&gray_image, 30);
        info!("Fast9 detected {} corners", corners_fast9.len());
        info!("Fast9 detector response in : {:?}", start.elapsed());
    
        // FAST12 코너 검출
        let start = Instant::now();
        let corners_fast12 = corners_fast12(&gray_image, 30);
        println!("Fast12 detected {} corners", corners_fast12.len());
        info!("Fast12 detector response in : {:?}", start.elapsed());

        let brief_descriptors_fast9: Vec<_> = corners_fast9.iter()
        .map(|&corner| brief_descriptor(&gray_image, corner.x as u32, corner.y as u32))
        .collect();
    
        info!("BRIEF descriptors for FAST9: {:?}", brief_descriptors_fast9);

        let brief_descriptors_fast12: Vec<_> = corners_fast12.iter()
        .map(|&corner| brief_descriptor(&gray_image, corner.x as u32, corner.y as u32))
        .collect();

        info!("BRIEF descriptors for FAST12: {:?}", brief_descriptors_fast12);

    
        // 그레이스케일 이미지를 RGB 이미지로 변환
        let mut rgb_img = RgbImage::new(gray_image.width(), gray_image.height());
        for (x, y, pixel) in gray_image.enumerate_pixels() {
            let rgb_pixel = Rgb([pixel[0], pixel[0], pixel[0]]);
            rgb_img.put_pixel(x, y, rgb_pixel);
        }
        
        // FAST9 코너를 빨간색으로 그리기
        for corner in corners_fast9 {
            draw_filled_circle_mut(&mut rgb_img, (corner.x as i32, corner.y as i32), 2, Rgb([255, 0, 0]));
        }
    
        // 결과 이미지 저장
        rgb_img.save(output_path_fast9).expect("Failed to save image");
    
        // 새로운 RGB 이미지 생성
        let mut rgb_img = RgbImage::new(gray_image.width(), gray_image.height());
        for (x, y, pixel) in gray_image.enumerate_pixels() {
            let rgb_pixel = Rgb([pixel[0], pixel[0], pixel[0]]);
            rgb_img.put_pixel(x, y, rgb_pixel);
        }
    
        // FAST12 코너를 빨간색으로 그리기
        for corner in corners_fast12 {
            draw_filled_circle_mut(&mut rgb_img, (corner.x as i32, corner.y as i32), 2, Rgb([255, 0, 0]));
        }
    
        // 결과 이미지 저장
        rgb_img.save(output_path_fast12).expect("Failed to save image");
        
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



