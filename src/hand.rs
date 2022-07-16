use leap_sys::*;

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

    pub fn digits(&self) -> &[LEAP_DIGIT; 5] {
        unsafe { &self.handle.__bindgen_anon_1.digits }
    }

    pub fn thumb(&self) -> &LEAP_DIGIT {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.thumb }
    }

    pub fn index(&self) -> &LEAP_DIGIT {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.index }
    }

    pub fn middle(&self) -> &LEAP_DIGIT {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.middle }
    }

    pub fn ring(&self) -> &LEAP_DIGIT {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.ring }
    }

    pub fn pinky(&self) -> &LEAP_DIGIT {
        unsafe { &self.handle.__bindgen_anon_1.__bindgen_anon_1.pinky }
    }
}
