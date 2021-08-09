use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Beq {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
}

impl Display for Beq {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rs2 == RegIndex::zero() {
      write!(
        f,
        "beqz        {}, pc {} {} ({2:#x})",
        self.rs1,
        if (self.imm as i64) < 0 { "-" } else { "+" },
        (self.imm as i64).abs()
      )
    } else {
      write!(
        f,
        "beq         {}, {}, pc {} {} ({3:#x})",
        self.rs1,
        self.rs2,
        if (self.imm as i64) < 0 { "-" } else { "+" },
        (self.imm as i64).abs()
      )
    }
  }
}

impl Instruction for Beq {
  fn exec(&self, cpu: &mut Cpu) {
    if cpu.read_reg(self.rs1) == cpu.read_reg(self.rs2) {
      cpu.jump_to(cpu.get_pc().wrapping_add(self.imm));
    } else {
      cpu.next_pc();
    }
  }
}
