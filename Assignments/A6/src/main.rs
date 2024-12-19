mod memory;
pub mod machine;
use std::env;

/// Main function to run the program.
/// 
/// Arguments:
/// 
fn main() {
    let input = env::args().nth(1);
    let instructions: Vec<u32> = memory::load(input.as_deref());
    memory::instructs(instructions);
}
