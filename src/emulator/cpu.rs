use std::usize;

use crate::emulator::{
    memory::{self, Memory},
    types::{AnyInstruction, Instruction, encode},
};

const THREAD_POINTER: usize = 0x4;

#[derive(Debug, Default)]
pub struct CPU {
    registers: [u32; 32],
    memory: Box<Memory>,
    mem_size: u32,
    starting_pc: u32,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 32],
            memory: Box::new(memory::Memory::default()),
            mem_size: 512,
            starting_pc: 12,
        }
    }

    pub fn get_register(&self, reg_n: u8) -> Option<u32> {
        if reg_n >= 32 {
            None
        } else {
            Some(self.registers[reg_n as usize])
        }
    }

    pub fn write_reg(&mut self, reg: usize, value: u32) {
        self.registers[reg] = value
    }

    pub fn write_memory_map(&mut self, memmap: Vec<u8>, start: u32) {
        let mmap_size = memmap.len();
        let a = start as usize;
        let b = mmap_size + (start as usize);

        for addr in a..b {
            self.memory.write(addr as u32, memmap[addr - a]);
        }
    }

    pub fn write_memory_map_u32(&mut self, memmap: Vec<u32>, start: usize) {
        let mmap_size = memmap.len();

        (0..mmap_size).for_each(|addr| {
            println!("map: {addr}");
            self.memory
                .write_u32((start + (addr * 4)) as u32, memmap[addr]);
        });
    }

    pub fn init(&mut self) {
        self.registers[THREAD_POINTER] = self.starting_pc;
        let pc = self.registers[THREAD_POINTER];
        println!("{pc}");
    }

    pub fn step(&mut self) {
        let pc = self.registers[THREAD_POINTER];
        let machine_code = self.memory.read(pc);
        let instr = encode(machine_code);

        if let Some(i) = instr {
            self.exec(i);
        }

        self.registers[THREAD_POINTER] += 4
    }

    fn exec(&mut self, instr: AnyInstruction) {
        match instr {
            AnyInstruction::R(instr) => match instr.funct3() {
                0x0 => {
                    // ADD
                    let rd = instr.rd();
                    let rs1 = self.registers[instr.rs1() as usize];
                    let rs2 = self.registers[instr.rs2() as usize];
                    self.registers[rd as usize] = rs1 + rs2;
                }
                _ => {
                    todo!()
                }
            },
            _ => {
                todo!()
            }
        }
    }
}

