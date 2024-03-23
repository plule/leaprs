use std::ops::Deref;

use crate::{sized_with_trailing_data::SizedWithTrailingData, FrameHeader, Hand};
use leap_sys::LEAP_TRACKING_EVENT;

pub struct InterpolationTrackingEvent {
    /// Store a boxed dynamic sized event
    /// The size is only known at runtime
    pub(crate) handle: Box<SizedWithTrailingData<LEAP_TRACKING_EVENT>>,
}

impl InterpolationTrackingEvent {
    /// Allocate a LEAP_TRACKING_EVENT with more data contiguous to it.
    /// Unsafe: inner struct is uninitialized
    pub(crate) unsafe fn new_uninitialized(requested_frame_size: u64) -> Self {
        let trailing_size =
            requested_frame_size as usize - std::mem::size_of::<LEAP_TRACKING_EVENT>();
        Self {
            handle: SizedWithTrailingData::allocate(std::mem::zeroed(), trailing_size),
        }
    }

    #[doc = " A universal frame identification header. @since 3.0.0"]
    pub fn info(&self) -> FrameHeader {
        (&self.handle.sized.info).into()
    }

    #[doc = " A pointer to the array of hands tracked in this frame."]
    #[doc = " @since 3.0.0"]
    pub fn hands(&self) -> Vec<Hand> {
        let n_hand = self.handle.sized.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.handle.sized.pHands.offset(hand_index))
                .map(|h| h.into())
                .collect()
        }
    }
}

impl Deref for InterpolationTrackingEvent {
    type Target = LEAP_TRACKING_EVENT;

    fn deref(&self) -> &Self::Target {
        &self.handle.sized
    }
}
