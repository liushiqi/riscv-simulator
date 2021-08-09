use std::{
  io::{stdout, Write},
  sync::{
    atomic::{AtomicUsize, Ordering},
    mpsc::{channel, Sender},
    Arc,
  },
  thread,
};

use crate::{
  device::{Device, ReqType},
  SimulateError, SimulateResult,
};

pub struct Uart {
  tx: Sender<char>,
  count: Arc<AtomicUsize>,
}

impl Uart {
  pub fn new() -> Self {
    let (tx, rx) = channel();
    let count = Arc::new(AtomicUsize::new(0));
    let count_clone = count.clone();
    thread::spawn(move || {
      while let Ok(c) = rx.recv() {
        count_clone.fetch_sub(1, Ordering::SeqCst);
        print!("{}", c);
        stdout().flush();
      }
    });
    Self { tx, count }
  }
}

impl Device for Uart {
  fn read(&mut self, address: u64, req_type: ReqType) -> SimulateResult<ReqType> {
    if address == 0 {
      Ok(req_type.fill_with(0))
    } else if address == 4 {
      Ok(req_type.fill_with(if self.count.load(Ordering::Relaxed) >= 20 { 0b1000 } else { 0 }))
    } else {
      Err(SimulateError::LoadMisaligned(address as usize, req_type))
    }
  }

  fn write(&mut self, address: u64, data: ReqType) -> SimulateResult<()> {
    if address == 0 {
      if self
        .count
        .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |i| if i >= 20 { None } else { Some(i + 1) })
        .is_ok()
      {
        self.tx.send(data.get_inner() as u8 as char).unwrap();
      }
      Ok(())
    } else if address == 4 {
      Ok(())
    } else {
      Err(SimulateError::StoreMisaligned(address as usize, data))
    }
  }
}
