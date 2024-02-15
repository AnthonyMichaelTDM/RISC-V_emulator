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
    #[display(fmt = "addw")]
    Addw,
    #[display(fmt = "subw")]
    Subw,
    #[display(fmt = "sllw")]
    Sllw,
    #[display(fmt = "srlw")]
    Srlw,
    #[display(fmt = "sraw")]
    Sraw,
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
    #[display(fmt = "addiw")]
    Addiw,
    #[display(fmt = "slliw")]
    Slliw,
    #[display(fmt = "srliw")]
    Srliw,
    #[display(fmt = "sraiw")]
    Sraiw,
    #[display(fmt = "ecall")]
    Ecall,
    #[display(fmt = "ebreak")]
    Ebreak,
    #[display(fmt = "csrrw")]
    Csrrw,
    #[display(fmt = "csrrs")]
    Csrrs,
    #[display(fmt = "csrrc")]
    Csrrc,
    #[display(fmt = "csrrwi")]
    Csrrwi,
    #[display(fmt = "csrrsi")]
    Csrrsi,
    #[display(fmt = "csrrci")]
    Csrrci,
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
    #[display(fmt = "sd")]
    Sd,
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
