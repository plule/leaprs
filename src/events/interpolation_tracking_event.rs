use std::ops::Deref;

use crate::{sized_with_trailing_data::SizedWithTrailingData, FrameHeader, Hand};
use leap_sys::LEAP_TRACKING_EVENT;

pub struct InterpolationTrackingEvent(pub(crate) Box<SizedWithTrailingData<LEAP_TRACKING_EVENT>>);

impl InterpolationTrackingEvent {
    /// Allocate a LEAP_TRACKING_EVENT with more data contiguous to it.
    /// Unsafe: inner struct is uninitialized
    pub(crate) unsafe fn new_uninitialized(requested_frame_size: u64) -> Self {
        let trailing_size =
            requested_frame_size as usize - std::mem::size_of::<LEAP_TRACKING_EVENT>();
        Self(SizedWithTrailingData::allocate(
            std::mem::zeroed(),
            trailing_size,
        ))
    }

    #[doc = " A universal frame identification header. @since 3.0.0"]
    pub fn info(&self) -> FrameHeader {
        FrameHeader(&self.info)
    }

    #[doc = " A pointer to the array of hands tracked in this frame."]
    #[doc = " @since 3.0.0"]
    pub fn hands(&self) -> Vec<Hand> {
        let n_hand = self.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.pHands.offset(hand_index))
                .map(Hand)
                .collect()
        }
    }
}

impl Deref for InterpolationTrackingEvent {
    type Target = LEAP_TRACKING_EVENT;

    fn deref(&self) -> &Self::Target {
        &self.0.sized
    }
}
