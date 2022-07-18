use leap_sys::LEAP_TRACKING_EVENT;

use crate::{sized_with_trailing_data::SizedWithTrailingData, FrameHeader, Hand};

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

    #[doc = " An identifier for this tracking frame. This identifier is meant to be monotonically"]
    #[doc = " increasing, but values may be skipped if the client application does not poll for messages"]
    #[doc = " fast enough. This number also generally increases at the same rate as info.frame_id, but"]
    #[doc = " if the server cannot process every image received from the device cameras, the info.frame_id"]
    #[doc = " identifier may increase faster."]
    #[doc = " @since 3.0.0"]
    pub fn tracking_frame_id(&self) -> i64 {
        self.handle.sized.tracking_frame_id
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

    #[doc = " Current tracking frame rate in hertz."]
    #[doc = ""]
    #[doc = " This frame rate is distinct from the image frame rate, which is the rate that images are"]
    #[doc = " being read from the device. Depending on host CPU limitations, the tracking frame rate"]
    #[doc = " may be substantially less than the device frame rate."]
    #[doc = ""]
    #[doc = " This number is generally equal to or less than the device frame rate, but there is one"]
    #[doc = " case where this number may be _higher_ than the device frame rate:  When the device rate"]
    #[doc = " drops. In this case, the device frame rate will fall sooner than the tracking frame rate."]
    #[doc = ""]
    #[doc = " This number is equal to zero if there are not enough frames to estimate frame rate."]
    #[doc = ""]
    #[doc = " This number cannot be negative."]
    #[doc = " @since 3.0.0"]
    pub fn framerate(&self) -> f32 {
        self.handle.sized.framerate
    }
}
