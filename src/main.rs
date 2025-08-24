pub mod emulator;

use emulator::types::*;

use crate::emulator::cpu::CPU;

fn main() {
    println!("Hello, world!");
    let a: Instruction<R> = emulator::types::Instruction::new(0x123456u32);
    let a_opcode = a.opcode();
    println!("{a_opcode:?}");

    let mut system = CPU::new();
    system.write_memory_map_u32(vec![0x013904B3], 12);
    system.write_reg(0b10010, 1);
    system.init();
    system.step();
    println!("{system:?}");
}
