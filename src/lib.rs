#![feature(box_syntax)]

pub use crate::{
  cpu::Cpu,
  device::{Bus, Ram},
  error::{SimulateError, SimulateResult},
  isa::{Isa, XLen},
};

mod cpu;
mod device;
pub mod elf;
mod error;
mod inst;
mod isa;
mod reg_file;
