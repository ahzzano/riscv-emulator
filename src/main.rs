pub mod emulator;

use emulator::types::*;

fn main() {
    println!("Hello, world!");
    let a: Instruction<RType> = emulator::types::Instruction::new(0x123456u32);
    let a_opcode = a.opcode();
    println!("{a_opcode:?}");
}
