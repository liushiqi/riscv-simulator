use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Fence {
  #[allow(unused)]
  pub(crate) imm: u64,
  #[allow(unused)]
  pub(crate) rs1: RegIndex,
  #[allow(unused)]
  pub(crate) rd: RegIndex,
}

impl Display for Fence {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.imm == 0b1000_0011_0011 {
      write!(f, "fence.tso")
    } else if self.imm == 0b1111_1111 {
      write!(f, "fence")
    } else {
      let pred = if self.imm & 0b1000_0000 != 0 { "i" } else { "" }.to_string() +
        if self.imm & 0b0100_0000 != 0 { "o" } else { "" } +
        if self.imm & 0b0010_0000 != 0 { "r" } else { "" } +
        if self.imm & 0b0001_0000 != 0 { "w" } else { "" };
      let succ = if self.imm & 0b1000 != 0 { "i" } else { "" }.to_string() +
        if self.imm & 0b0100 != 0 { "o" } else { "" } +
        if self.imm & 0b0010 != 0 { "r" } else { "" } +
        if self.imm & 0b0001 != 0 { "w" } else { "" };
      write!(f, "fence       {}, {}", pred, succ)
    }
  }
}

impl Instruction for Fence {
  fn exec(&self, cpu: &mut Cpu) {
    if self.imm & 0b1111_0000_0000 != 0b0000_0000_0000 ||
      (self.imm & 0b1111_0000_0000 == 0b1000_0000_0000 && self.imm != 0b1000_0011_0011)
    {
      todo!("Invalid instruction exception in fence")
    }
    cpu.next_pc();
  }
}
