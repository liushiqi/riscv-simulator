use std::fmt::{Display, Formatter};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RegIndex(pub(crate) usize);

impl RegIndex {
  pub fn zero() -> Self { RegIndex(0) }

  pub fn new(index: usize) -> Self { RegIndex(index) }
}

impl Display for RegIndex {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}{}",
      match self.0 {
        0 => "zero",
        1 => "ra",
        2 => "sp",
        3 => "gp",
        4 => "tp",
        5..=7 => "t",
        8..=9 => "s",
        10..=17 => "a",
        18..=27 => "s",
        28..=31 => "t",
        _ => return Err(std::fmt::Error),
      },
      match self.0 {
        0..=4 => "".to_string(),
        5..=7 => (self.0 - 5).to_string(),
        8..=9 => (self.0 - 8).to_string(),
        10..=17 => (self.0 - 10).to_string(),
        18..=27 => (self.0 - 16).to_string(),
        28..=31 => (self.0 - 25).to_string(),
        _ => return Err(std::fmt::Error),
      }
    )
  }
}

#[derive(Default)]
pub struct RegFile<Data, const PRESERVE_ZERO: bool>
where Data: Copy + Clone {
  regs: [Data; 32],
}

impl<Data, const PRESERVE_ZERO: bool> RegFile<Data, PRESERVE_ZERO>
where Data: Copy + Clone
{
  pub fn read(&self, reg: RegIndex) -> Data { self.regs[reg.0] }

  pub fn write(&mut self, reg: RegIndex, data: Data) {
    if !PRESERVE_ZERO || reg.0 != 0 {
      self.regs[reg.0] = data
    }
  }
}
