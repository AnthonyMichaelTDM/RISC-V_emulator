/*
MIT License

Copyright (c) 2024 Anthony Rubick

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use anyhow::{bail, Result};

use crate::instruction_set_definition::Rv32imInstruction;

use super::{
    cpu::{memory::MemoryBus, Size},
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
    fn fetch_and_decode(&self, pc: Self::PC) -> Result<Self::InstructionSet>;
}

impl Fetch32BitInstruction for MemoryBus {
    type InstructionSet = Rv32imInstruction;
    type PC = u32;
    const INSTRUCTION_SIZE: Size = Size::Word;

    fn fetch_and_decode(&self, pc: Self::PC) -> Result<Self::InstructionSet> {
        if pc.wrapping_sub(self.entrypoint()) >= self.code_size() {
            bail!("Program counter out of bounds: {:#010x}", pc);
        }

        // read the instruction from memory
        let instruction = self.read(pc, Self::INSTRUCTION_SIZE)?;
        // decode the instruction
        Rv32imInstruction::from_machine_code(instruction)
    }
}
