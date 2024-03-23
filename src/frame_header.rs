use derive_deref::Deref;
use leap_sys::LEAP_FRAME_HEADER;

#[doc = " Identifying information for a frame of tracking data. @since 3.0.0"]
#[derive(Deref)]
pub struct FrameHeader<'a>(pub(crate) &'a LEAP_FRAME_HEADER);
