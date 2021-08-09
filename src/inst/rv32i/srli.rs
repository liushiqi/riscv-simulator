use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Srli {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Srli {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "srli        {}, {}, {} ({2:#x})", self.rd, self.rs1, self.imm as i32)
  }
}

impl Instruction for Srli {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.read_reg_zext(self.rs1) >> (self.imm & (cpu.get_xlen_num() as u64 - 1)));
    cpu.next_pc();
  }
}
