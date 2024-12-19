
use array2::Array2;
use csc411_image::{Read, GrayImage};
use std::{env, process::exit};

//Function to check the sudoku solution.
fn check_sudoku(grid: &Array2<i32>) -> bool {
    // Check if the grid has 9 rows and 9 columns.
    if grid.num_rows() != 9 || grid.num_columns() != 9 {
        return false;
    }

    // Check if each row contains only digits from 1 to 9.
    for row in grid.rows_iter() {
        let mut seen = [false; 9];
        for &cell in row {
            if cell < 1 || cell > 9 {
                return false;
            }
            if seen[cell as usize - 1] {
                return false;
            }
            seen[cell as usize - 1] = true;
        }
    }

    // Check if each column contains only digits from 1 to 9.
    for col in grid.columns_iter() {
        let mut seen = [false; 9];
        for &cell in col {
            if cell < 1 || cell > 9 {
                return false;
            }
            if seen[cell as usize - 1] {
                return false;
            }
            seen[cell as usize - 1] = true;
        }
    }

    // Check if each 3x3 subgrid contains only digits from 1 to 9.
    for i in 0..3 {
        for j in 0..3 {
            let mut seen = [false; 9];
            for x in 0..3 {
                for y in 0..3 {
                    let cell = grid[(i * 3 + x, j * 3 + y)];
                    if cell < 1 || cell > 9 {
                        return false;
                    }
                    if seen[cell as usize - 1] {
                        return false;
                    }
                    seen[cell as usize - 1] = true;
                }
            }
        }
    }

    // If all checks pass, return true.
    true
}

//Main function.
fn main() {
    #[allow(dead_code)]
    let mut pixelvalues: Vec<i32> = Vec::new();
    let mut counter = 1;
    let mut value: i32;
    let input = env::args().nth(1);
    assert!(env::args().len() == 2, "Cannot input 2 files at once.");
    let img = GrayImage::read(input.as_deref()).unwrap();
    //Read pixel brightness values from .pgm image into a Vec.
    for pixel in img.pixels {
        value = pixel.value as i32;
        pixelvalues.resize(counter, value);
        counter += 1;
    }
    //Check length of the vec, if it's not 81 (number of values in a
    //sudoku solution), exit.
    if pixelvalues.len() != 81 {
        exit(1);
    }
    //Otherwise generate an exit code depending on the output from check_sudoku.
    else {
        let sudoku: Array2<i32> = Array2::from_row_major(&pixelvalues, 9, 9).expect("REASON");
        if check_sudoku(&sudoku) { 
            exit(0);
        } 
        else { 
            exit(1);
        };
}
}
