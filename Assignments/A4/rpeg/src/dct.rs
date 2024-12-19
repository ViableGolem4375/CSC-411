use csc411_arith::chroma_of_index;
use crate::codec::DCTValues;
use crate::format::PackedValues;
use csc411_image::Rgb;

/// A struct made to allow easier use of these values within a Vec.
#[derive(Clone, Debug)]
pub struct YpbprValues {
    pub y: f32,
    pub pb: f32,
    pub pr: f32,
}

/// Converts DCT values to rgb values.
/// 
/// # Arguments:
/// * `pos_list`: used to access the values to be converted to rgb.
pub fn dct_to_rgb(pos_list: Vec<DCTValues>) -> Vec<Rgb>{
    //dct to rgb float
    let mut rgb_final = Vec::new();
    for value in pos_list{
        let rgb_val = Rgb{
            red: ((1.0 * value.yval + 0.0 * value.avg_pb + 1.402 * value.avg_pr) * 255.0) as u16,
            green: ((1.0 * value.yval - 0.344136 * value.avg_pb - 0.714136 * value.avg_pr) * 255.0) as u16,
            blue: ((1.0 * value.yval + 1.772 * value.avg_pb + 0.0 * value.avg_pr) * 255.0) as u16,
        };
        rgb_final.push(rgb_val);
    }
    return rgb_final;
}

/// Calculates y values using DCT.
/// 
/// # Arguments:
/// * `pos_list`: list that holds the positions of the dct values.
/// * `height`: height.
/// * `width`: width.
/// * `y_list`: vector that contains the values needed to calculate y1,y2,y3,y4.
pub fn dct_calculator(mut pos_list: Vec<DCTValues>, height: u32, width: u32, y_list: Vec<PackedValues>) -> Vec<DCTValues>{
    let mut counter = 0;
    let x = 511.0;
    let y = 50.0;
    for i in (0..height).step_by(2){
        for j in (0..width).step_by(2){
            let a_new = (y_list[counter].a as f32 / x).clamp(0.0,1.0);
            let b_new = (y_list[counter].b as f32 / y).clamp(-0.3,0.3);
            let c_new = (y_list[counter].c as f32 / y).clamp(-0.3,0.3);
            let d_new = (y_list[counter].d as f32 / y).clamp(-0.3,0.3);
            let pb = chroma_of_index(y_list[counter].avg_pb as usize);
            let pr = chroma_of_index(y_list[counter].avg_pr as usize);
            let y1 = a_new - b_new - c_new + d_new;
            let y2 = a_new - b_new + c_new - d_new;
            let y3 = a_new + b_new - c_new - d_new;
            let y4 = a_new + b_new + c_new + d_new;
            pos_list[(i * width + j) as usize] = DCTValues{yval: y1, avg_pb: pb, avg_pr: pr,};
            pos_list[(i * width + (j+1)) as usize] = DCTValues{yval: y2, avg_pb: pb, avg_pr: pr,};
            pos_list[((i+1) * width + j) as usize] = DCTValues{yval: y3, avg_pb: pb, avg_pr: pr,};
            pos_list[((i+1) * width + (j+1)) as usize] = DCTValues{yval: y4, avg_pb: pb, avg_pr: pr,};
            counter += 1;
        }
    }
    return pos_list;
}
