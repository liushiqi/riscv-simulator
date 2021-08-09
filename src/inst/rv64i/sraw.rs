use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Sraw {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Sraw {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "sraw        {}, {}, {}", self.rd, self.rs1, self.rs2)
  }
}

impl Instruction for Sraw {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, ((cpu.read_reg(self.rs1) as i32) >> (cpu.read_reg(self.rs2) & 0b1_1111)) as u64);
    cpu.next_pc();
  }
}
