
use array2::Array2;
use csc411_image::{Read, RgbImage};
use clap::Command;
use std::cmp;
use itertools::Itertools;

fn main() {
    let (img1, img2) = get_images();
    let root_mean_squared = calculate_rms(img1, img2);
    println!("{:.4}", root_mean_squared);
}

fn calculate_rms(img1: csc411_image::RgbImage, img2: csc411_image::RgbImage) -> f64 {	
	let min_width = cmp::min(img1.width, img2.width) as usize;
    let min_height = cmp::min(img1.height, img2.height) as usize;
    let denominator = img1.denominator as f64;
    
    let pixel_diff: f64 = 
    	// first image, with extra odd row/column chopped off
	     Itertools::flatten(Array2::from_row_major(img1.width as usize, img1.height as usize, img1.pixels).unwrap().rows_iter()
	    	.chunks(img1.width as usize)
	    	.into_iter()
	    	.map(|chunk| chunk.take(min_width))
	    	.take(min_height)).zip(
	    	// second image, extra row/column chopped off
	    	// zip them together
		    	Itertools::flatten(Array2::from_row_major(img2.width as usize, img2.height as usize, img2.pixels).unwrap().rows_iter()
		    	.chunks(img2.width as usize)
		    	.into_iter()
		    	.map(|chunk| chunk.take(min_width))
		    	.take(min_height)))
			    	.map(|(pixel1, pixel2)| {
			    		// compare values pixels in same position in each image
			    		(pixel2.2.red as f64 - pixel1.2.red as f64).powi(2) +
			    		(pixel2.2.green as f64 - pixel1.2.green as f64).powi(2) + 
			    		(pixel2.2.blue as f64 - pixel1.2.blue as f64).powi(2)
			    	})
			    	.sum();
	    
    (pixel_diff / (3 * min_width * min_height) as f64).sqrt() / denominator
}

fn get_images() -> (csc411_image::RgbImage, csc411_image::RgbImage) {	
	// match upon the possible arguments using clap
	let matches = Command::new("Image Comparison")
		.args([
			clap::arg!([IMAGE_1]),
			clap::arg!([IMAGE_2])
			])
		.get_matches();
	
	let mut input1 = matches.value_of("IMAGE_1");
	let mut input2 = matches.value_of("IMAGE_2");
	
	if input1 == Some("-") {
		input1 = None;
	}
	if input2 == Some("-") {
		input2 = None;
	}
	
	let img1 = RgbImage::read(input1).unwrap();
	let img2 = RgbImage::read(input2).unwrap();
	
	if (img1.width as i64 - img2.width as i64).abs() > 1 ||(img1.height as i64 - img2.height as i64).abs() > 1 {
		eprintln!("Images should be within one pixel of each other for both width and height.");
		std::process::exit(1);
	}
	
	(img1, img2)
}
