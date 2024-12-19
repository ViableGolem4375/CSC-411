
use csc411_image;
use csc411_rpegio;
use csc411_image::Write;
use csc411_image::{Read, RgbImage};
use bitpack::bitpack::{newu, news};
use csc411_rpegio::{output_rpeg_data, read_in_rpeg_data};
use crate::format::{trim_img, divide_denom, unpack_data};
use crate::rgb::{rgb_to_ypbpr, chroma_index};
use crate::dct::{dct_calculator, dct_to_rgb};

/// Structs made to allow easier use of these values within a Vec.
#[derive(Clone, Debug)]
pub struct YpbprValues {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

#[derive(Clone, Debug)]
pub struct RgbFloatValues {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct DCTValues{
    pub yval: f32,
    pub avg_pb: f32,
    pub avg_pr: f32,
}

/// Function to perform image compression.
///
/// Arguments:
/// * `file`: the image being compressed.
pub fn compress(file: &str){
    let img = RgbImage::read(Some(file)).unwrap();

    let mut img_width = img.width;
    let mut img_height = img.height;

    if img.width % 2 != 0{
        img_width -= 1;
    }
    if img.height % 2 != 0{
        img_height -=1;
    }
    
    let new_img = trim_img(&img, img_width, img_height);
    
    let new_img_deci = divide_denom(&new_img, &img, img_width, img_height);
    
    let pb_vector = rgb_to_ypbpr(&new_img, &new_img_deci, img_width, img_height);

    let mut words = Vec::new();
    for row in (0..img_height).step_by(2){
        for col in (0..img_width).step_by(2){
            let (a,b,c,d,avg_pb,avg_pr) = chroma_index(&pb_vector, img_width, img_height, row, col);
            let mut word = 0_u64;
            word = newu(word, 9, 23, a as u64).unwrap();
            word = news(word, 5, 18, b as i64).unwrap();
            word = news(word, 5, 13, c as i64).unwrap();
            word = news(word, 5, 8, d as i64).unwrap();
            word = newu(word, 4, 4, avg_pb as u64).unwrap();
            word = newu(word, 4, 0, avg_pr as u64).unwrap();
            words.push((word as u32).to_be_bytes());
        }
    }
    output_rpeg_data(&words, img_width, img_height);
}

/// Function to perform image decompression.
///
/// Arguments:
/// * `file`: the image being decompressed.
pub fn decompress(file: &str) {
    let (_raw_bytes, _width, _height) = read_in_rpeg_data(Some(file)).unwrap();
    
    let unpack_word_list = unpack_data(_raw_bytes);

    let mut dct_val_list: Vec<DCTValues> = vec![DCTValues{yval: 0.0, avg_pb: 0.0, avg_pr: 0.0}; _height as usize* _width as usize];

    dct_val_list = dct_calculator(dct_val_list, _height, _width, unpack_word_list);
    
    let rgb_final = dct_to_rgb(dct_val_list);

    let final_img = RgbImage{
        width: _width as u32,
        height: _height as u32,
        denominator: 255,
        pixels: rgb_final,
    };
                
    final_img.write(None).unwrap();
}
