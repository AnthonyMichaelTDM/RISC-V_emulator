use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Parser};
use elf::{endian::AnyEndian, ElfBytes};

#[derive(Debug, Parser)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION")
)]
struct Args {
    #[clap(short, long="input", help="The input binary", value_name="FILE", value_hint=clap::ValueHint::FilePath, required=true, index=1)]
    input_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.input_file;

    let file_data = std::fs::read(path)?;
    let file = ElfBytes::<AnyEndian>::minimal_parse(file_data.as_slice())?;
    let common = file.find_common_data()?;

    todo!();
}
