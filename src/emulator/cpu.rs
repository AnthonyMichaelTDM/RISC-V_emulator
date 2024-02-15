pub struct Cpu32Bit {
    pub registers: [u32; 32],
    pub pc: u32,
    pub code: Vec<u8>,
    pub memory: Vec<u8>,
}
