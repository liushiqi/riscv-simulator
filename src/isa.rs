use std::{collections::HashMap, option::Option::Some};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum XLen {
  Bit32,
  Bit64,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub(crate) enum Extension {
  I,
  M,
  A,
  F,
  D,
  Q,
  C,
  B,
  K,
  H,
  P,
  V,
  Zicsr,
  Zifencei,
}

pub struct Isa {
  xlen: XLen,
  extensions: Vec<Extension>,
}

const ALL_EXT: &str = "imafdqcbkhpv";

fn get_ext_map() -> HashMap<char, Extension> {
  let mut ext_map = HashMap::new();
  ext_map.insert('i', Extension::I);
  ext_map.insert('m', Extension::M);
  ext_map.insert('a', Extension::A);
  ext_map.insert('f', Extension::F);
  ext_map.insert('d', Extension::D);
  ext_map.insert('q', Extension::Q);
  ext_map.insert('c', Extension::C);
  ext_map.insert('b', Extension::B);
  ext_map.insert('k', Extension::K);
  ext_map.insert('h', Extension::H);
  ext_map.insert('p', Extension::P);
  ext_map.insert('v', Extension::V);
  ext_map
}

impl Isa {
  pub fn new(str: &str) -> Option<Self> {
    let ext_map = get_ext_map();
    let str = str.to_lowercase();
    let (xlen, rest) = match &str[0..4] {
      "rv32" => (XLen::Bit32, &str[4..]),
      "rv64" => (XLen::Bit64, &str[4..]),
      _ if (&str[0..2] == "rv") => (XLen::Bit64, &str[2..]),
      _ => return None,
    };
    let mut extensions = vec![];
    let ext_str = if rest == "" {
      "imafdc".to_string()
    } else if rest.chars().nth(0) == Some('g') {
      "imafd".to_string() + &rest[1..]
    } else {
      rest.to_string()
    };
    let chars = ext_str.chars().collect::<Vec<_>>();
    let mut chars = chars.iter();
    while let Some(&c) = chars.next() {
      if ALL_EXT.contains(c) {
        extensions.push(ext_map[&c])
      } else if c == 'z' {
        let ext: String = chars.by_ref().take_while(|c| **c != '_').cloned().collect();
        match &ext[..] {
          "ifencei" => {
            extensions.push(Extension::Zifencei);
          },
          "icsr" => {
            extensions.push(Extension::Zicsr);
          },
          _ => return None,
        }
      } else {
        return None
      }
    }
    Some(Self { xlen, extensions })
  }

  pub(crate) fn get_xlen(&self) -> XLen { self.xlen }

  pub(crate) fn have_ext(&self, ext: Extension) -> bool { self.extensions.contains(&ext) }
}
