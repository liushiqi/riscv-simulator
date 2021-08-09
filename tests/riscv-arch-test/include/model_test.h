#ifndef _COMPLIANCE_MODEL_H
#define _COMPLIANCE_MODEL_H

#if XLEN == 64
#define ALIGNMENT 3
#else
#define ALIGNMENT 2
#endif

#define RVMODEL_DATA_SECTION                                                   \
.pushsection .tohost, "aw", @progbits;                                         \
  .align 8;                                                                    \
  .global tohost;                                                              \
tohost:                                                                        \
  .dword 0;                                                                    \
  .align 8;                                                                    \
  .global fromhost;                                                            \
fromhost:                                                                      \
  .dword 0;                                                                    \
.popsection;                                                                   \
  .align 8;                                                                    \
  .global begin_regstate;                                                      \
begin_regstate:                                                                \
  .word 128;                                                                   \
  .align 8;                                                                    \
  .global end_regstate;                                                        \
end_regstate:                                                                  \
  .word 4;

#define RVMODEL_HALT                                                           \
  addi x1, x1, 4;                                                              \
  li x1, 1;                                                                    \
write_tohost:                                                                  \
  sw x1, tohost, t5;                                                           \
self_loop:                                                                     \
  j self_loop;

#define RVMODEL_BOOT // No boot code is needed.

#define RVMODEL_DATA_BEGIN                                                     \
  .align 4;                                                                    \
  .global begin_signature;                                                     \
begin_signature:

#define RVMODEL_DATA_END                                                       \
  .align 4;                                                                    \
  .global end_signature;                                                       \
end_signature:                                                                 \
  RVMODEL_DATA_SECTION

#define RVMODEL_IO_WRITE_STR(STR)                                              \
.section .data.string;                                                         \
20001:                                                                         \
  .string STR;                                                                 \
.section .text;                                                                \
  la a0, 20001b;                                                               \
  jal __write_str

#if XLEN == 64
#define RSIZE 8
#define SAVE sd
#define LOAD ld
#else
#define RSIZE 4
#define SAVE sw
#define LOAD lw
#endif

#define IO_SAVE_REG_STATE(SP)                                                  \
  la SP, begin_regstate;                                                       \
  SAVE ra, (0*RSIZE)(SP);                                                      \
  SAVE sp, (1*RSIZE)(SP);                                                      \
  SAVE a0, (2*RSIZE)(SP);                                                      \
  SAVE t0, (3*RSIZE)(SP);                                                      \
  SAVE s0, (4*RSIZE)(SP);                                                      \
  SAVE s1, (5*RSIZE)(SP);                                                      \
  SAVE s2, (6*RSIZE)(SP);                                                      \
  SAVE s3, (7*RSIZE)(SP);                                                      \
  SAVE s4, (8*RSIZE)(SP);                                                      \
  SAVE s5, (9*RSIZE)(SP);                                                      \
  SAVE s7, (10*RSIZE)(SP);                                                     \
  SAVE s8, (11*RSIZE)(SP);                                                     \
  SAVE s9, (12*RSIZE)(SP);                                                     \
  SAVE s10, (13*RSIZE)(SP);                                                    \
  SAVE s11, (14*RSIZE)(SP)

#define IO_LOAD_REG_STATE(SP)                                                  \
  la SP, begin_regstate;                                                       \
  LOAD ra, (0*RSIZE)(SP);                                                      \
  LOAD sp, (1*RSIZE)(SP);                                                      \
  LOAD a0, (2*RSIZE)(SP);                                                      \
  LOAD t0, (3*RSIZE)(SP);                                                      \
  LOAD s0, (4*RSIZE)(SP);                                                      \
  LOAD s1, (5*RSIZE)(SP);                                                      \
  LOAD s2, (6*RSIZE)(SP);                                                      \
  LOAD s3, (7*RSIZE)(SP);                                                      \
  LOAD s4, (8*RSIZE)(SP);                                                      \
  LOAD s5, (9*RSIZE)(SP);                                                      \
  LOAD s7, (10*RSIZE)(SP);                                                     \
  LOAD s8, (11*RSIZE)(SP);                                                     \
  LOAD s9, (12*RSIZE)(SP);                                                     \
  LOAD s10, (13*RSIZE)(SP);                                                    \
  LOAD s11, (14*RSIZE)(SP)

.section .data;
  .align 8;
__put_char_temprary:
  .dword 0;
  .dword 0;
  .dword 0;
  .dword 0;
