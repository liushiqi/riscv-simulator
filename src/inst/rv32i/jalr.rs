use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Jalr {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Jalr {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rd == RegIndex::zero() && self.imm == 0 && self.rs1 == RegIndex::new(1) {
      write!(f, "ret")
    } else if self.rd == RegIndex::zero() && self.imm == 0 {
      write!(f, "jr          {}", self.rs1)
    } else if self.rd == RegIndex::new(1) && self.imm == 0 {
      write!(f, "jalr        {}", self.rs1)
    } else {
      write!(f, "jalr        {}, {}({})", self.rd, self.imm as i64, self.rs1)
    }
  }
}

impl Instruction for Jalr {
  fn exec(&self, cpu: &mut Cpu) {
    let next_pc = cpu.get_next_pc();
    cpu.jump_to(cpu.read_reg(self.rs1).wrapping_add(self.imm));
    cpu.write_register(self.rd, next_pc);
  }
}
