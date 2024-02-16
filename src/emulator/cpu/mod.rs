pub mod memory;
pub mod registers;

use std::fmt;

use anyhow::Result;

use debugger::DebuggerCommand;
use memory::MemoryBus;
use registers::{RegisterFile32Bit, RegisterMapping};

use self::memory::STACK_CEILING;

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
    /// The programs stdout
    pub output: String,
}

impl Cpu32Bit {
    /// Load the given program into the CPU's memory and set the program counter to the given entrypoint.
    ///
    /// also resets the CPU's registers and memory to their default state
    pub fn new(text: &[u8], data: &[u8], entrypoint: u32, gp: Option<u32>) -> Self {
        // init registers
        let mut registers = RegisterFile32Bit::new();
        // set the stack pointer to the top of the stack (highest address in the stack region)
        registers[RegisterMapping::Sp] = STACK_CEILING;
        // set the return address to the start of the text region, this will be overwritten by
        // structs using this register file (e.g. the CPU) upon loading a program
        registers[RegisterMapping::Ra] = entrypoint;
        if let Some(gp) = gp {
            registers[RegisterMapping::Gp] = gp;
        }

        Self {
            registers,
            pc: entrypoint,
            memory: MemoryBus::new(entrypoint, text, data),
            debug: false,
            output: String::new(),
        }
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
        self.execute(instruction)?;
        println!("\n Program Output: {}", self.output);

        Ok(())
    }
}

impl fmt::Display for Cpu32Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CPU32Bit {{\n")?;
        write!(f, "    memory bus layout: {{\n")?;
        write!(f, "        text: {{\n")?;
        write!(
            f,
            "            start: {:#010x},\n",
            self.memory.entrypoint()
        )?;
        write!(f, "            size: {}\n", self.memory.code_size())?;
        write!(f, "        }},\n")?;
        write!(f, "        data: {{\n")?;
        write!(
            f,
            "            start: {:#010x},\n",
            self.memory.dram_start()
        )?;
        write!(f, "            size: {}\n", self.memory.dram_size())?;
        write!(f, "        }},\n")?;
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
