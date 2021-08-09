use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Sltu {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Sltu {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rs1 == RegIndex::zero() {
      write!(f, "snez        {}, {}", self.rd, self.rs2)
    } else {
      write!(f, "sltu        {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
  }
}

impl Instruction for Sltu {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, (cpu.read_reg(self.rs1) < cpu.read_reg(self.rs2)) as u64);
    cpu.next_pc();
  }
}
