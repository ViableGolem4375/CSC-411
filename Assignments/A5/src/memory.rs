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
use crate::machine;
use crate::bitpack;

pub struct UmState {
    pub inst_count: u32,
    pub registers: Vec<u32>,
    pub memory: Vec<Vec<u32>>,
    pub memory_tracker: Vec<u32>,
}

/// Function to convert binary files into instructioins.
///
/// Arguments
/// * `input`: binary file input.
pub fn load(input: Option<&str>) -> Vec<u32> {
    let mut raw_reader: Box<dyn std::io::BufRead> = match input {
        None => Box::new(std::io::BufReader::new(std::io::stdin())),
        Some(filename) => Box::new(std::io::BufReader::new(std::fs::File::open(filename).unwrap(),)),
    };
    let mut buf = Vec::<u8>::new();
    raw_reader.read_to_end(&mut buf).unwrap();
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
pub fn instructs(instructions: Vec<u32>) {
    let inst_count = 0;
    let registers: Vec<u32> = vec![0; 8];
    let memory: Vec<Vec<u32>> = Vec::new();
    let memory_tracker: Vec<u32> = Vec::new();  

    let mut state = UmState {
        inst_count,
        registers,
        memory,
        memory_tracker,
    };
    
    state.memory.push(instructions.clone());

    while state.inst_count < state.memory[0].len() as u32 {
        let opcode: u32 = bitpack::getu(state.memory[0][state.inst_count as usize] as u64, 4, 28) as u32;
        let rega;
        let mut regb = 0;
        let mut regc = 0;
        let mut value = 0;
        
        if opcode == 13 {
            rega = bitpack::getu(state.memory[0][state.inst_count as usize] as u64, 3, 25) as u32;
            value = bitpack::getu(state.memory[0][state.inst_count as usize] as u64, 25, 0) as u32;
        } 
        else {
            rega = bitpack::getu(state.memory[0][state.inst_count as usize] as u64, 3, 6) as u32;
            regb = bitpack::getu(state.memory[0][state.inst_count as usize] as u64, 3, 3) as u32;
            regc = bitpack::getu(state.memory[0][state.inst_count as usize] as u64, 3, 0) as u32;
        }
        match opcode {
            0 => machine::cmov(&mut state, rega as usize, regb as usize, regc as usize),
            1 => machine::sload(&mut state, rega as usize, regb as usize, regc as usize),
            2 => machine::store(&mut state, rega as usize, regb as usize, regc as usize),
            3 => machine::add(&mut state, rega as usize, regb as usize, regc as usize),
            4 => machine::mul(&mut state, rega as usize, regb as usize, regc as usize),
            5 => machine::div(&mut state, rega as usize, regb as usize, regc as usize),
            6 => machine::nand(&mut state, rega as usize, regb as usize, regc as usize),
            7 => break, //This is the halt opcode.
            8 => machine::map_seg(&mut state, regb as usize, regc as usize),
            9 => machine::unmap_seg(&mut state, regc as usize),
            10 => machine::output(&mut state, regc as usize),
            11 => machine::input(&mut state, regc as usize),
            12 => machine::load_program(&mut state, regb as usize, regc as usize),
            13 => machine::load_value(&mut state, rega as usize, value as usize),
            _ => panic!("Invalid OpCode {}", opcode)
        }
    }
}
