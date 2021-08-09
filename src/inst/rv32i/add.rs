use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Add {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Add {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "add         {}, {}, {}", self.rd, self.rs1, self.rs2)
  }
}

impl Instruction for Add {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.read_reg(self.rs1).wrapping_add(cpu.read_reg(self.rs2)));
    cpu.next_pc();
  }
}
