use leap_sys::LEAP_POINT_MAPPING_CHANGE_EVENT;

crate::leap_ref_struct!(PointMappingChangeEvent, LEAP_POINT_MAPPING_CHANGE_EVENT);

impl<'a> PointMappingChangeEvent<'a> {
    #[doc = " The ID of the frame corresponding to the source of the currently tracked points. @since 4.0.0"]
    pub fn frame_id(&self) -> i64 {
        self.handle.frame_id
    }

    #[doc = " The timestamp of the frame, in microseconds, referenced against LeapGetNow(). @since 4.0.0"]
    pub fn timestamp(&self) -> i64 {
        self.handle.timestamp
    }

    #[doc = " The number of points being tracked. @since 4.0.0"]
    pub fn n_points(&self) -> u32 {
        self.handle.nPoints
    }
}
