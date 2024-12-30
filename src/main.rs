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

pub mod emulator;
pub mod instruction_set_definition;
pub mod utils;

#[allow(unused_imports)]
use std::{path::PathBuf, str::FromStr as _};

use anyhow::{bail, Result};
use clap::{command, Parser};
use elf::{endian::AnyEndian, ElfBytes};
use emulator::cpu::Cpu32Bit;

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    #[clap( help="The input binary", value_name="FILE", value_hint=clap::ValueHint::FilePath, required=true, index=1)]
    input_file: PathBuf,
    #[clap(short, long, help = "Enable debug mode")]
    debug: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.input_file;
    let debug = args.debug;

    // let path = PathBuf::from_str("test_binaries/matrix_mult.bin")?;
    // let debug = true;

    let file_data = std::fs::read(path)?;
    let file = ElfBytes::<AnyEndian>::minimal_parse(file_data.as_slice())?;

    let data_header = file.section_header_by_name(".data")?;
    let (data_section, _data_compression_header) = if let Some(header) = data_header {
        let (a, b) = file.section_data(&header)?;
        (Some(a), b)
    } else {
        (None, None)
    };

    let entrypoint = u32::try_from(file.ehdr.e_entry)?; // the entrypoint should fit in a u32, if it doesn't, the file is invalid

    let text_header = file.section_header_by_name(".text")?;
    let (text_section, _text_compression_header) = if let Some(header) = text_header {
        let (a, b) = file.section_data(&header)?;
        (a, b)
    } else {
        bail!("No .text section found")
    };

    assert!(
        text_section.len() % 4 == 0,
        "Text section length is not a multiple of 4, this is not a valid RISC-V binary"
    );

    // extract `__global_pointer$` from the ELF file, it's a symbol not a section
    #[allow(clippy::cast_possible_truncation)]
    let gp = file.symbol_table()?.and_then(|table| {
        table
            .0
            .iter()
            .find(|symbol| table.1.get(symbol.st_name as usize).unwrap() == "__global_pointer$")
            .map(|symbol| symbol.st_value as u32)
    });

    let mut cpu: Cpu32Bit = Cpu32Bit::new(
        text_section,
        data_section.unwrap_or_default(),
        entrypoint,
        gp,
    );

    if debug {
        // pause before executing the first instruction
        cpu.debug = true;
    }

    loop {
        if let Err(e) = cpu.step() {
            eprintln!("Error: {e}");
            break;
        }
    }

    Ok(())
}
