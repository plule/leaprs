use leap_sys::{LEAP_HAND, LEAP_TRACKING_EVENT};

pub trait TrakingEvent {
    fn get_hands(&self) -> Vec<&LEAP_HAND>;
}

impl TrakingEvent for LEAP_TRACKING_EVENT {
    fn get_hands(&self) -> Vec<&LEAP_HAND> {
        let n_hand = self.nHands as isize;
        unsafe {
            (0..n_hand)
                .map(|hand_index| &*self.pHands.offset(hand_index))
                .collect()
        }
    }
}
