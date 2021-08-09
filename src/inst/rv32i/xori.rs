use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Xori {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Xori {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.imm == u64::MAX {
      write!(f, "not         {}, {}", self.rd, self.rs1)
    } else {
      write!(f, "xori        {}, {}, {} ({2:#x})", self.rd, self.rs1, self.imm as i32)
    }
  }
}

impl Instruction for Xori {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.read_reg(self.rs1) ^ self.imm);
    cpu.next_pc();
  }
}
