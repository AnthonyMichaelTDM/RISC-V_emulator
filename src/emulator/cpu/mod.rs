pub mod memory;
pub mod registers;

use std::fmt;

use anyhow::Result;

use debugger::DebuggerCommand;
use memory::MemoryBus;
use registers::{RegisterFile32Bit, RegisterMapping};

use crate::instruction_set_definition::{operations::ITypeOperation, Ri32imInstruction};

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
    /// Whether the CPU should pause before executing the next instruction.
    pub debug: bool,
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
            debug: false,
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
    /// # Side effects
    ///
    /// This method will update the CPU's state, including the program counter, registers, and memory.
    ///
    /// If the debug flag is set, this method will also print the CPU's state to the console,
    /// and start the debugger.
    /// The debugger will also start if the instruction is an ebreak.
    ///
    /// # Errors
    ///
    /// This method will return an error if the instruction cannot be fetched, decoded, or executed.
    /// This can happen if the program counter is out of bounds or misaligned, if the instruction is invalid or
    /// results in an invalid memory/register read / write, if a zero pointer is dereferenced, etc.
    pub fn step(&mut self) -> Result<()> {
        // fetch and decode the instruction
        let instruction = self.memory.fetch_and_decode(self.pc)?;

        // if the instruction is an ebreak,
        // enter debugger mode
        if let Ri32imInstruction::IType {
            operation: ITypeOperation::Ebreak,
            ..
        } = instruction
        {
            // pause execution and wait for user input
            self.debug = true;
        }

        if self.debug {
            debugger::clear_screen();
            debugger::print_screen(self);
            println!();
            // pause execution until user input is received
            // this is useful for debugging, as it allows the user to inspect the CPU's state at each step
            // and to step through the program one instruction at a time
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                match DebuggerCommand::from(input.trim()) {
                    DebuggerCommand::ContinueToNextBreakpoint => {
                        self.debug = false;
                        break;
                    }
                    DebuggerCommand::StepToNextInstruction => {
                        break;
                    }
                    DebuggerCommand::ExitProgram => {
                        anyhow::bail!("User requested to quit");
                    }
                    DebuggerCommand::Unknown => {
                        debugger::clear_screen();
                        debugger::print_screen(self);
                        println!("Unknown command: {}", input.trim());
                    }
                }
            }
        }

        // execute the instruction, updating the CPU's state as necessary (e.g. updating registers and memory, incrementing the program counter, etc.)
        self.execute(instruction)
    }
}

impl fmt::Display for Cpu32Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPU32Bit {{\n")?;
        write!(f, "    pc: {:#010x},\n", self.pc)?;
        write!(f, "    context: {{\n")?;
        // print the 4 instructions before the current instruction
        for offset in (1..=4).rev() {
            let addr = self.pc.wrapping_sub(offset * 4);
            if let Ok(instruction) = self.memory.fetch_and_decode(addr) {
                write!(f, "        {:#010x}: {},\n", addr, instruction)?;
            } else {
                write!(f, "        {:#010x}: <invalid instruction>,\n", addr)?;
            }
        }
        write!(
            f,
            "   ---> {:#010x}: {},\n",
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
                write!(f, "        {:#010x}: {},\n", addr, instruction)?;
            } else {
                write!(f, "        {:#010x}: <invalid instruction>,\n", addr)?;
            }
        }
        write!(f, "    }},\n")?;
        write!(f, "    registers: {{")?;
        write!(
            f,
            "    {}\n",
            self.registers.to_string().replace("\n", "\n        ")
        )?;
        write!(f, "    }},\n")?;
        write!(f, "}}")
    }
}

mod debugger {
    pub fn clear_screen() {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    }

    pub fn print_screen(cpu: &super::Cpu32Bit) {
        // print cpu state
        println!("CPU state:");
        println!("{}", cpu);
        //print instructions
        println!("Press 'c' to continue to the next breakpoint");
        println!("Press 's' or the Enter key to step to the next instruction");
        println!("Press 'q' to quit the program");
    }

    pub enum DebuggerCommand {
        ContinueToNextBreakpoint,
        StepToNextInstruction,
        ExitProgram,
        Unknown,
    }

    impl From<&str> for DebuggerCommand {
        fn from(s: &str) -> Self {
            match s.trim() {
                "c" => Self::ContinueToNextBreakpoint,
                "s" | "" => Self::StepToNextInstruction,
                "q" => Self::ExitProgram,
                _ => Self::Unknown,
            }
        }
    }
}
