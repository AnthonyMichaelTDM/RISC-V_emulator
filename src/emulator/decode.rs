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

//! Decoding of RISC-V instructions from 32-bit machine code
use anyhow::{bail, Result};

use crate::instruction_set_definition::{
    operations::{
        ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
        UTypeOperation,
    },
    Rv32imInstruction,
};

use super::cpu::registers::RegisterMapping;

#[allow(clippy::module_name_repetitions)]
pub trait Decode32BitInstruction {
    /// Decode a 32-bit machine code into an instruction
    ///
    /// # Arguments
    /// - `machine_code` - the 32-bit machine code
    ///
    /// # Returns
    /// - the decoded instruction
    ///
    /// # Errors
    /// - if the opcode is not recognized, or if the machinecode is malformed
    fn from_machine_code(machine_code: u32) -> Result<Self>
    where
        Self: Sized;
}

impl Decode32BitInstruction for Rv32imInstruction {
    #[allow(clippy::too_many_lines)]
    fn from_machine_code(machine_code: u32) -> Result<Self> {
        // extract the opcode
        let opcode: u32 = machine_code & 0b111_1111;

        // fields that are common to most instructions
        // (or at least are extracted the same way in all instructions the fields are present in)
        // we defer propogating any errors in reading the register mappings until we know the instruction uses them
        let rd = RegisterMapping::try_from(((machine_code >> 7) & 0b11111) as u8);
        let rs1 = RegisterMapping::try_from(((machine_code >> 15) & 0b11111) as u8);
        let rs2 = RegisterMapping::try_from(((machine_code >> 20) & 0b11111) as u8);
        let funct3: u8 = ((machine_code >> 12) & 0b111) as u8;

        match opcode {
            // R-type instructions
            0b011_0011 | 0b011_1011 => {
                // mask out the fields
                let funct7: u8 = ((machine_code >> 25) & 0b111_1111) as u8;

                // determine the operation based on the opcode, funct3, and funct7 fields
                let operation = match (opcode, funct3, funct7) {
                    // normal arithmetic instructions
                    (0b011_0011, 0b000, 0b000_0000) => RTypeOperation::Add,
                    (0b011_0011, 0b000, 0b010_0000) => RTypeOperation::Sub,
                    (0b011_0011, 0b001, 0b000_0000) => RTypeOperation::Sll,
                    (0b011_0011, 0b010, 0b000_0000) => RTypeOperation::Slt,
                    (0b011_0011, 0b011, 0b000_0000) => RTypeOperation::Sltu,
                    (0b011_0011, 0b100, 0b000_0000) => RTypeOperation::Xor,
                    (0b011_0011, 0b101, 0b000_0000) => RTypeOperation::Srl,
                    (0b011_0011, 0b101, 0b010_0000) => RTypeOperation::Sra,
                    (0b011_0011, 0b110, 0b000_0000) => RTypeOperation::Or,
                    (0b011_0011, 0b111, 0b000_0000) => RTypeOperation::And,
                    // M extension instructions
                    (0b011_0011, 0b000, 0b000_0001) => RTypeOperation::Mul,
                    (0b011_0011, 0b001, 0b000_0001) => RTypeOperation::Mulh,
                    (0b011_0011, 0b010, 0b000_0001) => RTypeOperation::Mulhsu,
                    (0b011_0011, 0b011, 0b000_0001) => RTypeOperation::Mulhu,
                    (0b011_0011, 0b100, 0b000_0001) => RTypeOperation::Div,
                    (0b011_0011, 0b101, 0b000_0001) => RTypeOperation::Divu,
                    (0b011_0011, 0b110, 0b000_0001) => RTypeOperation::Rem,
                    (0b011_0011, 0b111, 0b000_0001) => RTypeOperation::Remu,
                    _ => bail!("Unknown R-type instruction\n machine code: {machine_code:#010x}"),
                };

                Ok(Self::RType {
                    operation,
                    rd: rd?,
                    funct3,
                    rs1: rs1?,
                    rs2: rs2?,
                    funct7,
                })
            }
            // I-type instructions
            0b000_0011 | 0b000_1111 | 0b001_0011 | 0b001_1011 | 0b110_0111 | 0b111_0011 => {
                // convert to i32 so that our shift operations are sign extended, and we're explicity okay with the possible wrap
                #[allow(clippy::cast_possible_wrap)]
                let machine_code: i32 = machine_code as i32;
                let mut imm: i32 =
                    /* extract the lowest 12 bits of the immediate from the machine code */
                    (machine_code >> 20) & 0xFFF;

                let operation = match (opcode, funct3, imm) {
                    // memory load instructions
                    (0b000_0011, 0b000, _) => ITypeOperation::Lb,
                    (0b000_0011, 0b001, _) => ITypeOperation::Lh,
                    (0b000_0011, 0b010, _) => ITypeOperation::Lw,
                    (0b000_0011, 0b100, _) => ITypeOperation::Lbu,
                    (0b000_0011, 0b101, _) => ITypeOperation::Lhu,
                    // fence and fence.i instructions
                    (0b000_1111, 0b000, _) => ITypeOperation::Fence,
                    (0b000_1111, 0b001, _) => ITypeOperation::FenceI,
                    // I-type arithmetic instructions
                    (0b001_0011, 0b000, _) => ITypeOperation::Addi,
                    (0b001_0011, 0b111, _) => ITypeOperation::Andi,
                    (0b001_0011, 0b110, _) => ITypeOperation::Ori,
                    (0b001_0011, 0b001, immediate) if immediate >> 5 == 0b000_0000 => {
                        // only the lower 5 bits are used, these are the shift amount,
                        // they are also always unsigned so this type of mask is safe
                        imm &= 0b11111;
                        ITypeOperation::Slli
                    }
                    (0b001_0011, 0b101, immediate) if immediate >> 5 == 0b000_0000 => {
                        // only the lower 5 bits are used, these are the shift amount,
                        // they are also always unsigned so this type of mask is safe
                        imm &= 0b11111;
                        ITypeOperation::Srli
                    }
                    (0b001_0011, 0b101, immediate) if immediate >> 5 == 0b010_0000 => {
                        // only the lower 5 bits are used, these are the shift amount,
                        // they are also always unsigned so this type of mask is safe
                        imm &= 0b11111;
                        ITypeOperation::Srai
                    }
                    (0b001_0011, 0b010, _) => ITypeOperation::Slti,
                    (0b001_0011, 0b011, _) => ITypeOperation::Sltiu,
                    (0b001_0011, 0b100, _) => ITypeOperation::Xori,
                    // jalr instruction
                    (0b110_0111, 0b000, _) => ITypeOperation::Jalr,
                    // system instructions
                    (0b111_0011, 0b000, 0b0000_0000_0000) => ITypeOperation::Ecall,
                    (0b111_0011, 0b000, 0b0000_0000_0001) => ITypeOperation::Ebreak,
                    _ => bail!("Unknown I-type instruction\n machine code: {machine_code:#010x}"),
                };

                // if the instruction is not one of the unsigned instructions, sign extend the immediate
                if !matches!(operation, ITypeOperation::Sltiu) {
                    imm = imm << 20 >> 20;
                }

                Ok(Self::IType {
                    operation,
                    rd: rd?,
                    funct3,
                    rs1: rs1?,
                    imm,
                })
            }
            // S-type instructions
            0b010_0011 => {
                // convert to i32 so that our shift operations are sign extended, and we're explicity okay with the possible wrap
                #[allow(clippy::cast_possible_wrap)]
                let machine_code: i32 = machine_code as i32;
                // only the lower 12 bits of the immediate are given, so we need to sign extend it to 32 bits
                let imm: i32 =
                    /* extract the lowest 12 bits of the immediate from the machine code */
                     (((machine_code >> 7) & 0b11111) | ((machine_code >> 20) & 0b1111_1110_0000))
                    /* sign extend the immediate */
                    << 20 >> 20;

                let operation = match funct3 {
                    // memory store instructions
                    0b000 => STypeOperation::Sb,
                    0b001 => STypeOperation::Sh,
                    0b010 => STypeOperation::Sw,
                    _ => bail!("Unknown S-type instruction\n machine code: {machine_code:#010x}"),
                };

                Ok(Self::SType {
                    operation,
                    rs1: rs1?,
                    rs2: rs2?,
                    funct3,
                    imm,
                })
            }
            // SB-type instructions
            0b110_0011 => {
                // convert to i32 so that our shift operations are sign extended, and we're explicity okay with the possible wrap
                #[allow(clippy::cast_possible_wrap)]
                let machine_code: i32 = machine_code as i32;
                let imm: i32 =
                    /* extract the lowest 12 bits of the immediate from the machine code */
                    (machine_code >> 31) << 12// 12th bit
                    | ((machine_code << 4) & 0b1000_0000_0000)// 11th bit
                    | ((machine_code >> 20) & 0b111_1110_0000)// 10th:5th bits
                    | ((machine_code >> 7) & 0b11110) // 4th:1st bits, 0th bit is always 0
                    /* sign extend the immediate */
                    << 19 >> 19; // 19 because we know the last bit is 0 (and we want to keep it that way)

                let operation = match funct3 {
                    0b000 => SBTypeOperation::Beq,
                    0b001 => SBTypeOperation::Bne,
                    0b100 => SBTypeOperation::Blt,
                    0b101 => SBTypeOperation::Bge,
                    0b110 => SBTypeOperation::Bltu,
                    0b111 => SBTypeOperation::Bgeu,
                    _ => bail!("Unknown SB-type instruction\n machine code: {machine_code:#010x}"),
                };

                Ok(Self::SBType {
                    operation,
                    rs1: rs1?,
                    rs2: rs2?,
                    funct3,
                    imm,
                })
            }
            // UJ-type instructions
            0b110_1111 => {
                let imm: u32 = ((machine_code >> 11) & 0b1_0000_0000_0000_0000_0000) // 20th bit
                    | (machine_code & 0b1111_1111_0000_0000_0000)// 19th:12th bits
                    | ((machine_code >> 9) & 0b1000_0000_0000)// 11th bit
                    | ((machine_code >> 20) & 0b111_1111_1110); // 10th:1st bits, 0th bit is always 0

                Ok(Self::UJType {
                    operation: UJTypeOperation::Jal,
                    rd: rd?,
                    imm,
                })
            }
            // U-type instructions
            0b001_0111 | 0b011_0111 => {
                #[allow(clippy::cast_possible_wrap)]
                #[allow(clippy::cast_sign_loss)]
                let imm: u32 = (((machine_code & 0xFFFF_F000) as i32) >> 12) as u32;

                let operation = match opcode {
                    0b011_0111 => UTypeOperation::Lui,
                    0b001_0111 => UTypeOperation::Auipc,
                    _ => bail!("Unknown U-type instruction\n machine code: {machine_code:#010x}"),
                };

                Ok(Self::UType {
                    operation,
                    rd: rd?,
                    imm,
                })
            }
            // Unknown instruction
            _ => bail!(
                "Unknown OpCode: {:07b}\n machine code: {machine_code:#010x}",
                opcode
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use anyhow::Result;

    #[test]
    fn test_add() -> Result<()> {
        let machine_code: u32 = 0b0000_0000_0011_0010_0000_0010_1011_0011;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::RType {
                operation: RTypeOperation::Add,
                rs1: RegisterMapping::Tp,
                rs2: RegisterMapping::Gp,
                rd: RegisterMapping::T0,
                funct3: 0,
                funct7: 0,
            }
        );
        Ok(())
    }
    #[test]
    fn test_andi() -> Result<()> {
        let machine_code: u32 = 0b0000_0000_1010_0110_0111_0110_1001_0011;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::IType {
                operation: ITypeOperation::Andi,
                rs1: RegisterMapping::A2,
                rd: RegisterMapping::A3,
                funct3: 0b111,
                imm: 0xA, // 10
            }
        );
        Ok(())
    }
    #[test]
    fn test_sb() -> Result<()> {
        let machine_code: u32 = 0b1111_1110_0011_0010_0000_1000_0010_0011;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::SType {
                operation: STypeOperation::Sb,
                rs1: RegisterMapping::Tp,
                rs2: RegisterMapping::Gp,
                funct3: 0b000,
                imm: -16,
            }
        );
        Ok(())
    }
    #[test]
    fn test_bne() -> Result<()> {
        let machine_code: u32 = 0b0000_0001_1110_0010_1001_0011_0110_0011;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::SBType {
                operation: SBTypeOperation::Bne,
                rs1: RegisterMapping::T0,
                rs2: RegisterMapping::T5,
                funct3: 0b001,
                imm: 6,
            }
        );
        Ok(())
    }
    #[test]
    fn test_jal() -> Result<()> {
        let machine_code: u32 = 0b0000_0000_1010_0000_0000_0000_1110_1111;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::UJType {
                operation: UJTypeOperation::Jal,
                rd: RegisterMapping::Ra,
                imm: 0xA, // 10
            }
        );
        Ok(())
    }
    #[test]
    fn test_jal_2() -> Result<()> {
        let machine_code: u32 = 0b1000_0000_1011_0000_1000_0000_1110_1111;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::UJType {
                operation: UJTypeOperation::Jal,
                rd: RegisterMapping::Ra,
                imm: 0b1_0000_1000_1000_0000_1010,
            }
        );
        Ok(())
    }

    #[test]
    fn test_auipc() -> Result<()> {
        let machine_code: u32 = 0x0fc1_0497;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::UType {
                operation: UTypeOperation::Auipc,
                rd: RegisterMapping::S1,
                imm: 0xfc10,
            }
        );
        Ok(())
    }

    #[test]
    fn test_lui() -> Result<()> {
        let machine_code: u32 = 0x186a_0337;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::UType {
                operation: UTypeOperation::Lui,
                rd: RegisterMapping::T1,
                imm: 0x186a0,
            }
        );
        Ok(())
    }

    #[test]
    fn test_lbu_negative_offset() -> Result<()> {
        let machine_code: u32 = 0xff434483;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::IType {
                operation: ITypeOperation::Lbu,
                rs1: RegisterMapping::T1,
                rd: RegisterMapping::S1,
                funct3: 0b100,
                imm: -12,
            }
        );
        Ok(())
    }

    #[test]
    fn test_lbu_positive_offset() -> Result<()> {
        let machine_code: u32 = 0x00c34483;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::IType {
                operation: ITypeOperation::Lbu,
                rs1: RegisterMapping::T1,
                rd: RegisterMapping::S1,
                funct3: 0b100,
                imm: 12,
            }
        );
        Ok(())
    }

    #[test]
    fn test_lhu_negative_offset() -> Result<()> {
        let machine_code: u32 = 0xff435483;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::IType {
                operation: ITypeOperation::Lhu,
                rs1: RegisterMapping::T1,
                rd: RegisterMapping::S1,
                funct3: 0b101,
                imm: -12,
            }
        );
        Ok(())
    }

    #[test]
    fn test_lhu_positive_offset() -> Result<()> {
        let machine_code: u32 = 0x00c35483;
        let instruction = Rv32imInstruction::from_machine_code(machine_code)?;
        assert_eq!(
            instruction,
            Rv32imInstruction::IType {
                operation: ITypeOperation::Lhu,
                rs1: RegisterMapping::T1,
                rd: RegisterMapping::S1,
                funct3: 0b101,
                imm: 12,
            }
        );
        Ok(())
    }
}