.section .text;
  .global __put_char;
__put_char:
  la t0, __put_char_temprary;
  SAVE ra, 0(t0);
  SAVE s0, 8(t0);
  SAVE s1, 16(t0);
  SAVE s2, 24(t0);
  li t0, 0x60000000;
test_loop_char:
  lw s0, 4(t0);
  andi s0, s0, 0x8;
  bnez s0, test_loop_char;
  sb a0, 0(t0);
  la t0, __put_char_temprary;
  LOAD ra, 0(t0);
  LOAD s0, 8(t0);
  LOAD s1, 16(t0);
  LOAD s2, 24(t0);
  ret;

.section .data;
  .align 8;
__put_hex_temprary:
  .dword 0;
  .dword 0;
.section .text;
  .global __put_hex;
__put_hex:
  la t0, __put_hex_temprary;
  SAVE ra, 0(t0);
  SAVE s0, 8(t0);
  li s0, 10;
  bge a0, s0, print_alpha;
  addi a0, a0, 48;
  jal __put_char;
  la t0, __put_hex_temprary;
  LOAD s0, 0(t0);
  ret;
print_alpha:
  addi a0, a0, 87;
  jal __put_char;
  la t0, __put_hex_temprary;
  LOAD ra, 0(t0);
  LOAD s0, 8(t0);
  ret;

.section .data;
  .align 8;
__write_str_temprary:
  .dword 0;
  .dword 0;
  .dword 0;
.section .text;
  .global __write_str;
__write_str:
  la t0, __write_str_temprary;
  SAVE ra, 0(t0);
  SAVE s0, 8(t0);
  SAVE s1, 16(t0);
  mv s0, a0;
loop_print_str:
  lb s1, 0(s0);
  beqz s1, loop_end_str;
  mv a0, s1;
  jal __put_char;
  addi s0, s0, 1;
  j loop_print_str
loop_end_str:
  la t0, __write_str_temprary;
  LOAD ra, 0(t0);
  LOAD s0, 8(t0);
  LOAD s1, 16(t0);
  ret;

.section .data;
  .align 8;
__write_num_temporary:
  .dword 0;
  .dword 0;
  .dword 0;
  .dword 0;
  .dword 0;
.section .text;
  .global __write_num;
__write_num:
  la t0, __write_num_temporary;
  SAVE ra, 0(t0);
  SAVE s0, 8(t0);
  SAVE s1, 16(t0);
  SAVE s2, 24(t0);
#if XLEN == 64
  addi s0, t0, 7;
#else
  addi s0, t0, 3;
#endif
  SAVE a0, 32(t0);
loop_print_num:
  lbu s1, 0(s0);
  srl s2, s1, 4;
  andi a0, s2, 0xf;
  jal __put_hex;
  andi a0, s1, 0xf;
  jal __put_hex;
  addi s0, s0, -1;
  bgez s0, loop_print_num;
  la t0, __write_num_temporary;
  LOAD ra, 0(t0);
  LOAD s0, 8(t0);
  LOAD s1, 16(t0);
  LOAD s2, 24(t0);
  ret;

#define RVMODEL_IO_ASSERT_GPR_EQ(SP, R, I)                                     \
  IO_SAVE_REG_STATE(SP);                                                       \
  mv s0, R;                                                                    \
  li s1, I;                                                                    \
  beq s0, s1, 20002f;                                                          \
  RVMODEL_IO_WRITE_STR("Test Failed ");                                        \
  RVMODEL_IO_WRITE_STR(": ");                                                  \
  RVMODEL_IO_WRITE_STR(#R);                                                    \
  RVMODEL_IO_WRITE_STR("( ");                                                  \
  mv a0, s0;                                                                   \
  jal __write_num;                                                             \
  RVMODEL_IO_WRITE_STR(" ) != ");                                              \
  mv a0, s1;                                                                   \
  jal __write_num;                                                             \
  j 20003f;                                                                    \
20002:                                                                         \
  RVMODEL_IO_WRITE_STR("Test Passed ");                                        \
20003:                                                                         \
  RVMODEL_IO_WRITE_STR("\n");                                                  \
  IO_LOAD_REG_STATE(SP);

#define RVMODEL_SET_MSW_INT

#define RVMODEL_CLEAR_MSW_INT

#define RVMODEL_CLEAR_MTIMER_INT

#define RVMODEL_CLEAR_MEXT_INT

#endif // _COMPLIANCE_MODEL_H