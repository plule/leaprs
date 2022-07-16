use leap_sys::*;

use crate::Digit;

pub enum HandType {
    Left,
    Right,
}

crate::leap_ref_struct!(Hand, LEAP_HAND);

impl<'a> Hand<'a> {
    pub fn id(&self) -> u32 {
        self.handle.id
    }

    pub fn hand_type(&self) -> HandType {
        match self.handle.type_ {
            leap_sys::_eLeapHandType_eLeapHandType_Left => HandType::Left,
            leap_sys::_eLeapHandType_eLeapHandType_Right => HandType::Right,
            _ => unreachable!(),
        }
    }

    pub fn confidence(&self) -> f32 {
        self.handle.confidence
    }

    pub fn visible_time(&self) -> u64 {
        self.handle.visible_time
    }

    pub fn pinch_distance(&self) -> f32 {
        self.handle.pinch_distance
    }

    pub fn grab_angle(&self) -> f32 {
        self.handle.grab_angle
    }

    pub fn pinch_strength(&self) -> f32 {
        self.handle.pinch_strength
    }

    pub fn grab_strength(&self) -> f32 {
        self.handle.grab_strength
    }

    pub fn palm(&self) -> &LEAP_PALM {
        &self.handle.palm
    }

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

    pub fn thumb(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.thumb }.into()
    }

    pub fn index(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.index }.into()
    }

    pub fn middle(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.middle }.into()
    }

    pub fn ring(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.ring }.into()
    }

    pub fn pinky(&self) -> Digit {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.pinky }.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::tests::*;
    use crate::*;

    #[test]
    #[ignore] // needs to put hand in front of leap
    fn get_all_hand_bones() {
        let mut connection = initialize_connection();
        connection.expect_event("No hand in view".to_string(), |e| match e {
            Event::Tracking(e) => {
                let hands = e.hands();
                if !hands.is_empty() {
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
                } else {
                    None
                }
            }
            _ => None,
        });
    }
}
