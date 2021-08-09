mod addiw;
mod addw;
mod ld;
mod lwu;
mod sd;
mod slliw;
mod sllw;
mod sraiw;
mod sraw;
mod srliw;
mod srlw;
mod subw;

pub(crate) use self::{
  addiw::Addiw, addw::Addw, ld::Ld, lwu::Lwu, sd::Sd, slliw::Slliw, sllw::Sllw, sraiw::Sraiw, sraw::Sraw, srliw::Srliw,
  srlw::Srlw, subw::Subw,
};
