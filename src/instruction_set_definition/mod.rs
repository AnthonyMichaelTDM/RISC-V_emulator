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
        fmt = "{:10} {:<4} {:<4} {:<10} # R-Type:  operation, rd, rs1, rs2",
        "operation.to_string()",
        "format!(\"x{rd},\")",
        "format!(\"x{rs1},\")",
        "format!(\"x{rs2},\")"
    )]
    RType {
        operation: RTypeOperation,
        rd: u8,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        funct7: u8,
    },
    #[display(
        fmt = "{:10} {:<4} {:<4} {:#010x} # I-Type:  operation, rd, rs1, imm",
        "operation.to_string()",
        "format!(\"x{rd},\")",
        "format!(\"x{rs1},\")",
        imm
    )]
    IType {
        operation: ITypeOperation,
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm: i32,
    },
    #[display(
        fmt = "{:10} {:<4} {:<4} {:#010x} # S-Type:  operation, rs2, rs1, imm",
        "operation.to_string()",
        "format!(\"x{rs2},\")",
        "format!(\"x{rs1},\")",
        imm
    )]
    SType {
        operation: STypeOperation,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    #[display(
        fmt = "{:10} {:<4} {:<4} {:#010x} # SB-Type: operation, rs1, rs2, imm",
        "operation.to_string()",
        "format!(\"x{rs1},\")",
        "format!(\"x{rs2},\")",
        imm
    )]
    SBType {
        operation: SBTypeOperation,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    #[display(
        fmt = "{:10} {:<9} {:#010x} # UJ-Type: operation, rd, imm",
        "operation.to_string()",
        "format!(\"x{rd},\")",
        imm
    )]
    UJType {
        operation: UJTypeOperation,
        rd: u8,
        imm: u32,
    },
    #[display(
        fmt = "{:10} {:<4}      {:#010x} # U-Type:  operation, rd, imm",
        "operation.to_string()",
        "format!(\"x{rd},\")",
        imm
    )]
    UType {
        operation: UTypeOperation,
        rd: u8,
        imm: u32,
    },
}
