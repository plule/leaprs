use leap_sys::*;

use crate::{Bone, Digit, Palm};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[doc = " The Hand chirality types."]
#[doc = " Used in the LEAP_HAND struct."]
#[doc = " @since 3.0.0"]
pub enum HandType {
    #[doc = " A left hand. @since 3.0.0"]
    Left,
    #[doc = " A right hand. @since 3.0.0"]
    Right,
}

crate::leap_ref_struct!(
    #[doc = " Describes a tracked hand. @since 3.0.0"]
    Hand,
    LEAP_HAND
);

impl<'a> Hand<'a> {
    #[doc = " A unique ID for a hand tracked across frames."]
    #[doc = " If tracking of a physical hand is lost, a new ID is assigned when"]
    #[doc = " tracking is reacquired."]
    #[doc = " @since 3.0.0"]
    pub fn id(&self) -> u32 {
        self.handle.id
    }

    #[doc = " Identifies the chirality of this hand. @since 3.0.0"]
    pub fn hand_type(&self) -> HandType {
        match self.handle.type_ {
            leap_sys::_eLeapHandType_eLeapHandType_Left => HandType::Left,
            leap_sys::_eLeapHandType_eLeapHandType_Right => HandType::Right,
            _ => unreachable!(),
        }
    }

    #[doc = " How confident we are with a given hand pose. Not currently used (always 1.0)."]
    #[doc = " @since 3.0.0"]
    pub fn confidence(&self) -> f32 {
        self.handle.confidence
    }

    #[doc = " The total amount of time this hand has been tracked, in microseconds."]
    #[doc = " @since 3.0.0"]
    pub fn visible_time(&self) -> u64 {
        self.handle.visible_time
    }

    #[doc = " The distance between index finger and thumb. @since 3.0.0"]
    pub fn pinch_distance(&self) -> f32 {
        self.handle.pinch_distance
    }

    #[doc = " The average angle of fingers to palm. @since 3.0.0"]
    pub fn grab_angle(&self) -> f32 {
        self.handle.grab_angle
    }

    #[doc = " The normalized estimate of the pinch pose."]
    #[doc = " Zero is not pinching; one is fully pinched."]
    #[doc = " @since 3.0.0"]
    pub fn pinch_strength(&self) -> f32 {
        self.handle.pinch_strength
    }

    #[doc = " The normalized estimate of the grab hand pose."]
    #[doc = " Zero is not grabbing; one is fully grabbing."]
    #[doc = " @since 3.0.0"]
    pub fn grab_strength(&self) -> f32 {
        self.handle.grab_strength
    }

    #[doc = " Additional information associated with the palm. @since 3.0.0"]
    pub fn palm(&self) -> Palm {
        (&self.handle.palm).into()
    }

    #[doc = " The arm to which this hand is attached."]
    #[doc = " An arm consists of a single LEAP_BONE struct."]
    #[doc = " @since 3.0.0"]
    pub fn arm(&self) -> Bone {
        (&self.handle.arm).into()
    }

    #[doc = " The fingers of the hand as an array. @since 3.0.0"]
    pub fn digits(&self) -> [Digit; 5] {
        let digits;
        unsafe {
            digits = &self.handle.__bindgen_anon_1.digits;
        }
        [
            (&digits[0]).into(),
            (&digits[1]).into(),
            (&digits[2]).into(),
            (&digits[3]).into(),
            (&digits[4]).into(),
        ]
    }

    #[doc = " The thumb. @since 3.0.0"]
    pub fn thumb(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.thumb }.into()
    }

    #[doc = " The index finger. @since 3.0.0"]
    pub fn index(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.index }.into()
    }

    #[doc = " The middle finger. @since 3.0.0"]
    pub fn middle(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.middle }.into()
    }

    #[doc = " The ring finger. @since 3.0.0"]
    pub fn ring(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.ring }.into()
    }

    #[doc = " The pinky finger. @since 3.0.0"]
    pub fn pinky(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.pinky }.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn get_all_hand_bones() {
        let mut connection = initialize_connection();
        connection.expect_event("No hand in view".to_string(), |e| match e {
            Event::Tracking(e) => {
                let hands = e.hands();
                if hands.is_empty() {
                    println!("Warning: Put hands in front of the sensor for this test.");
                }

                let digits_by_array = hands.iter().flat_map(|h| h.digits());

                let digits_by_name = hands
                    .iter()
                    .flat_map(|h| [h.thumb(), h.index(), h.middle(), h.ring(), h.pinky()]);

                for digit in digits_by_array.chain(digits_by_name) {
                    let bones_by_array = digit.bones();
                    let bones_by_name = [
                        digit.proximal(),
                        digit.intermediate(),
                        digit.proximal(),
                        digit.distal(),
                    ];

                    for bone in bones_by_array.iter().chain(bones_by_name.iter()) {
                        assert!(bone.width() > 0.0);
                    }
                }
                Some(())
            }
            _ => None,
        });
    }
}
