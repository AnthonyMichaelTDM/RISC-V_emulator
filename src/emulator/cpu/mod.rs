pub mod memory;
pub mod registers;

use std::fmt;

use anyhow::Result;

use memory::MemoryBus;
use registers::{RegisterFile32Bit, RegisterMapping};

use self::memory::TEXT_BASE;

use super::{execute::Execute32BitInstruction as _, fetch::Fetch32BitInstruction as _};

/// the number of registers in the RISC-V ISA
pub const REGISTERS_COUNT: u8 = 32;

/// The size of a memory access.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
pub enum Size {
    Byte = 8,
    Half = 16,
    Word = 32,
}

#[allow(clippy::module_name_repetitions)]
pub struct Cpu32Bit {
    pub registers: RegisterFile32Bit,
    pub pc: u32,
    pub memory: MemoryBus,
}

impl Default for Cpu32Bit {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu32Bit {
    /// Create a new CPU with the default state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            registers: RegisterFile32Bit::new(),
            pc: 0,
            memory: MemoryBus::new(),
        }
    }

    /// Load the given program into the CPU's memory and set the program counter to the given entrypoint.
    ///
    /// also resets the CPU's registers and memory to their default state
    pub fn load(&mut self, text: &[u8], data: &[u8], entrypoint: u32) {
        // reset the program counter
        self.pc = entrypoint + TEXT_BASE;

        // reset registers
        self.registers.reset();
        // set the return address to the entrypoint
        self.registers
            .write(RegisterMapping::Ra, entrypoint + TEXT_BASE);

        // reset memory
        self.memory.clear();
        self.memory.initialize_dram(data);
        self.memory.initialize_text(text);
    }

    /// Execute the current instruction and update the program counter.
    /// This method will fetch, decode, and execute the instruction at the current program counter.
    /// It will then update the program counter to the next instruction, branch, or jump as necessary.
    ///
    /// # Errors
    ///
    /// This method will return an error if the instruction cannot be fetched, decoded, or executed.
    /// This can happen if the program counter is out of bounds or misaligned, if the instruction is invalid or
    /// results in an invalid memory/register read / write, if a zero pointer is dereferenced, etc.
    pub fn step(&mut self) -> Result<()> {
        // fetch and decode the instruction
        let instruction = self.memory.fetch_and_decode(self.pc)?;

        // execute the instruction, updating the CPU's state as necessary (e.g. updating registers and memory, incrementing the program counter, etc.)
        self.execute(instruction)
    }
}

impl fmt::Display for Cpu32Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPU32Bit {{\n")?;
        write!(f, "    pc: {:#08x},\n", self.pc)?;

        write!(f, "    context: {{\n")?;
        // print the 4 instructions before the current instruction
        for offset in (1..=4).rev() {
            let addr = self.pc.wrapping_sub(offset * 4);
            if let Ok(instruction) = self.memory.fetch_and_decode(addr) {
                write!(f, "        {:#08x}: {},\n", addr, instruction)?;
            } else {
                write!(f, "        {:#08x}: <invalid instruction>,\n", addr)?;
            }
        }
        write!(
            f,
            "   ---> {:#08x}: {},\n",
            self.pc,
            if let Ok(instruction) = self.memory.fetch_and_decode(self.pc) {
                format!("{}", instruction)
            } else {
                "<invalid instruction>".to_string()
            }
        )?;
        // print the 4 instructions after the current instruction
        for offset in 1..=4 {
            let addr = self.pc.wrapping_add(offset * 4);
            if let Ok(instruction) = self.memory.fetch_and_decode(addr) {
                write!(f, "        {:#08x}: {},\n", addr, instruction)?;
            } else {
                write!(f, "        {:#08x}: <invalid instruction>,\n", addr)?;
            }
        }
        write!(f, "    }},\n")?;
        write!(f, "    registers: {{")?;
        write!(
            f,
            "    {}\n",
            format!("{}", self.registers).replace("\n", "\n    ")
        )?;
        write!(f, "}}")
    }
}
