use derive_more::Display;

use self::operations::{
    ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
    UTypeOperation,
};

pub mod operations;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum Instruction {
    #[display(
        fmt = "Instruction Type: R, Operation: {operation}, Rs1: x{rs1}, Rs2: x{rs2}, Rd: x{rd}, Funct3: {funct3}, Funct7: {funct7}"
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
        fmt = "Instruction Type: I, Operation: {operation}, Rs1: x{rs1}, Rd: x{rd}, Immediate: {imm}"
    )]
    IType {
        operation: ITypeOperation,
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm: i32,
    },
    #[display(
        fmt = "Instruction Type: S, Operation: {operation}, Rs1: x{rs1}, Rs2: x{rs2}, Immediate: {imm}"
    )]
    SType {
        operation: STypeOperation,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    #[display(
        fmt = "Instruction Type: SB, Operation: {operation}, Rs1: x{rs1}, Rs2: x{rs2}, Immediate: {imm}"
    )]
    SBType {
        operation: SBTypeOperation,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    #[display(fmt = "Instruction Type: UJ, Operation: {operation}, Rd: x{rd}, Immediate: {imm}")]
    UJType {
        operation: UJTypeOperation,
        rd: u8,
        imm: u32,
    },
    #[display(fmt = "Instruction Type: U, Operation: {operation}, Rd: x{rd}, Immediate: {imm}")]
    UType {
        operation: UTypeOperation,
        rd: u8,
        imm: u32,
    },
}
