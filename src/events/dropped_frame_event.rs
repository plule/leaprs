use leap_sys::LEAP_DROPPED_FRAME_EVENT;

use crate::DroppedFrameType;

crate::leap_ref_struct!(DroppedFrameEvent, LEAP_DROPPED_FRAME_EVENT);

impl<'a> DroppedFrameEvent<'a> {
    pub fn dropped_frame_type(&self) -> DroppedFrameType {
        self.handle.type_.into()
    }

    // TODO: device
}
