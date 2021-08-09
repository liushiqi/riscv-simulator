use std::{
  fs::File,
  io::{BufReader, Read},
  ops::Range,
};

use object::{Architecture, Object, ObjectSection, ObjectSymbol, SectionKind};

use crate::{
  error::{SimulateError, SimulateResult},
  isa::XLen,
  Ram,
};

const VALID_KINDS: [SectionKind; 5] = [
  SectionKind::Text,
  SectionKind::Data,
  SectionKind::ReadOnlyData,
  SectionKind::ReadOnlyString,
  SectionKind::Tls,
];

pub struct ParseResult {
  pub ram: Ram,
  pub entry_point: u64,
  pub xlen: XLen,
  pub signature: Option<Range<usize>>,
}

pub fn parse_file(elf_file: File, memory_size: usize) -> SimulateResult<ParseResult> {
  let mut elf = vec![];
  let mut reader = BufReader::new(elf_file);
  reader.read_to_end(&mut elf)?;
  parse_binary(&elf, memory_size)
}

pub fn parse_binary(elf: &[u8], memory_size: usize) -> SimulateResult<ParseResult> {
  let elf_file = object::File::parse(elf)?;

  let data = extract_data(memory_size, &elf_file)?;
  let signature = find_signature(&elf_file);

  match elf_file.architecture() {
    Architecture::Riscv32 => Ok(ParseResult {
      ram: Ram::new(data),
      entry_point: elf_file.entry(),
      xlen: XLen::Bit32,
      signature,
    }),
    Architecture::Riscv64 => Ok(ParseResult {
      ram: Ram::new(data),
      entry_point: elf_file.entry(),
      xlen: XLen::Bit64,
      signature,
    }),
    arch => Err(SimulateError::ElfArchError(arch)),
  }
}

fn extract_data(memory_size: usize, obj_file: &object::File) -> SimulateResult<Vec<u8>> {
  let memory_size = memory_size << 20;
  let mut data = Vec::with_capacity(memory_size);
  data.resize(memory_size, 0x0);

  for section in obj_file.sections() {
    if VALID_KINDS.contains(&section.kind()) {
      if 0x8000_0000 + memory_size < (section.address() + section.size()) as usize {
        panic!("Memory size too small : address {} overflow.", section.address() + section.size())
      }
      unsafe {
        std::ptr::copy(
          section.data()?.as_ptr(),
          data[section.address() as usize - 0x80000000..].as_mut_ptr(),
          section.size() as usize,
        );
      }
    }
  }
  Ok(data)
}

fn find_signature(elf_file: &object::File) -> Option<Range<usize>> {
  let begin_signature = elf_file
    .symbols()
    .filter(|symbol| symbol.name() == Ok("begin_signature"))
    .collect::<Vec<_>>()
    .get(0)
    .map(|symbol| symbol.address() as usize);
  let end_signature = elf_file
    .symbols()
    .filter(|symbol| symbol.name() == Ok("end_signature"))
    .collect::<Vec<_>>()
    .get(0)
    .map(|symbol| symbol.address() as usize);

  let signature = if let (Some(begin_signature), Some(end_signature)) = (begin_signature, end_signature) {
    Some(begin_signature..end_signature)
  } else {
    None
  };
  signature
}
