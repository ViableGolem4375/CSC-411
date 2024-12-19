use array2::Array2;
use clap::Parser;
#[allow(unused_imports)]
use csc411_image::{Read, Write, RgbImage, Rgb};
//use std::time::Instant;
#[allow(unused_imports)]
use std::env;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // Row-Major
    #[clap(long="row-major")]
    row_major: bool,
    // Col-Major
    #[clap(long="col-major")]
    col_major: bool,
    // Flip
    #[clap(short='f', long="flip")]
    flip: Option<String>,
    // Rotate
    #[clap(short='r', long="rotate")]
    rotate: Option<u32>,
    // Transpose
    #[clap(long="transpose")]
    transpose: bool,
    // File Name
    input_file: Option<String>,
}

fn main() {
    let args = Args::parse();
    let input = RgbImage::read(args.input_file.as_deref()).unwrap();
    let height = input.height as usize;
    let width = input.width as usize;
    let image = Array2::from_row_major( width, height, &input.pixels).expect("REASON");
    // Flip Check
    if args.flip.as_deref() != None { flip(image.clone(), args.flip.clone(), args.row_major, input.denominator); }

    // Rotate Check
    if args.rotate != None { rotation(image.clone(), args.rotate.clone(), args.row_major, input.denominator); }

    // Transpose Check
    if args.transpose { transpose(image.clone(), args.row_major, input.denominator); }
}

fn flip (mut image: Array2<Rgb>, flip: Option<String>, is_row_major: bool, denom: u16) {
    // Horizontal Flip
    if flip.as_deref() == Some("horizontal") {
        //let now = Instant::now();
        image.flip_horizontal(is_row_major);

        //let end = now.elapsed();
        //println!("Running the function took {:.3?} seconds.", end.as_secs());
        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };
        
        let _outputfile = output.write(Some("test.ppm"));
        //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // Vertical Flip
    if flip.as_deref() == Some("vertical") {
        //let now = Instant::now();
        image.flip_vertical(is_row_major);

        //let end = now.elapsed();
        //println!("Running the function took {:.3?} seconds.", end.as_secs());

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };

        let _outputfile = output.write(Some("test.ppm"));
        //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }
}

fn rotation (mut image: Array2<Rgb>, rotate: Option<u32>, is_row_major: bool, denom: u16) {
    // 0 Degree Rotation
    if rotate == Some(0) {
        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };

        let _outputfile = output.write(Some("test.ppm"));
        //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // 90 Degree Rotation
    if rotate == Some(90) {
        //let now = Instant::now();
        image.rotate_90(is_row_major);

        //let end = now.elapsed();
        //println!("Running the function took {:.3?} seconds.", end);

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };
        
        let _outputfile = output.write(Some("test.ppm"));
        //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // 180 Degree Rotation
    if rotate == Some(180) {
        //let now = Instant::now();
        image.rotate_180(is_row_major);

        //let end = now.elapsed();
        //println!("Running the function took {:.3?} seconds.", end);

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };

        let _outputfile = output.write(Some("test.ppm"));
        //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }

    // 270 Degree Rotation
    if rotate == Some(270) {
        //let now = Instant::now();
        image.rotate_270(is_row_major);

        //let end = now.elapsed();
        //println!("Running the function took {:.3?} seconds.", end);

        let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };

        let _outputfile = output.write(Some("test.ppm"));
        //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
    }
}

fn transpose (mut image: Array2<Rgb>, is_row_major: bool, denom: u16) {
    //let now = Instant::now();
    image.transpose(is_row_major);

    //let end = now.elapsed();
    //println!("Running the function took {:.3?} seconds.", end);

    let output = RgbImage { pixels: image.elements_row_major().clone(), width: image.num_columns() as u32, height: image.num_rows() as u32, denominator: denom };

    let _outputfile = output.write(Some("test.ppm"));
    //if output.write(None) != Ok(()) { eprintln!("Unable to write file!"); }
}
