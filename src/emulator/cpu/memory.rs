use anyhow::{bail, Result};

use crate::emulator::cpu::Size;

// /// The base address of the text section.
// pub const TEXT_BASE: u32 = 0x0040_0000; // where the pc starts
/// The text section is 4MB in size.
pub const TEXT_SIZE: u32 = 0x0040_0000;
// /// The end address of the text section.
// pub const TEXT_END: u32 = TEXT_BASE + TEXT_SIZE - 4;

/// the data portion of the memory starts at `0x1000_0000` with static data (.data section)
/// and grows upwards to `0x1000_0000` + 4MB
/// the end of the data portion which is at `0x7FFF_FFFF`, and it is the start of the stack, wich grows downwards
/// the heap starts at the end of the data section and grows upwards
pub const STATIC_DATA_SIZE: u32 = 0x0040_0000;
pub const STACK_CEILING: u32 = 0x7FFF_EFFC;
pub const DRAM_END: u32 = 0x8000_0000;

struct MemoryRegion {
    base: u32,
    size: u32,
    data: Box<[u8]>,
}

impl MemoryRegion {
    pub fn new(base: u32, size: u32) -> Self {
        Self {
            base,
            size,
            data: vec![0; size as usize].into_boxed_slice(),
        }
    }

    /// Set the binary data to the memory.
    /// The data is copied to the memory region starting from the base address.
    pub fn initialize(&mut self, data: &[u8]) {
        assert!(
            data.len() <= self.size as usize,
            "Data is too large for the memory region"
        );
        self.data[..data.len()].copy_from_slice(data);
    }

    /// Load `size`-bit data from the memory.
    ///
    /// addr is the unadjusted address, the base address of the memory region is removed from it before reading.
    pub fn read(&self, addr: u32, size: Size) -> Result<u32> {
        if addr < self.base || addr > self.base + self.size {
            bail!("Address {:08x} is out of bounds", addr);
        }
        match size {
            Size::Byte => Ok(self.read8(addr)),
            Size::Half => Ok(self.read16(addr)),
            Size::Word => Ok(self.read32(addr)),
        }
    }

    /// Store `size`-bit data to the memory.
    ///
    /// addr is the unadjusted address, the base address of the memory region is removed from it before writing.
    pub fn write(&mut self, addr: u32, value: u32, size: Size) -> Result<()> {
        if addr < self.base || addr > self.base + self.size {
            bail!("Address {:08x} is out of bounds", addr);
        }
        match size {
            Size::Byte => self.write8(addr, value),
            Size::Half => self.write16(addr, value),
            Size::Word => self.write32(addr, value),
        }
        Ok(())
    }

    /// Write a byte to the memory.
    fn write8(&mut self, addr: u32, val: u32) {
        let index = (addr - self.base) as usize;
        self.data[index] = (val & 0xff) as u8;
    }

    /// Write 2 bytes to the memory with little endian.
    fn write16(&mut self, addr: u32, val: u32) {
        let index = (addr - self.base) as usize;
        self.data[index] = (val & 0xff) as u8;
        self.data[index + 1] = ((val >> 8) & 0xff) as u8;
    }

    /// Write 4 bytes to the memory with little endian.
    fn write32(&mut self, addr: u32, val: u32) {
        let index = (addr - self.base) as usize;
        self.data[index] = (val & 0xff) as u8;
        self.data[index + 1] = ((val >> 8) & 0xff) as u8;
        self.data[index + 2] = ((val >> 16) & 0xff) as u8;
        self.data[index + 3] = ((val >> 24) & 0xff) as u8;
    }

    /// Read a byte from the memory.
    const fn read8(&self, addr: u32) -> u32 {
        let index = (addr - self.base) as usize;
        self.data[index] as u32
    }

    /// Read 2 bytes from the memory with little endian.
    const fn read16(&self, addr: u32) -> u32 {
        let index = (addr - self.base) as usize;
        (self.data[index] as u32) | ((self.data[index + 1] as u32) << 8)
    }

    /// Read 4 bytes from the memory with little endian.
    const fn read32(&self, addr: u32) -> u32 {
        let index = (addr - self.base) as usize;
        (self.data[index] as u32)
            | ((self.data[index + 1] as u32) << 8)
            | ((self.data[index + 2] as u32) << 16)
            | ((self.data[index + 3] as u32) << 24)
    }
}

/// The system bus.
#[allow(clippy::module_name_repetitions)]
pub struct MemoryBus {
    dram: MemoryRegion,
    text: MemoryRegion,
}

impl MemoryBus {
    /// Create a new `MemoryBus` object.
    #[must_use]
    pub fn new(entrypoint: u32, code: &[u8], data: &[u8]) -> Self {
        let dram_start = entrypoint + code.len() as u32 + 0x1000;
        let mut dram = MemoryRegion::new(dram_start, DRAM_END - dram_start);
        dram.initialize(data);
        let mut text = MemoryRegion::new(entrypoint, code.len() as u32 + 4);
        text.initialize(code);

        Self { dram, text }
    }

    /// get the size of the text segment in bytes
    pub fn code_size(&self) -> u32 {
        self.text.size
    }
    /// get the entrypoint of the program
    pub fn entrypoint(&self) -> u32 {
        self.text.base
    }

    pub fn dram_start(&self) -> u32 {
        self.dram.base
    }

    pub fn dram_size(&self) -> u32 {
        self.dram.size
    }

    /// Load a `size`-bit data from the device that connects to the system bus.
    ///
    /// This method is used to read from the memory.
    ///
    /// # Errors
    ///
    /// This method will return an error if the address is out of bounds.
    pub fn read(&self, addr: u32, size: Size) -> Result<u32> {
        match addr {
            addr if addr >= self.entrypoint()
                && addr <= self.entrypoint() + self.code_size() as u32 =>
            {
                self.text.read(addr, size)
            }
            addr if addr >= self.dram_start() && addr <= DRAM_END => self.dram.read(addr, size),
            _ => bail!("Unkown or Out-Of-Bounds memory region addressed"),
        }
    }

    /// Store a `size`-bit data to the device that connects to the system bus.
    ///
    /// This method is used to write to the memory.
    ///
    /// # Errors
    ///
    /// This method will return an error if the address is out of bounds.
    /// or if the address is in the text section. (self modifying code is not supported)
    pub fn write(&mut self, addr: u32, value: u32, size: Size) -> Result<()> {
        match addr {
            addr if addr >= self.entrypoint()
                && addr <= self.entrypoint() + self.code_size() as u32 =>
            {
                bail!("Self modifying code is not supported")
            }
            addr if addr >= self.dram_start() && addr <= DRAM_END => {
                self.dram.write(addr, value, size)
            }
            _ => bail!("Unkown memory region addressed"),
        }
    }
}
