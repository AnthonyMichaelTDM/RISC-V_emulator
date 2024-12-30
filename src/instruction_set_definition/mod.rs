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

use derive_more::Display;

use self::operations::{
    ITypeOperation, RTypeOperation, SBTypeOperation, STypeOperation, UJTypeOperation,
    UTypeOperation,
};
#[allow(unused_imports)]
use crate::emulator::cpu::registers::RegisterMapping;

pub mod operations;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum Rv32imInstruction {
    #[display(
        fmt = "{:10} {rd}, {rs1}, {rs2}        # R-Type:  operation, rd,  rs1, rs2",
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
        fmt = "{:10} {rd}, {rs1}, {imm:#010x} # I-Type:  operation, rd,  rs1, imm",
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
        fmt = "{:10} {rd},      {imm:#010x} # UJ-Type: operation, rd,  imm",
        "operation.to_string()"
    )]
    UJType {
        operation: UJTypeOperation,
        rd: RegisterMapping,
        imm: u32,
    },
    #[display(
        fmt = "{:10} {rd},      {imm:#010x} # U-Type:  operation, rd,  imm",
        "operation.to_string()"
    )]
    UType {
        operation: UTypeOperation,
        rd: RegisterMapping,
        imm: u32,
    },
}
