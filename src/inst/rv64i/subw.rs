use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Subw {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Subw {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "subw        {}, {}, {}", self.rd, self.rs1, self.rs2)
  }
}

impl Instruction for Subw {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, (cpu.read_reg(self.rs1) as i32).wrapping_sub(cpu.read_reg(self.rs2) as i32) as u64);
    cpu.next_pc();
  }
}
