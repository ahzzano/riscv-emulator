pub mod emulator;

use emulator::types::*;

use crate::emulator::cpu::CPU;

fn main() {
    println!("Hello, world!");
    let a: Instruction<RType> = emulator::types::Instruction::new(0x123456u32);
    let a_opcode = a.opcode();
    println!("{a_opcode:?}");

    let mut system = CPU::default();
    // system.write_memory_map(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 0);
    system.write_memory_map_u32(vec![0x12345678, 0x01020304], 0);
    println!("{system:?}");
}
