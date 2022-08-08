use leap_sys::LEAP_TRACKING_EVENT;

use crate::{FrameHeader, Hand};

crate::leap_ref_struct!(
    #[doc = " A snapshot, or frame of data, containing the tracking data for a single moment in time."]
    #[doc = " The LEAP_FRAME struct is the container for all the tracking data."]
    #[doc = " @since 3.0.0"]
    TrackingEvent,
    LEAP_TRACKING_EVENT
);

impl<'a> TrackingEvent<'a> {
    #[doc = " A universal frame identification header. @since 3.0.0"]
    pub fn info(&self) -> FrameHeader {
        (&self.handle.info).into()
    }

    #[doc = " An identifier for this tracking frame. This identifier is meant to be monotonically"]
    #[doc = " increasing, but values may be skipped if the client application does not poll for messages"]
    #[doc = " fast enough. This number also generally increases at the same rate as info.frame_id, but"]
    #[doc = " if the server cannot process every image received from the device cameras, the info.frame_id"]
    #[doc = " identifier may increase faster."]
    #[doc = " @since 3.0.0"]
    pub fn tracking_frame_id(&self) -> i64 {
        self.handle.tracking_frame_id
    }

    #[doc = " A pointer to the array of hands tracked in this frame."]
    #[doc = " @since 3.0.0"]
    pub fn hands(&self) -> Vec<Hand> {
        let n_hand = self.handle.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.handle.pHands.offset(hand_index))
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
        self.handle.framerate
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn read_hands() {
        let mut conn = initialize_connection();
        conn.wait_for(|e| match e {
            Event::Tracking(e) => {
                let hands = e.hands();
                let hand_ids: Vec<_> = hands.iter().map(|h| h.id()).collect();
                Some(hand_ids)
            }
            _ => None,
        })
        .expect("No tracking event received");
    }
}
