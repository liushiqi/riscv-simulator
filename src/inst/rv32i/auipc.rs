use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Auipc {
  pub(crate) imm: u64,
  pub(crate) rd: RegIndex,
}

impl Display for Auipc {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "auipc       {}, {} ({1:#x})", self.rd, self.imm as i32 >> 12)
  }
}

impl Instruction for Auipc {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.get_pc().wrapping_add(self.imm));
    cpu.next_pc();
  }
}
