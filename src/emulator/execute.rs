use anyhow::Result;

use crate::instruction_set_definition::{
    operations::{
        ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
        UTypeOperation,
    },
    Ri32imInstruction,
};

use super::cpu::{
    memory::MemoryBus,
    registers::{RegisterFile32Bit, RegisterMapping},
    Cpu32Bit,
};

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
    type InstructionSet = Ri32imInstruction;

    fn execute(&mut self, instruction: Self::InstructionSet) -> Result<()> {
        match instruction {
            Self::InstructionSet::IType {
                operation,
                rd,
                funct3: _,
                rs1,
                imm,
            } => execute_itype_instruction(
                &mut self.registers,
                &self.memory,
                operation,
                rd,
                rs1,
                imm,
            ),
            Self::InstructionSet::RType {
                operation,
                rd,
                funct3: _,
                rs1,
                rs2,
                funct7: _,
            } => execute_rtype_instruction(&mut self.registers, operation, rd, rs1, rs2),
            Self::InstructionSet::SType {
                operation,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => execute_stype_instruction(
                &mut self.registers,
                &self.memory,
                operation,
                rs1,
                rs2,
                imm,
            ),
            Self::InstructionSet::SBType {
                operation,
                funct3: _,
                rs1,
                rs2,
                imm,
            } => execute_sbtype_instruction(
                &mut self.registers,
                &self.memory,
                operation,
                rs1,
                rs2,
                imm,
            ),
            Self::InstructionSet::UJType { operation, rd, imm } => {
                execute_ujtype_instruction(&mut self.registers, operation, rd, imm)
            }
            Self::InstructionSet::UType { operation, rd, imm } => {
                execute_utype_instruction(&mut self.registers, operation, rd, imm)
            }
        }?;
        self.pc += 4;
        Ok(())
    }
}

fn execute_itype_instruction(
    _registers: &mut RegisterFile32Bit, // needs mutable access to the registers
    _memory: &MemoryBus,                // needs immutable access to the memory
    _operation: ITypeOperation,
    _rd: RegisterMapping,
    _rs1: RegisterMapping,
    _imm: i32,
) -> Result<()> {
    todo!()
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
            regs[rd] = if (regs[rs1] as i32) < (regs[rs2] as i32) {
                1
            } else {
                0
            }
        }
        RTypeOperation::Sltu => regs[rd] = if regs[rs1] < regs[rs2] { 1 } else { 0 },
        RTypeOperation::Sra => regs[rd] = ((regs[rs1] as i32) >> (regs[rs2] & 0b11111)) as u32,
        RTypeOperation::Srl => regs[rd] = regs[rs1] >> (regs[rs2] & 0b11111),
        RTypeOperation::Sub => regs[rd] = regs[rs1].wrapping_sub(regs[rs2]),
        RTypeOperation::Xor => regs[rd] = regs[rs1] ^ regs[rs2],
        RTypeOperation::Mul => regs[rd] = regs[rs1].wrapping_mul(regs[rs2]),
        // Multiply High
        RTypeOperation::Mulh => {
            regs[rd] = ((regs[rs1] as i32 as i64 * regs[rs2] as i32 as i64) as u64 >> 32) as u32
        }
        RTypeOperation::Mulhu => regs[rd] = ((regs[rs1] as u64 * regs[rs2] as u64) >> 32) as u32,
        RTypeOperation::Mulhsu => {
            regs[rd] = ((regs[rs1] as i32 as i64 * regs[rs2] as i64) as u64 >> 32) as u32
        }
        RTypeOperation::Div => {
            regs[rd] = (regs[rs1] as i32)
                .checked_div(regs[rs2] as i32)
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))? as u32
        }
        RTypeOperation::Divu => {
            regs[rd] = regs[rs1]
                .checked_div(regs[rs2])
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))?
        }
        RTypeOperation::Rem => {
            regs[rd] = (regs[rs1] as i32)
                .checked_rem(regs[rs2] as i32)
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))? as u32
        }
        RTypeOperation::Remu => {
            regs[rd] = regs[rs1]
                .checked_rem(regs[rs2])
                .ok_or_else(|| anyhow::anyhow!("Division by zero"))?
        }
    }
    Ok(())
}

fn execute_stype_instruction(
    _registers: &mut RegisterFile32Bit,
    _memory: &MemoryBus,
    _operation: STypeOperation,
    _rs1: RegisterMapping,
    _rs2: RegisterMapping,
    _imm: i32,
) -> Result<()> {
    todo!()
}

fn execute_sbtype_instruction(
    _registers: &mut RegisterFile32Bit,
    _memory: &MemoryBus,
    _operation: SBTypeOperation,
    _rs1: RegisterMapping,
    _rs2: RegisterMapping,
    _imm: i32,
) -> Result<()> {
    todo!()
}

fn execute_ujtype_instruction(
    _registers: &mut RegisterFile32Bit,
    _operation: UJTypeOperation,
    _rd: RegisterMapping,
    _imm: u32,
) -> Result<()> {
    todo!()
}

fn execute_utype_instruction(
    _registers: &mut RegisterFile32Bit,
    _operation: UTypeOperation,
    _rd: RegisterMapping,
    _imm: u32,
) -> Result<()> {
    todo!()
}
