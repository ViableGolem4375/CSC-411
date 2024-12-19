use csc411_image::{RgbImage, Rgb};
use bitpack::bitpack::{gets, getu};

/// Structs made to allow easier use of these values within a Vec.
#[derive(Clone, Debug)]
pub struct RgbFloatValues {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

#[derive(Clone, Debug)]
pub struct PackedValues{
    pub a: u64,
    pub b: i64,
    pub c: i64,
    pub d: i64,
    pub avg_pb: u64,
    pub avg_pr: u64,
}

/// Function to store the image into a vector which will be used in the compression/decompression functions.
///
/// Arguments: 
/// * `new_img`: The vector of pixels being read into the function.
/// * `for_denom`: The image being used to get the brightness denominator to be used for converting into the Vec.
/// * `width`: width.
/// * `height`: height,
pub fn divide_denom(new_img: &Vec<csc411_image::Rgb>, for_denom: &RgbImage, width: u32, height: u32) -> Vec<RgbFloatValues>{
    let mut img_decimal: Vec<RgbFloatValues> = vec![RgbFloatValues{red: 0.0, green:0.0, blue: 0.0}; width as usize * height as usize].clone();

    for pixel in 0..new_img.len(){
        img_decimal[pixel].red = new_img[pixel].red as f32/(for_denom.denominator as f32);
        img_decimal[pixel].green = new_img[pixel].green as f32/for_denom.denominator as f32;
        img_decimal[pixel].blue = new_img[pixel].blue as f32/for_denom.denominator as f32;
    }
    return img_decimal;
}

/// Function to unpack data in the form of bytes.
/// 
/// Arguments: 
/// * `bytes``: A vec of byte values.
pub fn unpack_data(bytes: Vec<[u8; 4]>) -> Vec<PackedValues> {
    let mut data = Vec::new();
    for byte in bytes{
        let unpacked_data = u32::from_be_bytes(byte);
        let a = getu(unpacked_data as u64, 9, 23);
        let b = gets(unpacked_data as u64, 5, 18);
        let c = gets(unpacked_data as u64, 5, 13);
        let d = gets(unpacked_data as u64, 5, 8);
        let avg_pb = getu(unpacked_data as u64, 4, 4);
        let avg_pr = getu(unpacked_data as u64, 4, 0);

        let packed_data = PackedValues{a: a.unwrap(), b: b.unwrap(), c: c.unwrap(), d: d.unwrap(), avg_pb: avg_pb.unwrap(), avg_pr: avg_pr.unwrap(),
        };
        data.push(packed_data);
    }
    return data;
}

/// Function to trim the edges off of the image in case of odd column or row count.
///
/// Arguments:
/// * `img`: the image being transformed.
/// * `new_width`: the new width after removing a column.
/// * `new_height`: the new height after removing a column.
pub fn trim_img(img: &RgbImage, new_width: u32, new_height: u32) -> Vec<csc411_image::Rgb>{
    let mut new_img: Vec<Rgb> = vec![Rgb{red: 0, green: 0, blue: 0}; (new_height * new_width) as usize];

    for i in 0..new_height{
        for j in 0..new_width{
            new_img[(new_width as usize * i as usize) + j as usize] = img.pixels[(img.width as usize * i as usize) + j as usize].clone();
        }
    }
    return new_img;
}
