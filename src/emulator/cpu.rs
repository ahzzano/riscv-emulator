use std::usize;

use crate::emulator::{
    memory::{self, Memory},
    types::Instruction,
};

#[derive(Debug, Default)]
pub struct CPU {
    registers: [u32; 32],
    memory: Box<Memory>,
    mem_size: u32,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: [0; 32],
            memory: Box::new(memory::Memory::default()),
            mem_size: 512,
        }
    }

    pub fn get_register(&self, reg_n: u8) -> Option<u32> {
        if reg_n >= 32 {
            None
        } else {
            Some(self.registers[reg_n as usize])
        }
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

        for addr in (0..mmap_size).map(|i| start + (i * 4)) {
            println!("map: {addr}");
            self.memory
                .write_u32(addr as u32, memmap[(addr / 4) - start]);
        }
    }
}

