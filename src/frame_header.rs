use leap_sys::LEAP_FRAME_HEADER;

crate::leap_ref_struct!(
    #[doc = " Identifying information for a frame of tracking data. @since 3.0.0"]
    FrameHeader,
    LEAP_FRAME_HEADER
);

impl<'a> FrameHeader<'a> {
    #[doc = " A unique identifier for this frame"]
    #[doc = ""]
    #[doc = " All frames carrying this frame ID are part of the same unit of processing. This counter"]
    #[doc = " is generally an increasing counter, but may reset to another value if the user stops and"]
    #[doc = " restarts streaming."]
    #[doc = ""]
    #[doc = " For interpolated frames, this value corresponds to the identifier of the frame upper bound."]
    #[doc = " @since 3.0.0"]
    pub fn frame_id(&self) -> i64 {
        self.handle.frame_id
    }

    #[doc = " The timestamp for this image, in microseconds, referenced against LeapGetNow()."]
    #[doc = " @since 3.0.0"]
    pub fn timestamp(&self) -> i64 {
        self.handle.timestamp
    }
}
