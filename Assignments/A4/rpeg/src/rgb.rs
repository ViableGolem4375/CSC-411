use csc411_arith::index_of_chroma;
use crate::format::RgbFloatValues;

/// A struct made to allow easier use of these values within a Vec.
#[derive(Clone, Debug)]
pub struct YpbprValues {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

///  Converts rgb into ypbpr. Returns a vector including all of the ypbpr values.
/// 
/// Arguments:
/// * `image`: &Vec<csc411_image::Rgb> holds the value of all the rgb pixels from the given file.
/// * `image_decimal`: Vec<RgbFloatValues> holds the decimal versions of the rgb pixels from image.
/// * `width`: width.
/// * `height`: height.
pub fn rgb_to_ypbpr(image: &Vec<csc411_image::Rgb>, image_decimal: &Vec<RgbFloatValues>, width: u32, height: u32) -> Vec<YpbprValues>{
    let mut pb_vec: Vec<YpbprValues> = vec![YpbprValues{y: 0.0, pb:0.0, pr: 0.0}; width as usize * height as usize].clone();
    
    for pixel in 0..image.len(){
        let y = 0.299 * image_decimal[pixel].red + 0.587 * image_decimal[pixel].green + 0.114 * image_decimal[pixel].blue;
        let pb = -0.168736 * image_decimal[pixel].red + (-0.331264) * image_decimal[pixel].green + 0.5 * image_decimal[pixel].blue;
        let pr = 0.5 * image_decimal[pixel].red + (-0.418688) * image_decimal[pixel].green + (-0.081312) * image_decimal[pixel].blue;
        pb_vec[pixel].y = y;
        pb_vec[pixel].pb = pb;
        pb_vec[pixel].pr = pr;
    }

    return pb_vec;
}

/// Takes the index of chroma for the pb and pr values.
/// 
/// Arguments:
/// * `pb`: A vector of ypbpr values.
/// * `width`: width.
/// * `_height`: height.
/// * `row`: row.
/// * `col`: column.
pub fn chroma_index(pb: &Vec<YpbprValues>, width: u32, _height: u32, row: u32, col: u32) -> (f32, f32, f32, f32, usize, usize) {
    let x = 511.0;
    let y = 50.0;
    let z = 4.0;
    let top_l = pb[(width * row + col) as usize].clone();
    let top_r = pb[(width * row + (col + 1)) as usize].clone();
    let bot_l = pb[(width * (row + 1) + col) as usize].clone();
    let bot_r = pb[(width * (row + 1) + (col + 1)) as usize].clone();
    let avg_pb = (top_l.pb + top_r.pb + bot_r.pb + bot_l.pb) / z;
    let avg_pr = (top_l.pr + top_r.pr + bot_r.pr + bot_l.pr) / z;
    let avg_pb = index_of_chroma(avg_pb as f32);
    let avg_pr = index_of_chroma(avg_pr as f32);
    let mut a = (bot_r.y + bot_l.y + top_r.y + top_l.y) / z;
    let mut b = (bot_r.y + bot_l.y - top_r.y - top_l.y) / z;
    let mut c = (bot_r.y - bot_l.y + top_r.y - top_l.y) / z;
    let mut d = (bot_r.y - bot_l.y - top_r.y + top_l.y) / z;

    a = (a* x).round();
    b = (b.clamp(-0.3,0.3) * y).round();
    c = (c.clamp(-0.3,0.3) * y).round();
    d = (d.clamp(-0.3,0.3) * y).round();

    return (a,b,c,d,avg_pb, avg_pr);
}
