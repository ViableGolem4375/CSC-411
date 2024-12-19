use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use bitpack;

type Umi = u32;

pub struct Field {
    width: u32,
    lsb: u32,
}

static RA: Field = Field { width: 3, lsb: 6 };
static RB: Field = Field { width: 3, lsb: 3 };
static RC: Field = Field { width: 3, lsb: 0 };
static RL: Field = Field { width: 3, lsb: 25 };
static VL: Field = Field { width: 25, lsb: 0 };
static OP: Field = Field { width: 4, lsb: 28 };

/// Given a `field` and `instruction`, extract
/// that field from the instruction as a u32
pub fn get(field: &Field, instruction: Umi) -> u32 {
    bitpack::getu(instruction as u64, field.width as u64, field.lsb as u64).unwrap() as u32
}

/// Given an instruction word, extract the opcode
fn op(instruction: Umi) -> Option<Opcode> {
    FromPrimitive::from_u32(bitpack::getu(instruction as u64, OP.width as u64, OP.lsb as u64).unwrap() as u32)
}

pub fn disassemble(inst: Umi) -> String {

    match op(inst) {
        Some(Opcode::CMov) => {
            format!(
                "if (r{} != 0) r{} := r{};",
                get(&RC, inst),
                get(&RA, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::Load) => {
            format!(
                "r{} := m[r{}][r{}];",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Store) => {
            format!(
                "m[r{}][r{}] := r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Add) => {
            format!(
                "r{} := r{} + r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Mul) => {
            format!(
                "r{} := r{} * r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Div) => {
            format!(
                "r{} := r{} / r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }

        Some(Opcode::Nand) => {
            format!(
                "r{} := r{} nand r{};",
                get(&RA, inst),
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::Halt) => "halt".to_string(),
        Some(Opcode::MapSegment) => {
            format!(
                "r{} := map segment (r{} words);",
                get(&RB, inst),
                get(&RC, inst)
            )
        }
        Some(Opcode::UnmapSegment) => {
            format!("unmap r{};", get(&RC, inst))
        }
        Some(Opcode::Output) => {
            format!("output r{};", get(&RC, inst))
        }
        Some(Opcode::Input) => {
            format!("r{} := input();", get(&RC, inst))
        }
        Some(Opcode::LoadProgram) => {
            format!(
                "goto r{} in program m[r{}];",
                get(&RC, inst),
                get(&RB, inst)
            )
        }
        Some(Opcode::LoadValue) => {
            format!("r{} := {};", get(&RL, inst), get(&VL, inst))
        }

        _ => {
            format!(".data 0x{:08x}", inst)
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone, FromPrimitive)]
#[repr(u32)]
enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
}
