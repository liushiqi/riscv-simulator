use std::{
  array::TryFromSliceError,
  error,
  fmt::{Debug, Display, Formatter},
  io,
  sync::PoisonError,
};

use crate::device::ReqType;

pub type SimulateResult<T> = Result<T, SimulateError>;

#[derive(Debug)]
pub enum SimulateError {
  // wrapped error
  IoError(io::Error),
  SliceError(TryFromSliceError),
  ObjectFileError(object::Error),
  LockError(String),

  // elf error
  ElfArchError(object::Architecture),

  // load error
  LoadMisaligned(usize, ReqType),
  StoreMisaligned(usize, ReqType),
}

impl From<io::Error> for SimulateError {
  fn from(error: io::Error) -> Self { SimulateError::IoError(error) }
}

impl From<TryFromSliceError> for SimulateError {
  fn from(error: TryFromSliceError) -> Self { SimulateError::SliceError(error) }
}

impl From<object::Error> for SimulateError {
  fn from(error: object::Error) -> Self { SimulateError::ObjectFileError(error) }
}

impl<T: Sized> From<PoisonError<T>> for SimulateError {
  fn from(error: PoisonError<T>) -> Self { SimulateError::LockError(error.to_string()) }
}

impl Display for SimulateError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SimulateError::IoError(error) => {
        write!(f, "IO Error: {}", error)
      },
      SimulateError::SliceError(error) => {
        write!(f, "Slice operation error: {}", error)
      },
      SimulateError::ObjectFileError(error) => {
        write!(f, "Object file load error: {}", error)
      },
      SimulateError::LockError(error) => {
        write!(f, "Get mutex lock error: {}", error)
      },
      SimulateError::ElfArchError(arch) => {
        write!(f, "Invalid object file architecture {:?}", arch)
      },
      SimulateError::LoadMisaligned(address, req) => {
        write!(f, "Misaligned {} read on address {}", req, address)
      },
      SimulateError::StoreMisaligned(address, req) => {
        write!(f, "Misaligned {} store on address {}", req, address)
      },
    }
  }
}

impl error::Error for SimulateError {}
