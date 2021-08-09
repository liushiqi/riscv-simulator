use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Slti {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Slti {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "slti        {}, {}, {} ({2:#x})", self.rd, self.rs1, self.imm as i32)
  }
}

impl Instruction for Slti {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, ((cpu.read_reg(self.rs1) as i64) < (self.imm as i64)) as u64);
    cpu.next_pc();
  }
}
