use derive_deref::Deref;
use leap_sys::LEAP_VERSION;

#[derive(Deref, Clone, Copy)]
pub struct Version(pub(crate) LEAP_VERSION);
