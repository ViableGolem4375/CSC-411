use std::io::Write;
use std::io::Read;
use crate::memory::UmState;

/// Move instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn cmov(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    if state.registers[regc] != 0 {
        state.registers[rega] = state.registers[regb];
    }
    state.inst_count += 1;
}

/// Load instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn sload(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    state.registers[rega] = state.memory[state.registers[regb] as usize][state.registers[regc] as usize];
    state.inst_count += 1;
}

/// Store instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn store(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    state.memory[state.registers[rega] as usize][state.registers[regb] as usize] = state.registers[regc];
    state.inst_count += 1;
}

/// Add instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn add(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    state.registers[rega] = state.registers[regb].wrapping_add(state.registers[regc]);
    state.inst_count += 1;
}

/// Multiply instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn mul(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    state.registers[rega] = state.registers[regb].wrapping_mul(state.registers[regc]);
    state.inst_count += 1;
}

/// Divide instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn div(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    state.registers[rega] = state.registers[regb] / state.registers[regc];
    state.inst_count += 1;
}

/// Bitwise NAND instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn nand(state: &mut UmState, rega:usize, regb:usize, regc:usize) {
    state.registers[rega] = !(state.registers[regb] & state.registers[regc]);
    state.inst_count += 1;
}

/// Map segment instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`regb`: Index of register B
pub fn map_seg(state: &mut UmState, regb:usize, regc:usize) {
    if state.memory_tracker.len() == 0 {
        let init = state.registers[regc] as usize;
        let mem_seg: Vec<u32> = vec![0; init];
        state.memory.push(mem_seg);
        state.registers[regb] = (state.memory.len() - 1) as u32;
    }
    else {
        let init = state.registers[regc] as usize;
        let mem_seg: Vec<u32> = vec![0; init];
        let mem_pos = state.memory_tracker.pop();
        state.memory[mem_pos.unwrap() as usize]= mem_seg; 
        state.registers[regb] = mem_pos.unwrap();
    }
    state.inst_count += 1;
}

/// Unmap segment instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rebc`: Index of register C
pub fn unmap_seg(state: &mut UmState, regc:usize) {
    state.memory_tracker.push(state.registers[regc]);
    state.inst_count += 1;
}

/// Output instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rebc`: Index of register C
pub fn output(state: &mut UmState, regc: usize) {
        let out_word: [u8; 1] = [state.registers[regc] as u8];
        std::io::stdout().write(&out_word).ok();
        state.inst_count += 1;
}

/// Input instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rebc`: Index of register C
pub fn input(state: &mut UmState, regc: usize) {
    let mut take_in: [u8; 1] = [0; 1];
    match std::io::stdin().read(&mut take_in) {
        Ok(0) => {
            state.registers[regc] = u32::MAX;
        }
        Ok(_) => {
            state.registers[regc] = take_in[0] as u32;
        }
        Err(_) => {
            panic!();
        }
    }
    state.inst_count += 1;
}

/// Load program instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`regb`: Index of register B
/// *`rebc`: Index of register C
pub fn load_program(state: &mut UmState, regb: usize, regc: usize) {
    state.inst_count = state.registers[regc];
    if state.registers[regb] != 0 {
        state.memory[0] = state.memory[state.registers[regb] as usize].clone();
    }
}

/// Load value instruction.
/// 
/// Arguments:
/// *`state`: A struct containing the registers, memory, instance counter, and memory tracker.
/// *`rega`: Index of register A
/// *`value`: The value being loaded.
pub fn load_value(state: &mut UmState, rega: usize, value: usize) {
    state.registers[rega] = value as u32;
    state.inst_count += 1;
}
