use std::fmt;

use anyhow::bail;

use super::{
    memory::{STACK_CEILING, TEXT_BASE},
    REGISTERS_COUNT,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd, Ord)]
#[repr(u8)]
pub enum RegisterMapping {
    Zero = 0,
    Ra = 1,
    Sp = 2,
    Gp = 3,
    Tp = 4,
    T0 = 5,
    T1 = 6,
    T2 = 7,
    S0 = 8,
    S1 = 9,
    A0 = 10,
    A1 = 11,
    A2 = 12,
    A3 = 13,
    A4 = 14,
    A5 = 15,
    A6 = 16,
    A7 = 17,
    S2 = 18,
    S3 = 19,
    S4 = 20,
    S5 = 21,
    S6 = 22,
    S7 = 23,
    S8 = 24,
    S9 = 25,
    S10 = 26,
    S11 = 27,
    T3 = 28,
    T4 = 29,
    T5 = 30,
    T6 = 31,
}

impl TryFrom<u8> for RegisterMapping {
    type Error = anyhow::Error;
    fn try_from(value: u8) -> Result<Self, anyhow::Error> {
        if value >= REGISTERS_COUNT {
            bail!(
                "Invalid register number provided to RegisterMapping::from(u8): {}",
                value
            );
        }
        // this is safe because:
        // 1. the value is checked to be within the range of the enum
        // 2. the enum is repr(u8), so the memory layout is the same as u8
        // 3. we explicityly define the src and dst generics to ensure that future changes to the enum's memory size are caught at compile time
        Ok(unsafe { std::mem::transmute::<u8, Self>(value) })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct RegisterFile32Bit {
    registers: [u32; REGISTERS_COUNT as usize],
}

impl RegisterFile32Bit {
    #[must_use]
    pub const fn new() -> Self {
        let mut registers = [0; REGISTERS_COUNT as usize];

        // set the stack pointer to the top of the stack (highest address in the stack region)
        registers[RegisterMapping::Sp as usize] = STACK_CEILING;
        // set the return address to the start of the text region, this will be overwritten by
        // structs using this register file (e.g. the CPU) upon loading a program
        registers[RegisterMapping::Ra as usize] = TEXT_BASE;

        Self { registers }
    }

    pub fn reset(&mut self) {
        for i in 0..REGISTERS_COUNT {
            self.registers[i as usize] = 0;
        }
        self.registers[RegisterMapping::Sp as usize] = STACK_CEILING;
        self.registers[RegisterMapping::Ra as usize] = TEXT_BASE;
    }

    #[must_use]
    pub const fn read(&self, reg: RegisterMapping) -> u32 {
        self.registers[reg as usize]
    }

    pub fn write(&mut self, reg: RegisterMapping, value: u32) {
        self.registers[reg as usize] = value;
    }
}

impl fmt::Display for RegisterFile32Bit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let abi = [
            "zero", " ra ", " sp ", " gp ", " tp ", " t0 ", " t1 ", " t2 ", " s0 ", " s1 ", " a0 ",
            " a1 ", " a2 ", " a3 ", " a4 ", " a5 ", " a6 ", " a7 ", " s2 ", " s3 ", " s4 ", " s5 ",
            " s6 ", " s7 ", " s8 ", " s9 ", " s10", " s11", " t3 ", " t4 ", " t5 ", " t6 ",
        ];
        let mut output = String::new();
        for i in (0..REGISTERS_COUNT).step_by(4) {
            output = format!(
                "{output}\nx{:02}({})={:#010x} x{:02}({})={:#010x} x{:02}({})={:#010x} x{:02}({})={:#010x}",
                i,
                abi[i as usize],
                self.read(RegisterMapping::try_from(i).expect("Invalid register number")),
                i + 1,
                abi[i as usize+ 1],
                self.read(RegisterMapping::try_from(i + 1).expect("Invalid register number")),
                i + 2,
                abi[i as usize + 2],
                self.read(RegisterMapping::try_from(i + 2).expect("Invalid register number")),
                i + 3,
                abi[i as usize + 3],
                self.read(RegisterMapping::try_from(i + 3).expect("Invalid register number")),
            );
        }
        write!(f, "{output}")
    }
}
