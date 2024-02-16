#![allow(clippy::cast_possible_wrap)]
#![allow(clippy::cast_sign_loss)]
#![allow(clippy::cast_possible_truncation)]

use anyhow::{bail, Result};

use crate::instruction_set_definition::{
    operations::{
        ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
        UTypeOperation,
    },
    Rv32imInstruction,
};

use super::cpu::{
    memory::MemoryBus,
    registers::{RegisterFile32Bit, RegisterMapping},
    Cpu32Bit, Size,
};

#[allow(clippy::module_name_repetitions)]
pub trait Execute32BitInstruction {
    type InstructionSet;

    /// Execute the given instruction,
    /// and update the CPU's state accordingly.
    ///
    /// # Arguments
    ///
    /// * `instruction` - The instruction to execute.
    ///
    /// # Errors
    ///
    /// Returns an error if the instruction cannot be executed.
    /// This can happen if the instruction is invalid, if the instruction is not implemented, if the instruction results in an invalid memory/register read / write, etc.
    fn execute(&mut self, instruction: Self::InstructionSet) -> Result<()>;
}

impl Execute32BitInstruction for Cpu32Bit {
    type InstructionSet = Rv32imInstruction;

    fn execute(&mut self, instruction: Self::InstructionSet) -> Result<()> {
        match instruction {
            Self::InstructionSet::IType {
                operation,
                rd,
                funct3: _,
                rs1,
                imm,
            } => {
                execute_itype_instruction(
                    &mut self.debug,
                    &mut self.pc,
                    &mut self.output,
                    &mut self.registers,
                    &mut self.memory,
                    operation,
                    rd,
                    rs1,
                    imm,
                )?;
                if operation == ITypeOperation::Jalr {
                    // if the instruction is a jalr, the program counter is already updated
                    // by the execute_itype_instruction function
                    return Ok(());
                }
            }
            Self::InstructionSet::RType {
                operation,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => execute_rtype_instruction(&mut self.registers, operation, rd, rs1, rs2)?,
            Self::InstructionSet::SType {
                operation,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => execute_stype_instruction(
                &self.registers,
                &mut self.memory,
                operation,
                rs1,
                rs2,
                imm,
            )?,
            Self::InstructionSet::SBType {
                operation,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => {
                execute_sbtype_instruction(&mut self.pc, &self.registers, operation, rs1, rs2, imm);
            }
            Self::InstructionSet::UJType { operation, rd, imm } => {
                return {
                    execute_ujtype_instruction(
                        &mut self.pc,
                        &mut self.registers,
                        operation,
                        rd,
                        imm,
                    );
                    Ok(())
                };
            }
            Self::InstructionSet::UType { operation, rd, imm } => {
                execute_utype_instruction(self.pc, &mut self.registers, operation, rd, imm);
            }
        }
        self.pc += 4;
        Ok(())
    }
}

#[allow(clippy::too_many_arguments)]
fn execute_itype_instruction(
    debug: &mut bool,
    pc: &mut u32,
    output: &mut String,
    regs: &mut RegisterFile32Bit, // needs mutable access to the registers
    memory: &mut MemoryBus, // needs immutable access to the memory, except for the ReadString syscall which needs mutable access
    operation: ITypeOperation,
    rd: RegisterMapping,
    rs1: RegisterMapping,
    imm: i32,
) -> Result<()> {
    match operation {
        ITypeOperation::Addi => regs[rd] = regs[rs1].wrapping_add(imm as u32),
        ITypeOperation::Andi => regs[rd] = regs[rs1] & (imm as u32),
        ITypeOperation::Jalr => {
            let t = *pc + 4;
            *pc = regs[rs1].wrapping_add(imm as u32) & !1;
            if rd != RegisterMapping::Zero {
                regs[rd] = t;
            }
        }
        ITypeOperation::Lb => {
            regs[rd] = ((memory.read(regs[rs1].wrapping_add_signed(imm), Size::Byte)? as i32) << 24
                >> 24) as u32;
        }
        ITypeOperation::Lh => {
            regs[rd] = ((memory.read(regs[rs1].wrapping_add_signed(imm), Size::Half)? as i32) << 16
                >> 16) as u32;
        }
        ITypeOperation::Lw => {
            regs[rd] = memory.read(regs[rs1].wrapping_add_signed(imm), Size::Word)?;
        }
        ITypeOperation::Ori => regs[rd] = regs[rs1] | (imm as u32),
        ITypeOperation::Slli => regs[rd] = regs[rs1] << (imm & 0b11111),
        ITypeOperation::Slti => regs[rd] = u32::from((regs[rs1] as i32) < imm),
        ITypeOperation::Sltiu => regs[rd] = u32::from(regs[rs1] < (imm as u32)),
        ITypeOperation::Srai => regs[rd] = ((regs[rs1] as i32) >> (imm & 0b11111)) as u32,
        ITypeOperation::Srli => regs[rd] = regs[rs1] >> (imm & 0b11111),
        ITypeOperation::Xori => regs[rd] = regs[rs1] ^ (imm as u32),
        ITypeOperation::Lbu => {
            regs[rd] = memory.read(regs[rs1].wrapping_add_signed(imm), Size::Byte)?;
        }
        ITypeOperation::Lhu => {
            regs[rd] = memory.read(regs[rs1].wrapping_add_signed(imm), Size::Half)?;
        }
        ITypeOperation::Fence => unimplemented!("fence instruction not implemented"),
        ITypeOperation::FenceI => unimplemented!("fence.i instruction not implemented"),
        ITypeOperation::Ecall => process_ecall(regs, memory, output)?,
        ITypeOperation::Ebreak => *debug = true,
    }
    Ok(())
}

fn execute_rtype_instruction(
    regs: &mut RegisterFile32Bit,
    operation: RTypeOperation,
    rd: RegisterMapping,
    rs1: RegisterMapping,
    rs2: RegisterMapping,
) -> Result<()> {
    match operation {
        RTypeOperation::Add => regs[rd] = regs[rs1].wrapping_add(regs[rs2]),
        RTypeOperation::And => regs[rd] = regs[rs1] & regs[rs2],
        RTypeOperation::Or => regs[rd] = regs[rs1] | regs[rs2],
        RTypeOperation::Sll => regs[rd] = regs[rs1] << (regs[rs2] & 0b11111),
        RTypeOperation::Slt => {
            regs[rd] = u32::from((regs[rs1] as i32) < (regs[rs2] as i32));
        }
        RTypeOperation::Sltu => regs[rd] = u32::from(regs[rs1] < regs[rs2]),
        RTypeOperation::Sra => regs[rd] = ((regs[rs1] as i32) >> (regs[rs2] & 0b11111)) as u32,
        RTypeOperation::Srl => regs[rd] = regs[rs1] >> (regs[rs2] & 0b11111),
        RTypeOperation::Sub => regs[rd] = regs[rs1].wrapping_sub(regs[rs2]),
        RTypeOperation::Xor => regs[rd] = regs[rs1] ^ regs[rs2],
        RTypeOperation::Mul => regs[rd] = regs[rs1].wrapping_mul(regs[rs2]),
        // Multiply High
        RTypeOperation::Mulh => {
            regs[rd] = ((regs[rs1] as i32 as i64 * regs[rs2] as i32 as i64) as u64 >> 32) as u32;
        }
        RTypeOperation::Mulhu => regs[rd] = ((regs[rs1] as u64 * regs[rs2] as u64) >> 32) as u32,
        RTypeOperation::Mulhsu => {
            regs[rd] = ((regs[rs1] as i32 as i64 * regs[rs2] as i64) as u64 >> 32) as u32;
        }
        RTypeOperation::Div => {
            regs[rd] = (regs[rs1] as i32)
                .checked_div(regs[rs2] as i32)
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))? as u32;
        }
        RTypeOperation::Divu => {
            regs[rd] = regs[rs1]
                .checked_div(regs[rs2])
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))?;
        }
        RTypeOperation::Rem => {
            regs[rd] = (regs[rs1] as i32)
                .checked_rem(regs[rs2] as i32)
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))? as u32;
        }
        RTypeOperation::Remu => {
            regs[rd] = regs[rs1]
                .checked_rem(regs[rs2])
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))?;
        }
    }
    Ok(())
}

