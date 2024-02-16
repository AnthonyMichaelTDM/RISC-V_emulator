pub mod emulator;
pub mod instruction_set_definition;
pub mod utils;

use std::path::PathBuf;

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

    let mut cpu: Cpu32Bit = Cpu32Bit::new();
    cpu.load(text_section, data_section.unwrap_or_default(), entrypoint);

    loop {
        if args.debug {
            // clear the screen
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            // print cpu state
            println!("CPU state before fetching the next instruction:");
            println!("{}", cpu);
        }

        if let Err(e) = cpu.step() {
            eprintln!("Error: {}", e);
            break;
        }
    }

    Ok(())
}
