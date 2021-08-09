use std::fmt::{Display, Formatter};

use crate::{device::ReqType, inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Sb {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rs2: RegIndex,
}

impl Display for Sb {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "sb          {}, {}({})", self.rs2, self.imm as i64, self.rs1)
  }
}

impl Instruction for Sb {
  fn exec(&self, cpu: &mut Cpu) {
    if cpu
      .write_addr(cpu.read_reg(self.rs1).wrapping_add(self.imm), ReqType::Byte(cpu.read_reg(self.rs2) as u8))
      .is_err()
    {
      panic!("Memory load failed.")
    }
    cpu.next_pc();
  }
}
