use derive_more::Display;

use self::operations::{
    ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
    UTypeOperation,
};

pub mod operations;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum Ri32imInstruction {
    #[display(
        fmt = "Operation: {operation}, Rs1: x{rs1}, Rs2: x{rs2}, Rd: x{rd}, Funct3: {funct3}, Funct7: {funct7}, Instruction Type: R"
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
        fmt = "Operation: {operation}, Rs1: x{rs1}, Rd: x{rd}, Immediate: {imm}, Instruction Type: I"
    )]
    IType {
        operation: ITypeOperation,
        rd: u8,
        funct3: u8,
        rs1: u8,
        imm: i32,
    },
    #[display(
        fmt = "Operation: {operation}, Rs1: x{rs1}, Rs2: x{rs2}, Immediate: {imm}, Instruction Type: S"
    )]
    SType {
        operation: STypeOperation,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    #[display(
        fmt = "Operation: {operation}, Rs1: x{rs1}, Rs2: x{rs2}, Immediate: {imm}, Instruction Type: SB"
    )]
    SBType {
        operation: SBTypeOperation,
        funct3: u8,
        rs1: u8,
        rs2: u8,
        imm: i32,
    },
    #[display(fmt = "Operation: {operation}, Rd: x{rd}, Immediate: {imm}, Instruction Type: UJ")]
    UJType {
        operation: UJTypeOperation,
        rd: u8,
        imm: u32,
    },
    #[display(fmt = "Operation: {operation}, Rd: x{rd}, Immediate: {imm}, Instruction Type: U")]
    UType {
        operation: UTypeOperation,
        rd: u8,
        imm: u32,
    },
}
