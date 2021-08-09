use std::fmt::{Debug, Display};

use crate::{
  cpu::PcLength,
  inst::extract::{extract_funct3, extract_funct7, extract_opcode, extract_optype},
  Cpu,
};

mod extract;
mod rv32i;
mod rv64i;

pub trait Instruction: Debug + Display {
  fn exec(&self, cpu: &mut Cpu);
}

macro_rules! inst_r {
  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident) => {{
    use self::extract::{extract_rd, extract_rs1, extract_rs2};
    use crate::isa::Extension;
    if $cpu.have_ext(Extension::$ext) {
      box self::$mod::$type {
        rs1: extract_rs1($inst),
        rs2: extract_rs2($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};

  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident, $_:ident) => {{
    use self::extract::{extract_rd, extract_rs1, extract_rs2};
    use crate::isa::{Extension, XLen};
    if $cpu.have_ext(Extension::$ext) && $cpu.get_xlen() == XLen::Bit64 {
      box self::$mod::$type {
        rs1: extract_rs1($inst),
        rs2: extract_rs2($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};
}

macro_rules! inst_i {
  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident) => {{
    use self::extract::{extract_imm_i, extract_rd, extract_rs1};
    use crate::isa::Extension;
    if $cpu.have_ext(Extension::$ext) {
      box self::$mod::$type {
        imm: extract_imm_i($inst),
        rs1: extract_rs1($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};

  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident, $_:ident) => {{
    use self::extract::{extract_imm_i, extract_rd, extract_rs1};
    use crate::isa::{Extension, XLen};
    if $cpu.have_ext(Extension::$ext) && $cpu.get_xlen() == XLen::Bit64 {
      box self::$mod::$type {
        imm: extract_imm_i($inst),
        rs1: extract_rs1($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};
}

macro_rules! inst_s {
  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident) => {{
    use self::extract::{extract_imm_s, extract_rs1, extract_rs2};
    use crate::isa::Extension;
    if $cpu.have_ext(Extension::$ext) {
      box self::$mod::$type {
        imm: extract_imm_s($inst),
        rs1: extract_rs1($inst),
        rs2: extract_rs2($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};

  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident, $_:ident) => {{
    use self::extract::{extract_imm_s, extract_rs1, extract_rs2};
    use crate::isa::{Extension, XLen};
    if $cpu.have_ext(Extension::$ext) && $cpu.get_xlen() == XLen::Bit64 {
      box self::$mod::$type {
        imm: extract_imm_s($inst),
        rs1: extract_rs1($inst),
        rs2: extract_rs2($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};
}

macro_rules! inst_b {
  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident) => {{
    use self::extract::{extract_imm_b, extract_rs1, extract_rs2};
    use crate::isa::Extension;
    if $cpu.have_ext(Extension::$ext) {
      box self::$mod::$type {
        imm: extract_imm_b($inst),
        rs1: extract_rs1($inst),
        rs2: extract_rs2($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};

  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident, $_:ident) => {{
    use self::extract::{extract_imm_b, extract_rs1, extract_rs2};
    use crate::isa::{Extension, XLen};
    if $cpu.have_ext(Extension::$ext) && $cpu.get_xlen() == XLen::Bit64 {
      box self::$mod::$type {
        imm: extract_imm_b($inst),
        rs1: extract_rs1($inst),
        rs2: extract_rs2($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};
}

macro_rules! inst_u {
  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident) => {{
    use self::extract::{extract_imm_u, extract_rd};
    use crate::isa::Extension;
    if $cpu.have_ext(Extension::$ext) {
      box self::$mod::$type {
        imm: extract_imm_u($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};

  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident, $_:ident) => {{
    use self::extract::{extract_imm_u, extract_rd};
    use crate::isa::{Extension, XLen};
    if $cpu.have_ext(Extension::$ext) && $cpu.get_xlen() == XLen::Bit64 {
      box self::$mod::$type {
        imm: extract_imm_u($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};
}

macro_rules! inst_j {
  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident) => {{
    use self::extract::{extract_imm_j, extract_rd};
    use crate::isa::Extension;
    if $cpu.have_ext(Extension::$ext) {
      box self::$mod::$type {
        imm: extract_imm_j($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};

  ($mod:ident, $type:ident, $ext:ident, $cpu:ident, $inst:ident, $_:ident) => {{
    use self::extract::{extract_imm_j, extract_rd};
    use crate::isa::{Extension, XLen};
    if $cpu.have_ext(Extension::$ext) && $cpu.get_xlen() == XLen::Bit64 {
      box self::$mod::$type {
        imm: extract_imm_j($inst),
        rd: extract_rd($inst),
      }
    } else {
      todo!("Invalid instruction exception on inst {:032b}", $inst);
    }
  }};
}

const LUI_OPCODE: u32 = 0b0110111;
const AUIPC_OPCODE: u32 = 0b0010111;
const JAL_OPCODE: u32 = 0b1101111;
const JALR_OPCODE: u32 = 0b1100111;
const BRANCH_OPCODE: u32 = 0b1100011;
const LOAD_OPCODE: u32 = 0b0000011;
const STORE_OPCODE: u32 = 0b0100011;
const OP_IMM_OPCODE: u32 = 0b0010011;
const OP_IMM32_OPCODE: u32 = 0b0011011;
const OP_OPCODE: u32 = 0b0110011;
const OP32_OPCODE: u32 = 0b0111011;
const MISC_MEM_OPCODE: u32 = 0b0001111;

pub(crate) fn decode_instruction(instruction: u32, cpu: &Cpu) -> (Box<dyn Instruction>, PcLength) {
  match extract_optype(instruction) {
    0b11 => (
      match extract_opcode(instruction) {
        LUI_OPCODE => inst_u!(rv32i, Lui, I, cpu, instruction),
        AUIPC_OPCODE => inst_u!(rv32i, Auipc, I, cpu, instruction),
        JAL_OPCODE => inst_j!(rv32i, Jal, I, cpu, instruction),
        JALR_OPCODE => {
          if extract_funct3(instruction) == 0b000 {
            inst_i!(rv32i, Jalr, I, cpu, instruction)
          } else {
            todo!("Invalid instruction exception on inst {:032b}", instruction);
          }
        },
        BRANCH_OPCODE => match extract_funct3(instruction) {
          0b000 => inst_b!(rv32i, Beq, I, cpu, instruction),
          0b001 => inst_b!(rv32i, Bne, I, cpu, instruction),
          0b100 => inst_b!(rv32i, Blt, I, cpu, instruction),
          0b101 => inst_b!(rv32i, Bge, I, cpu, instruction),
          0b110 => inst_b!(rv32i, Bltu, I, cpu, instruction),
          0b111 => inst_b!(rv32i, Bgeu, I, cpu, instruction),
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        LOAD_OPCODE => match extract_funct3(instruction) {
          0b000 => inst_i!(rv32i, Lb, I, cpu, instruction),
          0b001 => inst_i!(rv32i, Lh, I, cpu, instruction),
          0b010 => inst_i!(rv32i, Lw, I, cpu, instruction),
          0b011 => inst_i!(rv64i, Ld, I, cpu, instruction, true),
          0b100 => inst_i!(rv32i, Lbu, I, cpu, instruction),
          0b101 => inst_i!(rv32i, Lhu, I, cpu, instruction),
          0b110 => inst_i!(rv64i, Lwu, I, cpu, instruction, true),
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        STORE_OPCODE => match extract_funct3(instruction) {
          0b000 => inst_s!(rv32i, Sb, I, cpu, instruction),
          0b001 => inst_s!(rv32i, Sh, I, cpu, instruction),
          0b010 => inst_s!(rv32i, Sw, I, cpu, instruction),
          0b011 => inst_s!(rv64i, Sd, I, cpu, instruction, true),
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        OP_IMM_OPCODE => match extract_funct3(instruction) {
          0b000 => inst_i!(rv32i, Addi, I, cpu, instruction),
          0b010 => inst_i!(rv32i, Slti, I, cpu, instruction),
          0b011 => inst_i!(rv32i, Sltiu, I, cpu, instruction),
          0b100 => inst_i!(rv32i, Xori, I, cpu, instruction),
          0b110 => inst_i!(rv32i, Ori, I, cpu, instruction),
          0b111 => inst_i!(rv32i, Andi, I, cpu, instruction),
          0b001 => {
            if extract_funct7(instruction) & !0b1 == 0 {
              inst_i!(rv32i, Slli, I, cpu, instruction)
            } else {
              todo!("Invalid instruction exception on inst {:032b}", instruction)
            }
          },
          0b101 => match extract_funct7(instruction) {
            0b000_0000 => inst_i!(rv32i, Srli, I, cpu, instruction),
            0b010_0000 => inst_i!(rv32i, Srai, I, cpu, instruction),
            0b000_0001 => inst_i!(rv32i, Srli, I, cpu, instruction, true),
            0b010_0001 => inst_i!(rv32i, Srai, I, cpu, instruction, true),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        OP_IMM32_OPCODE => match extract_funct3(instruction) {
          0b000 => inst_i!(rv64i, Addiw, I, cpu, instruction, true),
          0b001 => inst_i!(rv64i, Slliw, I, cpu, instruction, true),
          0b101 => match extract_funct7(instruction) {
            0b000_0000 => inst_i!(rv64i, Srliw, I, cpu, instruction, true),
            0b010_0000 => inst_i!(rv64i, Sraiw, I, cpu, instruction, true),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        OP_OPCODE => match extract_funct3(instruction) {
          0b000 => match extract_funct7(instruction) {
            0b000_0000 => inst_r!(rv32i, Add, I, cpu, instruction),
            0b010_0000 => inst_r!(rv32i, Sub, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b001 => match extract_funct7(instruction) {
            0b0000000 => inst_r!(rv32i, Sll, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b010 => match extract_funct7(instruction) {
            0b0000000 => inst_r!(rv32i, Slt, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b011 => match extract_funct7(instruction) {
            0b0000000 => inst_r!(rv32i, Sltu, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b100 => match extract_funct7(instruction) {
            0b0000000 => inst_r!(rv32i, Xor, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b101 => match extract_funct7(instruction) {
            0b0000000 => inst_r!(rv32i, Srl, I, cpu, instruction),
            0b0100000 => inst_r!(rv32i, Sra, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b110 => match extract_funct7(instruction) {
            0b0000000 => inst_r!(rv32i, Or, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b111 => {
            if extract_funct7(instruction) != 0 {
              todo!("Invalid instruction exception on inst {:032b}", instruction)
            } else {
              inst_r!(rv32i, And, I, cpu, instruction)
            }
          },
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        OP32_OPCODE => match extract_funct3(instruction) {
          0b000 => match extract_funct7(instruction) {
            0b000_0000 => inst_r!(rv64i, Addw, I, cpu, instruction),
            0b010_0000 => inst_r!(rv64i, Subw, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          0b001 => {
            if extract_funct7(instruction) != 0 {
              todo!("Invalid instruction exception on inst {:032b}", instruction)
            } else {
              inst_r!(rv64i, Sllw, I, cpu, instruction, instruction)
            }
          },
          0b101 => match extract_funct7(instruction) {
            0b000_0000 => inst_r!(rv64i, Srlw, I, cpu, instruction),
            0b010_0000 => inst_r!(rv64i, Sraw, I, cpu, instruction),
            _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
          },
          _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
        },
        MISC_MEM_OPCODE => {
          if extract_funct3(instruction) != 0 {
            todo!("Invalid instruction exception on inst {:032b}", instruction)
          } else {
            inst_i!(rv32i, Fence, I, cpu, instruction)
          }
        },
        _ => todo!("Invalid instruction exception on inst {:032b}", instruction),
      },
      PcLength::Four,
    ),
    _ => todo!("Compress instructions"),
  }
}
