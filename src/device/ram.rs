use std::{
  convert::TryInto,
  fs::OpenOptions,
  io::{BufWriter, Write},
  ops::Range,
  path::PathBuf,
};

use crate::{
  device::{Device, ReqType},
  error::SimulateError,
  SimulateResult,
};

pub struct Ram {
  data: Vec<u8>,
}

impl Ram {
  pub fn new(data: Vec<u8>) -> Self { Self { data } }

  pub fn get_size(&self) -> usize { self.data.len() }
}

impl Device for Ram {
  fn read(&mut self, address: u64, req_type: ReqType) -> SimulateResult<ReqType> {
    let address = address as usize;
    match req_type {
      ReqType::Byte(_) => Ok(ReqType::Byte(self.data[address])),
      ReqType::HalfWord(_) => {
        if address & 0b1 != 0 {
          Err(SimulateError::LoadMisaligned(address + 0x80000000, req_type))
        } else {
          Ok(ReqType::HalfWord(u16::from_le_bytes(self.data[address..=address + 1].try_into()?)))
        }
      },
      ReqType::Word(_) => {
        if address & 0b11 != 0 {
          Err(SimulateError::LoadMisaligned(address + 0x80000000, req_type))
        } else {
          Ok(ReqType::Word(u32::from_le_bytes(self.data[address..=address + 3].try_into()?)))
        }
      },
      ReqType::DoubleWord(_) => {
        if address & 0b111 != 0 {
          Err(SimulateError::LoadMisaligned(address + 0x80000000, req_type))
        } else {
          Ok(ReqType::DoubleWord(u64::from_le_bytes(self.data[address..=address + 7].try_into()?)))
        }
      },
    }
  }

  fn write(&mut self, address: u64, data: ReqType) -> SimulateResult<()> {
    let address = address as usize;
    match data {
      ReqType::Byte(d) => Ok(self.data[address] = d),
      ReqType::HalfWord(d) => {
        if address & 0b1 != 0 {
          Err(SimulateError::StoreMisaligned(address + 0x80000000, data))
        } else {
          Ok(self.data[address..=address + 1].clone_from_slice(&u16::to_le_bytes(d)))
        }
      },
      ReqType::Word(d) => {
        if address & 0b11 != 0 {
          Err(SimulateError::StoreMisaligned(address + 0x80000000, data))
        } else {
          Ok(self.data[address..=address + 3].clone_from_slice(&u32::to_le_bytes(d)))
        }
      },
      ReqType::DoubleWord(d) => {
        if address & 0b111 != 0 {
          Err(SimulateError::StoreMisaligned(address + 0x80000000, data))
        } else {
          Ok(self.data[address..=address + 7].clone_from_slice(&u64::to_le_bytes(d)))
        }
      },
    }
  }
}

impl Ram {
  pub fn get_data(&self, address: usize) -> u32 {
    u32::from_le_bytes(self.data[address..=address + 3].try_into().unwrap())
  }

  pub fn dump_to_file(&self, to_file: PathBuf) -> SimulateResult<()> {
    let new_file = OpenOptions::new().create(true).truncate(true).write(true).open(to_file)?;
    let mut writer = BufWriter::new(new_file);
    for line in self.data.chunks(4) {
      writeln!(writer, "{:08x}", u32::from_le_bytes(line.try_into()?))?
    }
    Ok(())
  }

  pub fn dump_range(&self, range: Range<usize>) -> SimulateResult<Vec<String>> {
    self.data[range]
      .chunks(4)
      .map(|bytes| Ok(format!("{:08x}", u32::from_le_bytes(bytes.try_into()?))))
      .collect()
  }

  pub fn dump_range_to_file(&self, range: Range<usize>, to_file: PathBuf) -> SimulateResult<()> {
    let new_file = OpenOptions::new().create(true).truncate(true).write(true).open(to_file)?;
    let mut writer = BufWriter::new(new_file);
    for line in self.data[range].chunks(4) {
      writeln!(writer, "{:08x}", u32::from_le_bytes(line.try_into()?))?;
    }
    Ok(())
  }
}
