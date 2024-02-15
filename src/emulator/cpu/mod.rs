pub mod memory;
pub mod registers;

use memory::MemoryBus;
use registers::{RegisterFile32Bit, RegisterMapping};

use self::memory::TEXT_BASE;

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
        self.pc = entrypoint;

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
}
