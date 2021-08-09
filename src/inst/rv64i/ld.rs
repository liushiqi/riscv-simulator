use std::fmt::{Display, Formatter};

use crate::{device::ReqType, inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Ld {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Ld {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "ld          {}, {}({})", self.rd, self.imm as i64, self.rs1)
  }
}

impl Instruction for Ld {
  fn exec(&self, cpu: &mut Cpu) {
    if let Ok(ReqType::DoubleWord(result)) =
      cpu.read_addr(cpu.read_reg(self.rs1).wrapping_add(self.imm), ReqType::DoubleWord(0))
    {
      cpu.write_register(self.rd, result);
    } else {
      panic!("Memory load failed.")
    }
    cpu.next_pc();
  }
}
