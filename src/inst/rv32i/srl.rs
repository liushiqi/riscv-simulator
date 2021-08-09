use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Srl {
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Srl {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "srl         {}, {}, {}", self.rd, self.rs1, self.rs2)
  }
}

impl Instruction for Srl {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(
      self.rd,
      cpu.read_reg_zext(self.rs1) >> (cpu.read_reg(self.rs2) & (cpu.get_xlen_num() as u64 - 1)),
    );
    cpu.next_pc();
  }
}
