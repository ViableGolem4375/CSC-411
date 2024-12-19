use std::io::stdin;
use std::io::stdout;
use std::io::Write;
use std::io::Read;
use crate::memory::UmState;

/// Performs a Conditional Move if $r[C] != 0
/// Modifies the a register in the VM object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn cmov(um: &mut UmState, a: usize, b: usize, c: usize){
    if um.registers[c] != 0{
        um.registers[a] = um.registers[b];
    }
}

/// Performs a Segmented Load
/// Modifies the a register in the UmState object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn sload(um: &mut UmState, a: usize, b: usize, c: usize){
    um.registers[a] = um.memory[um.registers[b] as usize][um.registers[c] as usize];
}

/// Performs a Segmented Store
/// Modifies the memory address at the $m[$r[a]][$r[b]] index
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn store(um: &mut UmState, a: usize, b: usize, c: usize){
    um.memory[um.registers[a] as usize][um.registers[b] as usize] = um.registers[c];
}

/// Performs an Addition operation
/// Modifies the a register in the UmState object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn add(um: &mut UmState, a: usize, b: usize, c: usize){
    um.registers[a] = um.registers[b].wrapping_add(um.registers[c]);
}

/// Performs a Multiplication operation
/// Modifies the a register in the UmState object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn mult(um: &mut UmState, a: usize, b: usize, c: usize){
    um.registers[a] = um.registers[b].wrapping_mul(um.registers[c]);
}

/// Performs integer division
/// Modifies the a register in the UmState object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn div(um: &mut UmState, a: usize, b: usize, c: usize){
    if um.registers[c] == 0{
        panic!("Cannot divide by 0")
    }
    um.registers[a] = um.registers[b] / um.registers[c];
}

/// Performs Bitwise NAND
/// Modifies the a register in the UmState object
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * a: The a register
/// * b: The b register
/// * c: The c register
pub fn nand(um: &mut UmState, a: usize, b: usize, c: usize){
    um.registers[a] = !(um.registers[b] & um.registers[c]);
}

/// Ends the program
pub fn halt(){
    std::process::exit(0);
}

/// Maps a segment
/// The new segment is mapped as $m[$r[b]]
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * b: The b register
/// * c: The c register
pub fn map_seg(um: &mut UmState, b: usize, c: usize){
    let length = um.registers[c] as usize;
    let new_segment = vec![0_u32; length];

    if um.unmap_index_values.len() != 0{
        um.registers[b] = (um.unmap_index_values.pop().unwrap()) as u32;
        um.memory[um.registers[b] as usize] = new_segment;
    }else {
        um.memory.push(new_segment); // Removed the .clone() call
        um.registers[b] = (um.memory.len() - 1) as u32;
    }
}

/// Unmaps a segment
/// The segment $m[$r[c]] is unmapped
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * c: The c register
pub fn unmap_seg(um: &mut UmState, c: usize){
    if um.registers[c] as usize == 0{
        panic!("Instruction is trying to unmap $m[0]")
    }else{
        um.unmap_index_values.push(um.registers[c] as usize);
    }
    
}

/// Outputs a specified value
/// Only valid values to output between 0 and 255
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * c: The c register
pub fn output(um: &mut UmState, c: usize){
    let value = u8::try_from(um.registers[c]).unwrap();
    let mut buffer = std::io::stdout();
    match buffer.write(&[value]).unwrap() {
        1 =>{
            stdout().flush().unwrap();
        },
        _ =>{
            panic!("Wrong output value")
        }
    }
}

/// Reads an input from standard in
/// When the input arrives, $r[c] is loaded with the input
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * c: The c register
pub fn input(um: &mut UmState, c: usize){
    let mut input = [0_u8; 1];
    let mut number = stdin();
    um.registers[c] = match number.read(&mut input).expect("Failed to read line") {
        1 =>{
            input[0] as u32
        },
        _ => {
            u32::MAX
        }
    }
}

/// Performs the load program
/// Segment $m[$r[b]] is duplicated, and the duplicate replaces $m[0]
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * b: The b register
/// * c: The c register
pub fn load_program(um: &mut UmState, b: usize, c: usize){
    um.program_counter = um.registers[c] as usize;
    if um.registers[b] != 0{
        let new_segment = um.memory[um.registers[b] as usize].clone();
        um.memory[0] = new_segment;
    }
}

/// Loads a value
/// 
/// # Arguments:
/// * um: A Virtual Machine object
/// * rl: The a register
/// * vl: The value
pub fn load_value(um: &mut UmState, rl: usize, vl: u32){
    um.registers[rl] = vl;
}
