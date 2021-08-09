use std::fmt::{Display, Formatter};

use crate::{device::ReqType, inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Lh {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Lh {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "lh          {}, {}({})", self.rd, self.imm as i64, self.rs1)
  }
}

impl Instruction for Lh {
  fn exec(&self, cpu: &mut Cpu) {
    if let Ok(ReqType::HalfWord(result)) =
      cpu.read_addr(cpu.read_reg(self.rs1).wrapping_add(self.imm), ReqType::HalfWord(0))
    {
      cpu.write_register(self.rd, result as i16 as u64)
    } else {
      panic!("Memory load failed.")
    }
    cpu.next_pc();
  }
}
