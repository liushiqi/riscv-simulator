#![feature(concat_idents)]

use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path,
  process::Command,
};

use riscv_simulator::{
  elf::{parse_file, ParseResult},
  Bus, Cpu, Isa,
};

fn compile_source(name: &str) -> ParseResult {
  let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
  let source_path = project_path.join(format!("tests/riscv-arch-test/rv32i_m/I/src/{}.S", name));
  let target_path = project_path.join(format!("tests/target/rv32/i/{}.elf", name));
  let objdump_path = project_path.join(format!("tests/target/rv32/i/{}.elf.objdump", name));
  let debug_objdump_path = project_path.join(format!("tests/target/rv32/i/{}.elf.debug", name));
  let include_path = project_path.join("tests/riscv-arch-test/include");
  let link_script_path = project_path.join("tests/riscv-arch-test/link.ld");

  std::fs::create_dir_all(target_path.parent().unwrap()).unwrap();

  let result = Command::new("riscv64-unknown-elf-gcc")
    .arg("-march=rv32i")
    .arg("-mabi=ilp32")
    .arg("-DXLEN=32")
    .arg("-g")
    .arg("-nostdlib")
    .arg("-nostartfiles")
    .arg(format!("-I{}", include_path.to_str().unwrap()))
    .arg(format!("-T{}", link_script_path.to_str().unwrap()))
    .arg(source_path)
    .arg("-o")
    .arg(target_path.clone())
    .output()
    .unwrap();
  assert!(
    result.status.success(),
    "Compile failed with error: \n{}",
    result.stderr.iter().map(|&c| c as char).collect::<String>()
  );
  let result = parse_file(File::open(&target_path).unwrap(), 0x10).unwrap();

  let objdump = Command::new("riscv64-unknown-elf-objdump").arg(target_path.clone()).arg("-D").output().unwrap();
  assert!(objdump.status.success());
  std::fs::write(objdump_path, objdump.stdout).unwrap();

  let debug = Command::new("riscv64-unknown-elf-objdump")
    .arg(target_path.clone())
    .arg("--source")
    .output()
    .unwrap();
  assert!(debug.status.success());
  std::fs::write(debug_objdump_path, debug.stdout).unwrap();

  result
}

fn read_reference(name: &str) -> std::io::Result<Vec<String>> {
  let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
  let reference_path =
    project_path.join(format!("tests/riscv-arch-test/rv32i_m/I/references/{}.reference_output", name));

  BufReader::new(File::open(reference_path).unwrap()).lines().collect()
}

macro_rules! generate_test_inst {
  ($($test_name:ident => $name:literal),*,) => {
    $(#[test]
    fn $test_name() {
      let parse_result = compile_source($name);
      let reference = read_reference($name).unwrap();
      let bus = Bus::new(parse_result.ram, 0x80000000);

      let mut cpu = Cpu::new(parse_result.entry_point, Isa::new("rv32i").unwrap(), bus, false);

      cpu.execute();

      assert_eq!(cpu.dump_ram_range(parse_result.signature.unwrap()).unwrap(), reference)
    })*
  };
}

generate_test_inst!(
  test_add => "add-01",
  test_addi => "addi-01",
  test_and => "and-01",
  test_andi => "andi-01",
  test_auipc => "auipc-01",
  test_beq => "beq-01",
  test_bge => "bge-01",
  test_bgeu => "bgeu-01",
  test_blt => "blt-01",
  test_bltu => "bltu-01",
  test_bne => "bne-01",
  test_fence => "fence-01",
  test_jal => "jal-01",
  test_jalr => "jalr-01",
  test_lb => "lb-align-01",
  test_lbu => "lbu-align-01",
  test_lh => "lh-align-01",
  test_lhu => "lhu-align-01",
  test_lui => "lui-01",
  test_lw => "lw-align-01",
  test_or => "or-01",
  test_ori => "ori-01",
  test_sb => "sb-align-01",
  test_sh => "sh-align-01",
  test_sll => "sll-01",
  test_slli => "slli-01",
  test_slt => "slt-01",
  test_slti => "slti-01",
  test_sltiu => "sltiu-01",
  test_sltu => "sltu-01",
  test_sra => "sra-01",
  test_srai => "srai-01",
  test_srl => "srl-01",
  test_srli => "srli-01",
  test_sub => "sub-01",
  test_sw => "sw-align-01",
  test_xor => "xor-01",
  test_xori => "xori-01",
);
