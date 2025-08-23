use std::{any::TypeId, marker::PhantomData};

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

impl Instruction<IType> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn imm(&self) -> u16 {
        (self.instruction >> 20) as u16
    }
}

impl Instruction<SType> {
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn rs2(&self) -> u8 {
        (self.instruction >> 20 & 0b11111) as u8
    }
    pub fn imm(&self) -> u16 {
        let p1 = (self.instruction >> 7 & 0b11111) as u16;
        let p2 = (self.instruction >> 25) as u16;
        let offset_p2 = p2 << 5;
        offset_p2 | p1
    }
}

impl Instruction<BType> {
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn rs2(&self) -> u8 {
        (self.instruction >> 20 & 0b11111) as u8
    }
}

impl Instruction<UType> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }
}

impl Instruction<JType> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::types::{IType, Instruction, RType, SType};

    #[test]
    fn rtype_instructions() {
        // add s2, s3, s4 -- s2 = s3 + s4
        // in machine code
        // 00110011 10100 10011 000 10010 0110011
        let add: Instruction<RType> = Instruction::new(0x01498933);

        assert_eq!(add.opcode(), 0b0110011);
        assert_eq!(add.rd(), 0b10010);
        assert_eq!(add.funct3(), 0x0);
        assert_eq!(add.rs1(), 0b10011);
        assert_eq!(add.rs2(), 0b10100);
        assert_eq!(add.funct7(), 0x0)
    }

    #[test]
    fn itype_instructions() {
        // add s4, s5, 0x25
        // 20, 21
        // MC: 000000110101 10101 000 10100 0010011
        // MC: 0000 0011 0101 1010 1000 1010 0001 0011
        // 025A8A13
        let addi: Instruction<IType> = Instruction::new(0x025a8a13);
        assert_eq!(addi.opcode(), 0b0010011);
        assert_eq!(addi.rd(), 0b10100);
        assert_eq!(addi.funct3(), 0b000);
        assert_eq!(addi.rs1(), 0b10101);
        assert_eq!(addi.imm(), 0x25);
    }

    #[test]
    fn stype_instructions() {
        // sw s3, 5(s2)
        // 19, 18
        // MC: 0000000 10011 10010 000 00101 0100011
        // MC: 00000001 0011 1001 0000 0010 1010 0011
        let sb: Instruction<SType> = Instruction::new(0x013902A3);
        assert_eq!(sb.opcode(), 0b0100011);
        assert_eq!(sb.imm(), 0x05);
        assert_eq!(sb.funct3(), 0);
        assert_eq!(sb.rs1(), 18);
        assert_eq!(sb.rs2(), 19);
    }
}
