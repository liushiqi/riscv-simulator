use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Sub {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Sub {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rs1 == RegIndex::zero() {
      write!(f, "neg         {}, {}", self.rd, self.rs2)
    } else {
      write!(f, "sub         {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
  }
}

impl Instruction for Sub {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.read_reg(self.rs1).wrapping_sub(cpu.read_reg(self.rs2)));
    cpu.next_pc();
  }
}