fn execute_stype_instruction(
    regs: &RegisterFile32Bit,
    memory: &mut MemoryBus,
    operation: STypeOperation,
    rs1: RegisterMapping,
    rs2: RegisterMapping,
    offset: i32,
) -> Result<()> {
    match operation {
        STypeOperation::Sb => {
            memory.write(regs[rs1].wrapping_add_signed(offset), regs[rs2], Size::Byte)
        }
        STypeOperation::Sh => {
            memory.write(regs[rs1].wrapping_add_signed(offset), regs[rs2], Size::Half)
        }
        STypeOperation::Sw => {
            memory.write(regs[rs1].wrapping_add_signed(offset), regs[rs2], Size::Word)
        }
    }
}

fn execute_sbtype_instruction(
    pc: &mut u32,
    regs: &RegisterFile32Bit,
    operation: SBTypeOperation,
    rs1: RegisterMapping,
    rs2: RegisterMapping,
    offset: i32,
) {
    match operation {
        SBTypeOperation::Beq => {
            if regs[rs1] == regs[rs2] {
                *pc = pc.wrapping_add_signed(offset - 4);
            }
        }
        SBTypeOperation::Bge => {
            if (regs[rs1] as i32) >= (regs[rs2] as i32) {
                *pc = pc.wrapping_add_signed(offset - 4);
            }
        }
        SBTypeOperation::Blt => {
            if (regs[rs1] as i32) < (regs[rs2] as i32) {
                *pc = pc.wrapping_add_signed(offset - 4);
            }
        }
        SBTypeOperation::Bne => {
            if regs[rs1] != regs[rs2] {
                *pc = pc.wrapping_add_signed(offset - 4);
            }
        }
        SBTypeOperation::Bltu => {
            if regs[rs1] < regs[rs2] {
                *pc = pc.wrapping_add_signed(offset - 4);
            }
        }
        SBTypeOperation::Bgeu => {
            if regs[rs1] >= regs[rs2] {
                *pc = pc.wrapping_add_signed(offset - 4);
            }
        }
    }
}

