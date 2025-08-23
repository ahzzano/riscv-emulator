use std::marker::PhantomData;

// Instruction Types
pub struct RType;
pub struct IType;
pub struct SType;
pub struct BType;
pub struct UType;
pub struct JType;

#[derive(Debug)]
pub struct Instruction<T> {
    instruction: u32,
    instruction_type: PhantomData<T>,
}

impl<T> Instruction<T> {
    pub fn opcode(&self) -> u8 {
        (self.instruction & 0b1111111) as u8
    }

    pub fn new(code: u32) -> Self {
        Self {
            instruction: code,
            instruction_type: PhantomData,
        }
    }
}

impl Instruction<RType> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn rs2(&self) -> u8 {
        (self.instruction >> 20 & 0b11111) as u8
    }
    pub fn funct7(&self) -> u8 {
        (self.instruction >> 25) as u8
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::types::{Instruction, RType};

    #[test]
    fn rtype_instructions() {
        // add s2, s3, s4 -- s2 = s3 + s4
        // in machine code
        // 00110011 10100 10011 000 10010 0110011
        // 011001110100100110000100100110011
        let add: Instruction<RType> = Instruction::new(0x01498933);

        assert_eq!(add.opcode(), 0b0110011);
        assert_eq!(add.rd(), 0b10010);
        assert_eq!(add.funct3(), 0x0);
        assert_eq!(add.rs1(), 0b10011);
        assert_eq!(add.rs2(), 0b10100);
        assert_eq!(add.funct7(), 0x0)
    }
}
