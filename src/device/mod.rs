use std::{
  fmt::{Display, Formatter},
  ops::Range,
  path::PathBuf,
};

pub use self::ram::Ram;
use crate::{device::uart::Uart, SimulateResult};

mod ram;
mod uart;

pub(crate) trait Device {
  fn read(&mut self, address: u64, req_type: ReqType) -> SimulateResult<ReqType>;

  fn write(&mut self, address: u64, data: ReqType) -> SimulateResult<()>;
}

#[derive(Debug)]
pub enum ReqType {
  Byte(u8),
  HalfWord(u16),
  Word(u32),
  DoubleWord(u64),
}

impl ReqType {
  pub(crate) fn fill_with(&self, data: u64) -> Self {
    match self {
      ReqType::Byte(_) => ReqType::Byte(data as u8),
      ReqType::HalfWord(_) => ReqType::HalfWord(data as u16),
      ReqType::Word(_) => ReqType::Word(data as u32),
      ReqType::DoubleWord(_) => ReqType::DoubleWord(data),
    }
  }

  pub(crate) fn get_inner(&self) -> u64 {
    match self {
      ReqType::Byte(data) => *data as u64,
      ReqType::HalfWord(data) => *data as u64,
      ReqType::Word(data) => *data as u64,
      ReqType::DoubleWord(data) => *data,
    }
  }
}

impl Display for ReqType {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      ReqType::Byte(_) => write!(f, "byte"),
      ReqType::HalfWord(_) => write!(f, "half word"),
      ReqType::Word(_) => write!(f, "word"),
      ReqType::DoubleWord(_) => write!(f, "double word"),
    }
  }
}

pub struct Bus {
  ram: (Ram, Range<u64>),
  uart: (Uart, Range<u64>),
}

impl Bus {
  pub fn new(ram: Ram, start: u64) -> Self {
    let range = start..start + ram.get_size() as u64;
    Self {
      ram: (ram, range),
      uart: (Uart::new(), 0x60000000..0x60001000),
    }
  }

  pub(crate) fn read(&mut self, address: u64, req_type: ReqType) -> SimulateResult<ReqType> {
    if self.ram.1.contains(&address) {
      self.ram.0.read(address - self.ram.1.start, req_type)
    } else if self.uart.1.contains(&address) {
      self.uart.0.read(address - self.uart.1.start, req_type)
    } else {
      panic!("Invalid address {:x} passed to Bus::read", address)
    }
  }

  pub(crate) fn write(&mut self, address: u64, data: ReqType) -> SimulateResult<()> {
    if self.ram.1.contains(&address) {
      self.ram.0.write(address - self.ram.1.start, data)
    } else if self.uart.1.contains(&address) {
      self.uart.0.write(address - self.uart.1.start, data)
    } else {
      panic!("Invalid address {:x} passed to Bus::write", address)
    }
  }
}

impl Bus {
  pub fn get_data(&self, address: u64) -> u32 { self.ram.0.get_data((address - self.ram.1.start) as usize) }

  pub(crate) fn dump_ram_to_file(&self, to_file: PathBuf) -> SimulateResult<()> { self.ram.0.dump_to_file(to_file) }

  pub(crate) fn dump_ram_range(&self, range: Range<usize>) -> SimulateResult<Vec<String>> {
    self
      .ram
      .0
      .dump_range(range.start - self.ram.1.start as usize..range.end - self.ram.1.start as usize)
  }

  pub(crate) fn dump_ram_range_to_file(&self, range: Range<usize>, to_file: PathBuf) -> SimulateResult<()> {
    self
      .ram
      .0
      .dump_range_to_file(range.start - self.ram.1.start as usize..range.end - self.ram.1.start as usize, to_file)
  }
}
