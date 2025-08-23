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
        (self.instruction | 0b111111) as u8
    }

    pub fn new(code: u32) -> Self {
        Self {
            instruction: code,
            instruction_type: PhantomData,
        }
    }
}

impl Instruction<RType> {}
