use derive_deref::Deref;
use leap_sys::LEAP_TRACKING_EVENT;

use crate::{FrameHeaderRef, HandRef};

#[doc = " A snapshot, or frame of data, containing the tracking data for a single moment in time."]
#[doc = " The LEAP_FRAME struct is the container for all the tracking data."]
#[doc = " @since 3.0.0"]
#[derive(Deref, Clone, Copy)]
pub struct TrackingEventRef<'a>(pub(crate) &'a LEAP_TRACKING_EVENT);

impl<'a> TrackingEventRef<'a> {
    #[doc = " A universal frame identification header. @since 3.0.0"]
    pub fn info(&self) -> FrameHeaderRef {
        FrameHeaderRef(&self.info)
    }

    #[doc = " A pointer to the array of hands tracked in this frame."]
    #[doc = " @since 3.0.0"]
    pub fn hands(&self) -> Vec<HandRef> {
        let n_hand = self.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.pHands.offset(hand_index))
                .map(HandRef)
                .collect()
        }
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
            EventRef::Tracking(e) => {
                let hands = e.hands();
                let hand_ids: Vec<_> = hands.iter().map(|h| h.id).collect();
                Some(hand_ids)
            }
            _ => None,
        })
        .expect("No tracking event received");
    }
}
