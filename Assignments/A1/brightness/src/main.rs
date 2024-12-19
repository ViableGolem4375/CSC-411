use csc411_image::{Read, GrayImage};
use std::env;

fn main() {
    let input = env::args().nth(1);
    assert!(env::args().len() == 2);
    let img = GrayImage::read(input.as_deref()).unwrap();
    let h: u32 = img.height;
    let w = img.width;
    let mut num: f32;
    let mut value: f32;
    let denom = img.denominator as f32;
    let dimensions: u32 = h * w;
    let mut sum: f32 = 0.0;
    for pixel in img.pixels {
        value = pixel.value as f32;
        num = value / denom;
        sum += num as f32;
    }
    let brightness = sum as f32 / dimensions as f32;
    println!("{:.3}", brightness);
}
