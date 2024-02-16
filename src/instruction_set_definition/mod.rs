use derive_more::Display;

use self::operations::{
    ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
    UTypeOperation,
};
#[allow(unused_imports)]
use crate::emulator::cpu::registers::RegisterMapping;

pub mod operations;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum Ri32imInstruction {
    #[display(
        fmt = "{:10} {rd}, {rs1}, {rs2}        # R-Type:  operation, rd, rs1, rs2",
        "operation.to_string()"
    )]
    RType {
        operation: RTypeOperation,
        rd: RegisterMapping,
        funct3: u8,
        rs1: RegisterMapping,
        rs2: RegisterMapping,
        funct7: u8,
    },
    #[display(
        fmt = "{:10} {rd}, {rs1}, {imm:#010x} # I-Type:  operation, rd, rs1, imm",
        "operation.to_string()"
    )]
    IType {
        operation: ITypeOperation,
        rd: RegisterMapping,
        funct3: u8,
        rs1: RegisterMapping,
        imm: i32,
    },
    #[display(
        fmt = "{:10} {rs2}, {rs1}, {imm:#010x} # S-Type:  operation, rs2, rs1, imm",
        "operation.to_string()"
    )]
    SType {
        operation: STypeOperation,
        funct3: u8,
        rs1: RegisterMapping,
        rs2: RegisterMapping,
        imm: i32,
    },
    #[display(
        fmt = "{:10} {rs1}, {rs2}, {imm:#010x} # SB-Type: operation, rs1, rs2, imm",
        "operation.to_string()"
    )]
    SBType {
        operation: SBTypeOperation,
        funct3: u8,
        rs1: RegisterMapping,
        rs2: RegisterMapping,
        imm: i32,
    },
    #[display(
        fmt = "{:10} {rd},      {imm:#010x} # UJ-Type: operation, rd, imm",
        "operation.to_string()"
    )]
    UJType {
        operation: UJTypeOperation,
        rd: RegisterMapping,
        imm: u32,
    },
    #[display(
        fmt = "{:10} {rd},      {imm:#010x} # U-Type:  operation, rd, imm",
        "operation.to_string()"
    )]
    UType {
        operation: UTypeOperation,
        rd: RegisterMapping,
        imm: u32,
    },
}
