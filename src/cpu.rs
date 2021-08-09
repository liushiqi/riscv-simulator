use std::{
  io::{stderr, Write},
  ops::Range,
  path::PathBuf,
};

use crate::{
  device::{Bus, ReqType},
  inst::decode_instruction,
  isa::{Extension, Isa, XLen},
  reg_file::{RegFile, RegIndex},
  SimulateResult,
};

pub(crate) enum PcLength {
  Four,
  #[allow(unused)]
  Two,
}

pub struct Cpu {
  regs: RegFile<u64, false>,
  pc: u64,
  pc_len: PcLength,
  isa: Isa,
  bus: Bus,
  debug: bool,
}

impl Cpu {
  pub fn new(entry_point: u64, isa: Isa, bus: Bus, debug: bool) -> Self {
    Self {
      regs: RegFile::default(),
      pc: entry_point,
      pc_len: PcLength::Four,
      isa,
      bus,
      debug,
    }
  }

  pub fn execute(&mut self) {
    loop {
      let stderr = stderr();
      let prev_pc = self.pc;
      let instruction = self.read_addr(self.pc, ReqType::Word(0));
      if let Ok(ReqType::Word(instruction)) = instruction {
        let (inst, r#type) = decode_instruction(instruction, self);
        self.pc_len = r#type;
        if self.debug {
          write!(
            stderr.lock(),
            "0x{:016x} (0x{:08x}) {}\n",
            if self.get_xlen() == XLen::Bit32 { self.pc as u32 as u64 } else { self.pc },
            instruction,
            inst
          )
          .unwrap()
        }
        inst.exec(self);
        if self.pc == prev_pc {
          break
        }
      } else {
        panic!("Instruction load failed.")
      };
    }
  }
}

impl Cpu {
  pub(crate) fn get_xlen(&self) -> XLen { self.isa.get_xlen() }

  pub(crate) fn get_xlen_num(&self) -> u64 {
    match self.isa.get_xlen() {
      XLen::Bit32 => 32,
      XLen::Bit64 => 64,
    }
  }

  pub(crate) fn have_ext(&self, ext: Extension) -> bool { self.isa.have_ext(ext) }

  pub(crate) fn get_pc(&self) -> u64 { self.pc }

  pub(crate) fn get_next_pc(&self) -> u64 {
    self.pc +
      match self.pc_len {
        PcLength::Four => 4,
        PcLength::Two => 2,
      }
  }

  pub(crate) fn jump_to(&mut self, destination: u64) { self.pc = destination & !1u64 }

  pub(crate) fn next_pc(&mut self) {
    self.pc += match self.pc_len {
      PcLength::Four => 4,
      PcLength::Two => 2,
    }
  }

  pub(crate) fn read_reg(&self, reg: RegIndex) -> u64 { self.regs.read(reg) }

  pub(crate) fn read_reg_zext(&self, reg: RegIndex) -> u64 {
    if self.get_xlen() == XLen::Bit32 { self.regs.read(reg) as u32 as u64 } else { self.regs.read(reg) }
  }

  pub(crate) fn write_register(&mut self, reg: RegIndex, data: u64) {
    if reg.0 != 0 {
      self.regs.write(reg, if self.get_xlen() == XLen::Bit32 { data as i32 as u64 } else { data })
    }
  }

  pub(crate) fn read_addr(&mut self, address: u64, size: ReqType) -> SimulateResult<ReqType> {
    self.bus.read(if self.get_xlen() == XLen::Bit32 { address as u32 as u64 } else { address }, size)
  }

  pub(crate) fn write_addr(&mut self, address: u64, data: ReqType) -> SimulateResult<()> {
    self.bus.write(if self.get_xlen() == XLen::Bit32 { address as u32 as u64 } else { address }, data)
  }
}

impl Cpu {
  #[allow(unused)]
  pub fn get_data(&self, address: u64) -> u32 { self.bus.get_data(address) }

  #[allow(unused)]
  pub fn dump_ram_to_file(&self, to_file: PathBuf) -> SimulateResult<()> { self.bus.dump_ram_to_file(to_file) }

  #[allow(unused)]
  pub fn dump_ram_range(&self, range: Range<usize>) -> SimulateResult<String> { self.bus.dump_ram_range(range) }

  pub fn dump_ram_range_to_file(&self, range: Range<usize>, to_file: PathBuf) -> SimulateResult<()> {
    self.bus.dump_ram_range_to_file(range, to_file)
  }
}
