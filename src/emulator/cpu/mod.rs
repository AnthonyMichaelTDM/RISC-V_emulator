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
    #[must_use]
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
            println!("Program Output:\n{}", self.output);
            println!();
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
                        println!("{}", self.output);
                        break;
                    }
                    DebuggerCommand::StepToNextInstruction => {
                        println!("{}", self.output);
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

        Ok(())
    }
}

impl fmt::Display for Cpu32Bit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "CPU32Bit {{")?;
        writeln!(f, "    memory bus layout: {{")?;
        writeln!(f, "        text: {{")?;
        writeln!(f, "            start: {:#010x},", self.memory.entrypoint())?;
        writeln!(f, "            size: {}", self.memory.code_size())?;
        writeln!(f, "        }},")?;
        writeln!(f, "        data: {{")?;
        writeln!(f, "            start: {:#010x},", self.memory.dram_start())?;
        writeln!(f, "            size: {}", self.memory.dram_size())?;
        writeln!(f, "        }},")?;
        writeln!(f, "    pc: {:#010x},", self.pc)?;
        writeln!(f, "    context: {{")?;
        // print the 4 instructions before the current instruction
        for offset in (1..=4).rev() {
            let addr = self.pc.wrapping_sub(offset * 4);
            if let Ok(instruction) = self.memory.fetch_and_decode(addr) {
                writeln!(f, "        {addr:#010x}: {instruction},")?;
            } else {
                writeln!(f, "        {addr:#010x}: <invalid instruction>,")?;
            }
        }
        writeln!(
            f,
            "   ---> {:#010x}: {},",
            self.pc,
            self.memory.fetch_and_decode(self.pc).map_or_else(
                |_| "<invalid instruction>".to_string(),
                |instruction| format!("{instruction}")
            )
        )?;
        // print the 4 instructions after the current instruction
        for offset in 1..=4 {
            let addr = self.pc.wrapping_add(offset * 4);
            if let Ok(instruction) = self.memory.fetch_and_decode(addr) {
                writeln!(f, "        {addr:#010x}: {instruction},")?;
            } else {
                writeln!(f, "        {addr:#010x}: <invalid instruction>,")?;
            }
        }
        writeln!(f, "    }},")?;
        write!(f, "    registers: {{")?;
        writeln!(
            f,
            "    {}",
            self.registers.to_string().replace('\n', "\n        ")
        )?;
        writeln!(f, "    }},")?;
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
        println!("{cpu}");
        //print instructions
        println!("Press 'c' to continue to the next breakpoint");
        println!("Press 's' or the Enter key to step to the next instruction");
        println!("Press 'q' to quit the program");
    }

    #[allow(clippy::module_name_repetitions)]
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
