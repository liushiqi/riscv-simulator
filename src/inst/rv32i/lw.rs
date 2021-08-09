use std::fmt::{Display, Formatter};

use crate::{device::ReqType, inst::Instruction, reg_file::RegIndex, Cpu};

#[derive(Debug)]
pub(crate) struct Lw {
  pub(crate) imm: u64,
  pub(crate) rs1: RegIndex,
  pub(crate) rd: RegIndex,
}

impl Display for Lw {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "lw          {}, {}({})", self.rd, self.imm as i64, self.rs1)
  }
}

impl Instruction for Lw {
  fn exec(&self, cpu: &mut Cpu) {
    match cpu.read_addr(cpu.read_reg(self.rs1).wrapping_add(self.imm), ReqType::Word(0)) {
      Ok(ReqType::Word(result)) => cpu.write_register(self.rd, result as i32 as u64),
      Ok(_) => panic!("Load returns invalid type."),
      Err(error) => panic!("{}", error),
    }
    cpu.next_pc();
  }
}
