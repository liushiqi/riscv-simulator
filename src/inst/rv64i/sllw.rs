use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Sllw {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Sllw {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "sllw        {}, {}, {}", self.rd, self.rs1, self.rs2)
  }
}

impl Instruction for Sllw {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, ((cpu.read_reg(self.rs1) as u32) << (cpu.read_reg(self.rs2) & 0b1_1111)) as i32 as u64);
    cpu.next_pc();
  }
}
