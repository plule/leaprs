use leap_sys::LEAP_FRAME_HEADER;

crate::leap_ref_struct!(FrameHeader, LEAP_FRAME_HEADER);

impl<'a> FrameHeader<'a> {
    pub fn frame_id(&self) -> i64 {
        self.handle.frame_id
    }

    pub fn timestamp(&self) -> i64 {
        self.handle.timestamp
    }
}
