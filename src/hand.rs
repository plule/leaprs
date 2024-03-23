use derive_deref::Deref;
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

#[doc = " Describes a tracked hand. @since 3.0.0"]
#[derive(Deref)]
pub struct Hand<'a>(pub(crate) &'a LEAP_HAND);

impl<'a> Hand<'a> {
    #[doc = " Identifies the chirality of this hand. @since 3.0.0"]
    pub fn hand_type(&self) -> HandType {
        match self.type_ {
            leap_sys::_eLeapHandType_eLeapHandType_Left => HandType::Left,
            leap_sys::_eLeapHandType_eLeapHandType_Right => HandType::Right,
            _ => unreachable!(),
        }
    }

    #[doc = " Additional information associated with the palm. @since 3.0.0"]
    pub fn palm(&self) -> Palm {
        Palm(&self.palm)
    }

    #[doc = " The arm to which this hand is attached."]
    #[doc = " An arm consists of a single LEAP_BONE struct."]
    #[doc = " @since 3.0.0"]
    pub fn arm(&self) -> Bone {
        Bone(&self.arm)
    }

    #[doc = " The fingers of the hand as an array. @since 3.0.0"]
    pub fn digits(&self) -> [Digit; 5] {
        let digits;
        unsafe {
            digits = &self.__bindgen_anon_1.digits;
        }
        [
            Digit(&digits[0]),
            Digit(&digits[1]),
            Digit(&digits[2]),
            Digit(&digits[3]),
            Digit(&digits[4]),
        ]
    }

    #[doc = " The thumb. @since 3.0.0"]
    pub fn thumb(&self) -> Digit {
        unsafe { Digit(&self.__bindgen_anon_1.__bindgen_anon_1.thumb) }
    }

    #[doc = " The index finger. @since 3.0.0"]
    pub fn index(&self) -> Digit {
        unsafe { Digit(&self.__bindgen_anon_1.__bindgen_anon_1.index) }
    }

    #[doc = " The middle finger. @since 3.0.0"]
    pub fn middle(&self) -> Digit {
        unsafe { Digit(&self.__bindgen_anon_1.__bindgen_anon_1.middle) }
    }

    #[doc = " The ring finger. @since 3.0.0"]
    pub fn ring(&self) -> Digit {
        unsafe { Digit(&self.__bindgen_anon_1.__bindgen_anon_1.ring) }
    }

    #[doc = " The pinky finger. @since 3.0.0"]
    pub fn pinky(&self) -> Digit {
        unsafe { Digit(&self.__bindgen_anon_1.__bindgen_anon_1.pinky) }
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    fn get_all_hand_bones() {
        let mut connection = initialize_connection();
        connection
            .wait_for(|e| match e {
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
                            assert!(bone.width > 0.0);
                        }
                    }
                    Some(())
                }
                _ => None,
            })
            .expect("No hand in view");
    }
}
