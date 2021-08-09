#![feature(box_syntax)]

use std::{path::PathBuf, process::exit};

use clap::{Clap, IntoApp};

use crate::{
  cpu::Cpu,
  device::{Bus, Ram},
  elf::ParseResult,
  error::{SimulateError, SimulateResult},
  isa::{Isa, XLen},
};

mod cpu;
mod device;
mod elf;
mod error;
mod inst;
mod isa;
mod reg_file;

/// RISC-V simulator in rust
#[derive(Clap, Debug)]
#[clap(name = "riscv-emulator")]
struct Options {
  /// RISC-V architecture elf file to simulate.
  binary: PathBuf,

  /// Print debug output
  #[clap(short, long)]
  debug: bool,

  /// ram size in MiB
  #[clap(long, default_value = "10")]
  ram_size: usize,

  /// dump the content in ram into file specified in <dump-ram>
  #[clap(long)]
  dump_ram: Option<PathBuf>,

  /// isa string to simulate on, should match the xlen in elf file.
  #[clap(long)]
  isa: String,

  /// dump the content between begin_signature and end_signature into file specified in <signature>
  #[clap(long)]
  signature: Option<PathBuf>,
}

fn run() -> SimulateResult<()> {
  let options: Options = Options::parse();
  let elf_file = std::fs::File::open(&options.binary)?;
  let isa = if let Some(isa) = Isa::new(&options.isa) {
    isa
  } else {
    println!("Invalid isa: {}", options.isa);
    Options::into_app().print_help()?;
    exit(1)
  };

  let ParseResult {
    ram,
    entry_point,
    signature,
    xlen,
  } = elf::parse_file(elf_file, options.ram_size)?;

  if isa.get_xlen() != xlen {
    Err(SimulateError::ElfArchError(match xlen {
      XLen::Bit32 => object::Architecture::Riscv32,
      XLen::Bit64 => object::Architecture::Riscv64,
    }))?
  }

  let mut cpu = Cpu::new(entry_point, isa, Bus::new(ram, 0x80000000), options.debug);
  cpu.execute();

  if let Some(dump_ram_path) = options.dump_ram {
    cpu.dump_ram_to_file(dump_ram_path)?;
  }
  if let (Some(signature_path), Some(signature)) = (options.signature, signature) {
    cpu.dump_ram_range_to_file(signature, signature_path)?;
  }
  Ok(())
}

fn main() {
  match run() {
    Ok(_) => {},
    Err(error) => {
      eprintln!("Simulate failed with error: {}", error)
    },
  }
}
