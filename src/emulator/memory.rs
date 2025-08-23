#[derive(Debug)]
pub struct Memory {
    mem: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self { mem: vec![0; 512] }
    }
}

impl Memory {
    pub fn new(mem_size: u32) -> Self {
        // assume 128 different addresses for the time being
        Self {
            mem: vec![0; mem_size as usize],
        }
    }

    pub fn read(&self, addr: u32) -> u32 {
        let a = self.mem[addr as usize];
        let b = self.mem[(addr + 1) as usize];
        let c = self.mem[(addr + 2) as usize];
        let d = self.mem[(addr + 3) as usize];
        u32::from_le_bytes([a, b, c, d])
    }

    pub fn write(&mut self, addr: u32, content: u8) {
        self.mem[addr as usize] = content;
    }
}

