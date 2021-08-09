use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Or {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Or {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "or, {}, {}, {}", self.rd, self.rs1, self.rs2) }
}

impl Instruction for Or {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.read_reg(self.rs1) | cpu.read_reg(self.rs2));
    cpu.next_pc();
  }
}
