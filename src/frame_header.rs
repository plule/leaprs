use derive_deref::Deref;
use leap_sys::LEAP_FRAME_HEADER;

#[doc = " Identifying information for a frame of tracking data. @since 3.0.0"]
/// # Fields
/// Available via dereference: [LEAP_FRAME_HEADER].
#[derive(Deref, Clone, Copy)]
pub struct FrameHeaderRef<'a>(pub(crate) &'a LEAP_FRAME_HEADER);
