use leap_sys::{LEAP_FRAME_HEADER, LEAP_TRACKING_EVENT};

use crate::Hand;

crate::leap_ref_struct!(TrackingEvent, LEAP_TRACKING_EVENT);

impl<'a> TrackingEvent<'a> {
    pub fn info(&self) -> &LEAP_FRAME_HEADER {
        &self.handle.info
    }

    pub fn tracking_frame_id(&self) -> i64 {
        self.handle.tracking_frame_id
    }

    pub fn hands(&self) -> Vec<Hand> {
        let n_hand = self.handle.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.handle.pHands.offset(hand_index))
                .map(|h| h.into())
                .collect()
        }
    }

    pub fn framerate(&self) -> f32 {
        self.handle.framerate
    }
}
