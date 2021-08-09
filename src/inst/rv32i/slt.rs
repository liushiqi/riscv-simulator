use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Slt {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Slt {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rs1 == RegIndex::zero() {
      write!(f, "sgtz        {}, {}", self.rd, self.rs2)
    } else if self.rs2 == RegIndex::zero() {
      write!(f, "sltz        {}, {}", self.rd, self.rs1)
    } else {
      write!(f, "slt         {}, {}, {}", self.rd, self.rs1, self.rs2)
    }
  }
}

impl Instruction for Slt {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, ((cpu.read_reg(self.rs1) as i64) < (cpu.read_reg(self.rs2) as i64)) as u64);
    cpu.next_pc();
  }
}
