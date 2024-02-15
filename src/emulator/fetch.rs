use anyhow::Result;

use crate::instruction_set_definition::Ri32imInstruction;

use super::{
    cpu::{Cpu32Bit, Size},
    decode::Decode32BitInstruction,
};

#[allow(clippy::module_name_repetitions)]
pub trait Fetch32BitInstruction {
    type InstructionSet;
    type PC;
    const INSTRUCTION_SIZE: Size;

    /// Fetch the instruction at the given program counter.
    /// and
    /// Decode the instruction into an Instruction of type `InstructionSet`.
    ///
    /// # Arguments
    ///
    /// * `pc` - The program counter to fetch the instruction from.
    ///
    /// # Errors
    ///
    /// Returns an error if the instruction cannot be fetched from the memory.
    /// this can happen if the memory is out of bounds, if the memory is not readable, if the memory is outside of the text segment, etc.
    fn fetch(&self, pc: Self::PC) -> Result<Self::InstructionSet>;
}

impl Fetch32BitInstruction for Cpu32Bit {
    type InstructionSet = Ri32imInstruction;
    type PC = u32;
    const INSTRUCTION_SIZE: Size = Size::Word;

    fn fetch(&self, pc: Self::PC) -> Result<Self::InstructionSet> {
        // read the instruction from memory
        let instruction = self.memory.read(pc, Self::INSTRUCTION_SIZE)?;
        // decode the instruction
        Ri32imInstruction::from_machine_code(instruction)
    }
}
