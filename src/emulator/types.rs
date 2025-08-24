use std::marker::PhantomData;

// Instruction Types
#[derive(Debug)]
pub struct R;
pub struct I;
pub struct S;
pub struct B;
pub struct U;
pub struct J;

pub enum AnyInstruction {
    R(Instruction<R>),
    I(Instruction<I>),
    S(Instruction<S>),
    B(Instruction<B>),
    U(Instruction<U>),
    J(Instruction<J>),
    Syscall,
}

#[derive(Debug)]
pub struct Instruction<T> {
    instruction: u32,
    instruction_type: PhantomData<T>,
}

pub fn encode(code: u32) -> Option<AnyInstruction> {
    // let instr = Instruction::new(code);
    let opcode = code & 0b1111111;

    match opcode {
        0b0110011 => Some(AnyInstruction::R(Instruction::new(code))),
        0b0010011 => Some(AnyInstruction::I(Instruction::new(code))),
        0b0000011 => Some(AnyInstruction::S(Instruction::new(code))),
        0b1110011 => Some(AnyInstruction::Syscall),
        _ => None,
    }
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

impl Instruction<R> {
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

impl Instruction<I> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn imm(&self) -> i16 {
        (self.instruction >> 20) as i16
    }
}

impl Instruction<S> {
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn rs2(&self) -> u8 {
        (self.instruction >> 20 & 0b11111) as u8
    }
    pub fn imm(&self) -> i16 {
        let p1 = (self.instruction >> 7 & 0b11111) as i16;
        let p2 = (self.instruction >> 25) as i16;
        let offset_p2 = p2 << 5;
        offset_p2 | p1
    }
}

impl Instruction<B> {
    pub fn funct3(&self) -> u8 {
        (self.instruction >> 12 & 0b111) as u8
    }
    pub fn rs1(&self) -> u8 {
        (self.instruction >> 15 & 0b11111) as u8
    }
    pub fn rs2(&self) -> u8 {
        (self.instruction >> 20 & 0b11111) as u8
    }

    pub fn imm(&self) -> i16 {
        let p1 = (self.instruction >> 7 & 0b11111) as i16;
        let b11 = (p1 & 1) << 11; // bit 11
        let p1 = p1 >> 1; // bits 0-4
        let p2 = (self.instruction >> 25) as i16; // bits 5-10, 12
        let b12 = (p2 >> 6) << 12;
        let p2 = p2 & 0b011111;

        p1 | b11 | p2 | b12
    }
}

impl Instruction<U> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }
    pub fn imm(&self) -> u32 {
        self.instruction >> 12
    }
}

impl Instruction<J> {
    pub fn rd(&self) -> u8 {
        (self.instruction >> 7 & 0b11111) as u8
    }

    pub fn imm(&self) -> i32 {
        let imm20 = ((self.instruction >> 31) & 0x1) << 20;
        let imm10_1 = ((self.instruction >> 21) & 0x3FF) << 1;
        let imm11 = ((self.instruction >> 20) & 0x1) << 11;
        let imm19_12 = ((self.instruction >> 12) & 0xFF) << 12;

        // Combine
        let imm = (imm20 | imm19_12 | imm11 | imm10_1) as i32;
        (imm << 11) >> 11
    }
}

#[cfg(test)]
mod test {
    use crate::emulator::types::{B, I, Instruction, J, R, S, U};

    #[test]
    fn rtype_instructions() {
        // add s2, s3, s4 -- s2 = s3 + s4
        // in machine code
        // 00110011 10100 10011 000 10010 0110011
        let add: Instruction<R> = Instruction::new(0x01498933);

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
        let addi: Instruction<I> = Instruction::new(0x025a8a13);
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
        let sb: Instruction<S> = Instruction::new(0x013902A3);
        assert_eq!(sb.opcode(), 0b0100011);
        assert_eq!(sb.imm(), 0x05);
        assert_eq!(sb.funct3(), 0);
        assert_eq!(sb.rs1(), 18);
        assert_eq!(sb.rs2(), 19);
    }

    #[test]
    fn btype_instructions() {
        // beq x5, x6, 16 <-- arb. offset, ignore
        // RS1 = 00101
        // RS2 = 00110
        // IMM: 1 0000
        //
        let beq: Instruction<B> = Instruction::new(0x02628063);

        assert_eq!(beq.opcode(), 0b1100011);
        assert_eq!(beq.funct3(), 0);
        assert_eq!(beq.rs1(), 5);
        assert_eq!(beq.rs2(), 6);
        assert_eq!(beq.imm(), 1);
    }

    #[test]
    fn utype_instructions() {
        let lui: Instruction<U> = Instruction::new(0x12345037);

        assert_eq!(lui.imm(), 0x12345)
    }

    #[test]
    fn jtype_instructions() {
        let jal_p: Instruction<J> = Instruction::new(0x00C000EF);
        assert_eq!(jal_p.imm(), 12);

        let jal_n: Instruction<J> = Instruction::new(0xFF9FF0EF);
        assert_eq!(jal_n.imm(), -8);
    }
}
