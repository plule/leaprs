use derive_deref::Deref;
use leap_sys::LEAP_DROPPED_FRAME_EVENT;

use crate::DroppedFrameType;

#[derive(Deref)]
pub struct DroppedFrameEvent<'a>(pub(crate) &'a LEAP_DROPPED_FRAME_EVENT);

impl<'a> DroppedFrameEvent<'a> {
    pub fn dropped_frame_type(&self) -> DroppedFrameType {
        self.type_.into()
    }

    // TODO: device
}
