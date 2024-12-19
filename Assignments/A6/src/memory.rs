/// Invariants:
/// 
/// Invariant: The instruction set of the machine is consistent and does 
/// not change regardless of the specific program being executed.
/// 
/// Invariant: The semantics of the instructions are consistent and do not 
/// change. For example, an “add” instruction will always perform an 
/// addition operation.
/// 
/// Invariant: The state of the machine (e.g., the values in registers or 
/// memory) after executing an instruction sequence starting from a 
/// certain state is an invariant. It does not depend on the specific 
/// path taken to reach that state, only on the initial state and the 
/// sequence of instructions.

use std::convert::TryInto;
use std::io::{self, stdin, stdout, BufRead, BufReader, Read, Write};
use std::fs::File;

pub struct UmState{
    pub registers: Vec<u32>,
    pub memory: Vec<Vec<u32>>,
    pub unmap_index_values: Vec<usize>,
    pub program_counter: usize
}

type Umi = u32;
pub struct Field { 
    width: u32,
    lsb: u32,
}
pub static RA: Field = Field {width: 3, lsb: 6}; 
pub static RB: Field = Field {width: 3, lsb: 3}; 
pub static RC: Field = Field {width: 3, lsb: 0}; 
pub static RL: Field = Field {width: 3, lsb: 25}; 
pub static VL: Field = Field {width: 25, lsb: 0}; 
pub static OP: Field = Field {width: 4, lsb: 28};

fn mask(bits: u32) -> u32 { (1 << bits) - 1 }

pub fn get(field: &Field, instruction: Umi) -> u32 { 
    (instruction >> field.lsb) & mask(field.width)
}

/// Function to convert binary files into instructioins.
///
/// Arguments
/// * `input`: binary file input.
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn BufRead> = match input {
        None => Box::new(BufReader::with_capacity(1<<16, io::stdin())),
        Some(filename) => match File::open(filename) {
            Ok(file) => Box::new(BufReader::with_capacity(1<<16, file)),
            Err(_) => return Vec::new(),  // Return an empty vector on error
        },
    };
    let mut buf = Vec::<u8>::new();
    if let Err(_) = raw_reader.read_to_end(&mut buf) {
        return Vec::new();  // Return an empty vector on error
    }
    let instructions: Vec<u32> = buf
        .chunks_exact(4)
        .map(|x| u32::from_be_bytes(x.try_into().unwrap()))
        .collect();

    instructions
}

/// Function to perform the desired instructions.
///
/// Arguments:
/// * `instructions`: A vector of instructions.
pub fn instructs(instructions: Vec<u32>){
    let registers: Vec<u32> = vec![0; 8];
    let program_counter = 0;
    let mut memory: Vec<Vec<u32>> = vec![];
    memory.push(instructions.clone());
    let unmap_index_values: Vec<usize> = vec![];

    let mut um = UmState{
        registers,
        memory,
        unmap_index_values,
        program_counter
    };

    if um.program_counter > 0{
        panic!("Program Counter outside the bounds of $m[0]")
    }

    if get(&OP, um.memory[0][um.program_counter]) > 13{
        panic!("Word being pointed to does not code for valid instructions")
    }

    loop{
        let instruction = um.memory[0][um.program_counter];

        let opcode = get(&OP, instruction);
        let b = (get(&RB, instruction)) as usize;
        let c = (get(&RC, instruction)) as usize;
        um.program_counter += 1;

        if opcode == 0{
            let a = (get(&RA, instruction)) as usize;
            if um.registers[c] != 0{
                um.registers[a] = um.registers[b];
            }
        }
        if opcode == 1{
            let a = (get(&RA, instruction)) as usize;
            um.registers[a] = um.memory[um.registers[b] as usize][um.registers[c] as usize];
        }
        if opcode == 2{
            let a = (get(&RA, instruction)) as usize;
            um.memory[um.registers[a] as usize][um.registers[b] as usize] = um.registers[c];
        }
        if opcode == 3{
            let a = (get(&RA, instruction)) as usize;
            um.registers[a] = um.registers[b].wrapping_add(um.registers[c]);
        }
        if opcode == 4{
            let a = (get(&RA, instruction)) as usize;
            um.registers[a] = um.registers[b].wrapping_mul(um.registers[c]);
        }
        if opcode == 5{
            let a = (get(&RA, instruction)) as usize;
            if um.registers[c] == 0{
                panic!("Cannot divide by 0")
            }
            um.registers[a] = um.registers[b] / um.registers[c];
        }
        if opcode == 6{
            let a = (get(&RA, instruction)) as usize;
            um.registers[a] = !(um.registers[b] & um.registers[c]);
        }
        if opcode == 7{
            std::process::exit(0);
        }
        if opcode == 8{
            let new_segment = vec![0_u32; um.registers[c] as usize];
        
            if um.unmap_index_values.len() != 0{
                um.registers[b] = (um.unmap_index_values.pop().unwrap()) as u32;
        
                um.memory[um.registers[b] as usize] = new_segment;
            }else {
                um.memory.push(new_segment);
                um.registers[b] = (um.memory.len() - 1) as u32;
            }
        }
        if opcode == 9{
            if um.registers[c] as usize == 0{
                panic!("Instruction is trying to unmap $m[0]")
            }else{
                um.unmap_index_values.push(um.registers[c] as usize);
            }
        }
        if opcode == 10{
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
        if opcode == 11{
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
        if opcode == 12{
            um.program_counter = um.registers[c] as usize;
    
            if um.registers[b] != 0{
                let new_segment = &um.memory[um.registers[b] as usize];
                um.memory[0] = (new_segment).to_vec();
            }
        }
        if opcode == 13{
            let rl = (get(&RL, instruction)) as usize;
            let vl = get(&VL, instruction);
            um.registers[rl] = vl;
        }
    }
}