fn execute_ujtype_instruction(
    pc: &mut u32,
    regs: &mut RegisterFile32Bit,
    operation: UJTypeOperation,
    rd: RegisterMapping,
    offset: u32,
) {
    match operation {
        UJTypeOperation::Jal => {
            if rd != RegisterMapping::Zero {
                regs[rd] = *pc + 4;
            }
            *pc = pc.wrapping_add_signed(((offset as i32) << 12) >> 12);
        }
    }
}

fn execute_utype_instruction(
    pc: u32,
    registers: &mut RegisterFile32Bit,
    operation: UTypeOperation,
    rd: RegisterMapping,
    imm: u32,
) {
    match operation {
        UTypeOperation::Lui => registers[rd] = imm << 12,
        UTypeOperation::Auipc => registers[rd] = pc.wrapping_add(imm << 12),
    }
}

/// Processes Syscalls (ecall) made by the program being executed.
///
/// # Arguments
///
/// * `registers` - The CPU's register file.
///
/// # Register Usage
///
/// * `a7` - The syscall number.
/// * `a0` - The first argument to the syscall.
/// * `a1` - The second argument to the syscall.
/// * `a2` - The third argument to the syscall.
/// * `a3` - The fourth argument to the syscall.
///
/// # Register Updates
///
/// * `a0` - The return value of the syscall.
fn process_ecall(
    regs: &mut RegisterFile32Bit,
    memory: &mut MemoryBus,
    output: &mut String,
) -> Result<()> {
    match Syscall::from(regs[RegisterMapping::A7]) {
        Syscall::PrintInt => {
            let out = &regs[RegisterMapping::A0].to_string();
            output.push_str(&regs[RegisterMapping::A0].to_string());
            print!("{out}");
        }
        Syscall::PrintString => {
            let mut addr = regs[RegisterMapping::A0];
            loop {
                let byte = memory.read(addr, Size::Byte).map_err(|e| {
                    anyhow::anyhow!(
                        "Error reading string from memory at address{}: {}",
                        regs[RegisterMapping::A0],
                        e
                    )
                })?;
                if byte == 0 {
                    break;
                }
                let byte = (byte & 0xff) as u8 as char;
                output.push(byte);
                print!("{byte}");
                addr += 1;
            }
        }
        Syscall::ReadInt => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let value = input.trim().parse::<i32>()? as u32;
            regs[RegisterMapping::A0] = value;
        }
        Syscall::ReadString => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            let addr = regs[RegisterMapping::A0];
            let max_len = regs[RegisterMapping::A1] as usize;
            let mut i = 0;
            for byte in input.bytes() {
                if i >= max_len - 1 {
                    break;
                }
                memory.write(addr + i as u32, u32::from(byte), Size::Byte)?;
                i += 1;
            }
            // ensure the last byte is the null terminator
            memory.write(addr + i as u32, 0, Size::Byte)?;
        }
        Syscall::Exit => bail!("Program exited with code: 0"),
        Syscall::PrintChar => {
            let out = char::from((regs[RegisterMapping::A0] & 0xff) as u8);
            output.push(out);
            println!("{out}");
        }
        Syscall::ReadChar => {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            let value = input.trim().chars().next().unwrap() as u8;
            regs[RegisterMapping::A0] = u32::from(value);
        }
        Syscall::Time => {
            let time = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| anyhow::anyhow!("Error getting time: {}", e))?;
            regs[RegisterMapping::A0] = time.as_millis() as u32;
            regs[RegisterMapping::A1] = (time.as_millis() >> 32) as u32;
        }
        Syscall::Sleep => {
            let duration = std::time::Duration::from_millis(u64::from(regs[RegisterMapping::A0]));
            std::thread::sleep(duration);
        }
        Syscall::PrintIntHex => {
            let out = &format!("{:#x}", regs[RegisterMapping::A0]);
            output.push_str(out);
            print!("{out}");
        }
        Syscall::PrintIntBinary => {
            let out = &format!("{:#b}", regs[RegisterMapping::A0]);
            output.push_str(out);
            print!("{out}");
        }
        Syscall::PrintIntUnsigned => {
            let out = &format!("{}", regs[RegisterMapping::A0]);
            output.push_str(out);
            print!("{out}");
        }
        Syscall::Exit2 => bail!("Program exited with code: {}", regs[RegisterMapping::A0]),
        Syscall::UnSupported => bail!("Unsupported syscall number: {}", regs[RegisterMapping::A7]),
    }
    Ok(())
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Syscall {
    /// Print an integer to the console.
    /// # Inputs:
    /// a0 - the integer to print
    PrintInt = 1,
    // PrintFloat = 2,
    // PrintDouble = 3,
    /// Print a string to the console.
    /// # Inputs:
    /// a0 - the address of the null-terminated string to print
    PrintString = 4,
    /// Read an integer from the console.
    /// # Outputs:
    /// a0 - the integer read from the console
    ReadInt = 5,
    // ReadFloat = 6,
    // ReadDouble = 7,
    /// Read a string from the console.
    /// # Inputs:
    /// a0 - the address of the buffer to read the string into
    /// a1 - the maximum number of characters to read
    ReadString = 8,
    /// Exit the program with code 0
    Exit = 10,
    /// Print an ascii character to the console.
    /// # Inputs:
    /// a0 - the ascii character to print (only the lower 8 bits are used)
    PrintChar = 11,
    /// Read an ascii character from the console.
    /// # Outputs:
    /// a0 - the ascii character read from the console
    ReadChar = 12,
    /// get the current Unix time (milliseconds since 1 January 1970)
    /// # Outputs:
    /// a0 - lower order 32-bits of the time
    /// a1 - upper order 32-bits of the time
    Time = 30,
    /// Sleep for the given number of milliseconds
    /// # Inputs:
    /// a0 - the number of milliseconds to sleep
    Sleep = 32,
    /// Print an integer to the console in hexadecimal format.
    /// # Inputs:
    /// a0 - the integer to print
    PrintIntHex = 34,
    /// Print an integer to the console in binary format.
    /// # Inputs:
    /// a0 - the integer to print
    PrintIntBinary = 35,
    /// Print an integer to the console in unsigned format.
    /// # Inputs:
    /// a0 - the integer to print
    PrintIntUnsigned = 36,
    // RandSeed = 40,
    // RandInt = 41,
    // RandIntRange = 42,
    // RandFloat = 43,
    // RandDouble = 44,
    /// Exit the program with the given exit code
    /// # Inputs:
    /// a0 - the exit code
    Exit2 = 93,
    UnSupported,
}

impl From<u32> for Syscall {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::PrintInt,
            4 => Self::PrintString,
            5 => Self::ReadInt,
            8 => Self::ReadString,
            10 => Self::Exit,
            11 => Self::PrintChar,
            12 => Self::ReadChar,
            30 => Self::Time,
            32 => Self::Sleep,
            34 => Self::PrintIntHex,
            35 => Self::PrintIntBinary,
            36 => Self::PrintIntUnsigned,
            93 => Self::Exit2,
            _ => Self::UnSupported,
        }
    }
}
