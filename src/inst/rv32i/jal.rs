use std::fmt::{Display, Formatter};

use crate::{inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Jal {
  pub(crate) imm: u64,
  pub(crate) rd: RegIndex,
}

impl Display for Jal {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    if self.rd == RegIndex::zero() {
      write!(
        f,
        "j           pc {} {} ({1:#x})",
        if (self.imm as i64) < 0 { "-" } else { "+" },
        (self.imm as i64).abs()
      )
    } else if self.rd == RegIndex::new(1) {
      write!(
        f,
        "jal         pc {} {} ({1:#x})",
        if (self.imm as i64) < 0 { "-" } else { "+" },
        (self.imm as i64).abs()
      )
    } else {
      write!(
        f,
        "jal         {}, pc {} {} ({2:#x})",
        self.rd,
        if (self.imm as i64) < 0 { "-" } else { "+" },
        (self.imm as i64).abs()
      )
    }
  }
}

impl Instruction for Jal {
  fn exec(&self, cpu: &mut Cpu) {
    let next_pc = cpu.get_next_pc();
    cpu.jump_to(cpu.get_pc().wrapping_add(self.imm));
    cpu.write_register(self.rd, next_pc);
  }
}
