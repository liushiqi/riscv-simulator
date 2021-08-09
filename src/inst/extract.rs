use crate::reg_file::RegIndex;

const OPTYPE_MASK: u32 = 0b11;
const OPCODE_MASK: u32 = 0b111_1111;
const FUNCT3_MASK: u32 = 0b111;
const FUNCT7_MASK: u32 = 0b111_1111;

const REG_ADDR_MASK: u32 = 0b1_1111;

const IMM_I_MASK: i32 = !0;

const IMM_S_MASK1: i32 = !0b0000_0001_1111;
const IMM_S_MASK2: i32 = 0b0000_0001_1111;

const IMM_B_MASK1: i32 = !0b0_1111_1111_1111;
const IMM_B_MASK2: i32 = 0b0_1000_0000_0000;
const IMM_B_MASK3: i32 = 0b0_0111_1110_0000;
const IMM_B_MASK4: i32 = 0b0_0000_0001_1110;

const IMM_U_MASK: u32 = 0b1111_1111_1111_1111_1111_0000_0000_0000;

const IMM_J_MASK1: i32 = !0b0_1111_1111_1111_1111_1111;
const IMM_J_MASK2: i32 = 0b0_1111_1111_0000_0000_0000;
const IMM_J_MASK3: i32 = 0b0_0000_0000_1000_0000_0000;
const IMM_J_MASK4: i32 = 0b0_0000_0000_0111_1111_1110;

#[inline]
pub(super) fn extract_optype(instruction: u32) -> u32 { instruction & OPTYPE_MASK }

#[inline]
pub(super) fn extract_opcode(instruction: u32) -> u32 { instruction & OPCODE_MASK }

#[inline]
pub(super) fn extract_funct3(instruction: u32) -> u32 { (instruction >> 12) & FUNCT3_MASK }

#[inline]
pub(super) fn extract_funct7(instruction: u32) -> u32 { (instruction >> 25) & FUNCT7_MASK }

#[inline]
pub(super) fn extract_rs1(instruction: u32) -> RegIndex { RegIndex(((instruction >> 15) & REG_ADDR_MASK) as usize) }

#[inline]
pub(super) fn extract_rs2(instruction: u32) -> RegIndex { RegIndex(((instruction >> 20) & REG_ADDR_MASK) as usize) }

#[inline]
pub(super) fn extract_rd(instruction: u32) -> RegIndex { RegIndex(((instruction >> 7) & REG_ADDR_MASK) as usize) }

#[inline]
pub(super) fn extract_imm_i(instruction: u32) -> u64 {
  let instruction = instruction as i32;
  ((instruction >> 20) & IMM_I_MASK) as i64 as u64
}

#[inline]
pub(super) fn extract_imm_s(instruction: u32) -> u64 {
  let instruction = instruction as i32;
  ((instruction >> 20) & IMM_S_MASK1 | (instruction >> 7) & IMM_S_MASK2) as i64 as u64
}

#[inline]
pub(super) fn extract_imm_b(instruction: u32) -> u64 {
  let instruction = instruction as i32;
  ((instruction >> 19) & IMM_B_MASK1 |
    (instruction << 4) & IMM_B_MASK2 |
    (instruction >> 20) & IMM_B_MASK3 |
    (instruction >> 7) & IMM_B_MASK4) as i64 as u64
}

#[inline]
pub(super) fn extract_imm_u(instruction: u32) -> u64 {
  let instruction = instruction as i32;
  (instruction & IMM_U_MASK as i32) as i64 as u64
}

#[inline]
pub(super) fn extract_imm_j(instruction: u32) -> u64 {
  let instruction = instruction as i32;
  ((instruction >> 11) & IMM_J_MASK1 |
    instruction & IMM_J_MASK2 |
    (instruction >> 9) & IMM_J_MASK3 |
    (instruction >> 20) & IMM_J_MASK4) as i64 as u64
}
