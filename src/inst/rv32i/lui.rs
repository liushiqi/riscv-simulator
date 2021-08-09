use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Lui {
  pub(crate) imm: u64,
  pub(crate) rd: RegIndex,
}

impl Display for Lui {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "lui         {}, {1:#x}", self.rd, (self.imm as u32) >> 12)
  }
}

impl Instruction for Lui {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, self.imm);
    cpu.next_pc();
  }
}
