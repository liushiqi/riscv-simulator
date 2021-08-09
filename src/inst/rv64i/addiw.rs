use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Addiw {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Addiw {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "addiw       {}, {}, {} ({2:#x})", self.rd, self.rs1, self.imm as i32)
  }
}

impl Instruction for Addiw {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, (cpu.read_reg(self.rs1) as i32).wrapping_add(self.imm as i32) as u64);
    cpu.next_pc();
  }
}
