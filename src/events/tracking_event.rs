use leap_sys::{LEAP_FRAME_HEADER, LEAP_HAND, LEAP_TRACKING_EVENT};

crate::leap_ref_struct!(TrackingEvent, LEAP_TRACKING_EVENT);

impl<'a> TrackingEvent<'a> {
    pub fn get_info(&self) -> &LEAP_FRAME_HEADER {
        &self.handle.info
    }

    pub fn get_tracking_frame_id(&self) -> i64 {
        self.handle.tracking_frame_id
    }

    pub fn get_hands(&self) -> Vec<&LEAP_HAND> {
        let n_hand = self.handle.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.handle.pHands.offset(hand_index))
                .collect()
        }
    }

    pub fn get_framerate(&self) -> f32 {
        self.handle.framerate
    }
}
