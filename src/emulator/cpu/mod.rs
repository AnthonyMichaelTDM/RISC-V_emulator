pub mod memory;
pub mod registers;

use memory::{MemoryBus, STACK_CEILING};
use registers::{RegisterFile32Bit, RegisterMapping};

use self::memory::{HEAP_BASE, TEXT_BASE};

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

impl Cpu32Bit {
    #[must_use]
    pub fn initialize(text: Vec<u8>, data: Vec<u8>, entrypoint: u32) -> Self {
        // initialize the register file
        let mut registers = RegisterFile32Bit::new();

        // set the stack pointer to the top of the stack (highest address in the stack region)
        registers.write(RegisterMapping::Sp, STACK_CEILING);
        // set the return address to the entrypoint
        registers.write(RegisterMapping::Ra, entrypoint + TEXT_BASE);
        // set the global pointer to the start of the heap
        registers.write(RegisterMapping::Gp, HEAP_BASE);

        let mut memory = MemoryBus::new();
        memory.initialize_dram(&data);
        memory.initialize_text(&text);

        Self {
            registers,
            pc: entrypoint,
            memory,
        }
    }
}
