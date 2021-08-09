use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Addi {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Addi {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rs1.0 == 0 && self.rd.0 == 0 && self.imm == 0 {
      write!(f, "nop")
    } else if self.imm == 0 {
      write!(f, "mv          {}, {}", self.rd, self.rs1)
    } else {
      write!(f, "addi        {}, {}, {} ({2:#x})", self.rd, self.rs1, self.imm as i32)
    }
  }
}

impl Instruction for Addi {
  fn exec(&self, cpu: &mut Cpu) {
    cpu.write_register(self.rd, cpu.read_reg(self.rs1).wrapping_add(self.imm));
    cpu.next_pc();
  }
}
