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

//! Definitions of the (supported) risc-v instructions    
use derive_more::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum RTypeOperation {
    #[display(fmt = "add")]
    Add,
    #[display(fmt = "and")]
    And,
    #[display(fmt = "or")]
    Or,
    #[display(fmt = "sll")]
    Sll,
    #[display(fmt = "slt")]
    Slt,
    #[display(fmt = "sltu")]
    Sltu,
    #[display(fmt = "sra")]
    Sra,
    #[display(fmt = "srl")]
    Srl,
    #[display(fmt = "sub")]
    Sub,
    #[display(fmt = "xor")]
    Xor,
    // below are not needed for this project, but included for completeness
    // below are the Multiply Extension instructions
    #[display(fmt = "mul")]
    Mul,
    #[display(fmt = "mulh")]
    Mulh,
    #[display(fmt = "mulhu")]
    Mulhu,
    #[display(fmt = "mulhsu")]
    Mulhsu,
    #[display(fmt = "div")]
    Div,
    #[display(fmt = "divu")]
    Divu,
    #[display(fmt = "rem")]
    Rem,
    #[display(fmt = "remu")]
    Remu,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum ITypeOperation {
    #[display(fmt = "addi")]
    Addi,
    #[display(fmt = "andi")]
    Andi,
    #[display(fmt = "jalr")]
    Jalr,
    #[display(fmt = "lb")]
    Lb,
    #[display(fmt = "lh")]
    Lh,
    #[display(fmt = "lw")]
    Lw,
    #[display(fmt = "ori")]
    Ori,
    #[display(fmt = "slli")]
    Slli,
    #[display(fmt = "slti")]
    Slti,
    #[display(fmt = "sltiu")]
    Sltiu,
    #[display(fmt = "srai")]
    Srai,
    #[display(fmt = "srli")]
    Srli,
    #[display(fmt = "xori")]
    Xori,
    // below are not needed for this project, but included for completeness
    #[display(fmt = "lbu")]
    Lbu,
    #[display(fmt = "lhu")]
    Lhu,
    #[display(fmt = "fence")]
    Fence,
    #[display(fmt = "fence.i")]
    FenceI,
    #[display(fmt = "ecall")]
    Ecall,
    #[display(fmt = "ebreak")]
    Ebreak,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum STypeOperation {
    #[display(fmt = "sb")]
    Sb,
    #[display(fmt = "sh")]
    Sh,
    #[display(fmt = "sw")]
    Sw,
    // below are not needed for this project, but included for completeness
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum SBTypeOperation {
    #[display(fmt = "beq")]
    Beq,
    #[display(fmt = "bge")]
    Bge,
    #[display(fmt = "blt")]
    Blt,
    #[display(fmt = "bne")]
    Bne,
    // below are not needed for this project, but included for completeness
    #[display(fmt = "bltu")]
    Bltu,
    #[display(fmt = "bgeu")]
    Bgeu,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum UJTypeOperation {
    #[display(fmt = "jal")]
    Jal,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Display)]
pub enum UTypeOperation {
    // below are not needed for this project, but included for completeness
    #[display(fmt = "lui")]
    Lui,
    #[display(fmt = "auipc")]
    Auipc,
}
